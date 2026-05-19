pub mod parser;

use std::path::PathBuf;

use anyhow::Context;

use parser::parse_skill_md;
use crate::agent::{Skill, SkillSource};
use crate::config::SkillsConfig;

/// Loads skills from three tiers: workspace, global, and bundled.
pub struct SkillLoader {
    skills: Vec<Skill>,
}

impl SkillLoader {
    pub fn new(config: &SkillsConfig) -> anyhow::Result<Self> {
        let mut skills: Vec<Skill> = Vec::new();

        if config.enabled {
            // Tier 3 (lowest priority): Bundled skills
            // These would be embedded via include_str!() in production
            // For now, they're empty.

            // Tier 2: Global skills (~/.gohiking/skills/)
            if let Some(home) = dirs_next_home() {
                let global_dir = home.join(".gohiking").join("skills");
                if global_dir.exists() {
                    let loaded = Self::load_from_dir(&global_dir, SkillSource::Global)?;
                    tracing::info!("Loaded {} global skills from {:?}", loaded.len(), global_dir);
                    skills.extend(loaded);
                }
            }

            // Tier 1 (highest priority): Workspace skills (./skills/)
            let workspace_dir = PathBuf::from(&config.workspace_dir);
            if workspace_dir.exists() {
                let loaded = Self::load_from_dir(&workspace_dir, SkillSource::Workspace)?;
                tracing::info!("Loaded {} workspace skills from {:?}", loaded.len(), workspace_dir);

                // Workspace skills override global/bundled skills with the same name
                for ws_skill in loaded {
                    skills.retain(|s| s.name != ws_skill.name);
                    skills.push(ws_skill);
                }
            }
        }

        Ok(Self { skills })
    }

    /// Load all SKILL.md files from a directory.
    /// Expects: <dir>/<skill-name>/SKILL.md
    fn load_from_dir(dir: &PathBuf, source: SkillSource) -> anyhow::Result<Vec<Skill>> {
        let mut skills = Vec::new();

        let entries = std::fs::read_dir(dir)
            .with_context(|| format!("Failed to read skills directory: {:?}", dir))?;

        for entry in entries {
            let entry = entry?;
            let skill_dir = entry.path();
            if !skill_dir.is_dir() {
                continue;
            }

            let skill_md = skill_dir.join("SKILL.md");
            if !skill_md.exists() {
                continue;
            }

            let content = std::fs::read_to_string(&skill_md)
                .with_context(|| format!("Failed to read: {:?}", skill_md))?;

            match parse_skill_md(&content) {
                Ok((frontmatter, body)) => {
                    skills.push(Skill {
                        name: frontmatter.name,
                        description: frontmatter.description,
                        version: frontmatter.version,
                        triggers: frontmatter.triggers,
                        body,
                        source: source.clone(),
                    });
                }
                Err(e) => {
                    tracing::warn!("Failed to parse skill at {:?}: {}", skill_md, e);
                }
            }
        }

        Ok(skills)
    }

    /// List all loaded skills.
    pub fn list(&self) -> &[Skill] {
        &self.skills
    }

    /// Find skills that match the user's message based on trigger keywords.
    pub fn find_matching(&self, user_message: &str) -> Vec<&Skill> {
        let lower = user_message.to_lowercase();
        self.skills
            .iter()
            .filter(|s| {
                s.triggers
                    .iter()
                    .any(|t| lower.contains(&t.to_lowercase()))
            })
            .collect()
    }
}

fn dirs_next_home() -> Option<PathBuf> {
    std::env::var("HOME").ok().map(PathBuf::from)
}
