use serde::{Deserialize, Serialize};

/// Parsed YAML frontmatter from a SKILL.md file.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillFrontmatter {
    pub name: String,
    pub description: String,
    pub version: String,
    #[serde(default)]
    pub triggers: Vec<String>,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub author: String,
}

/// Parse a SKILL.md file into its frontmatter and body.
///
/// The file starts with `---` followed by YAML frontmatter,
/// then `---` and the markdown body.
pub fn parse_skill_md(content: &str) -> anyhow::Result<(SkillFrontmatter, String)> {
    let content = content.trim();

    // Must start with ---
    if !content.starts_with("---") {
        anyhow::bail!("SKILL.md must start with YAML frontmatter (---)");
    }

    // Find the closing ---
    let after_first = &content[3..];
    let end = after_first
        .find("\n---")
        .or_else(|| after_first.find("\r\n---"))
        .ok_or_else(|| anyhow::anyhow!("SKILL.md has unclosed YAML frontmatter"))?;

    let yaml_str = after_first[..end].trim();
    let frontmatter: SkillFrontmatter = serde_yaml::from_str(yaml_str)?;

    // Body starts after the closing ---
    let body_start = end + 4; // "\n---" is 4 chars
    let body = after_first[body_start..].trim().to_string();

    Ok((frontmatter, body))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_skill_md() {
        let content = r#"---
name: test-skill
description: A test skill
version: 1.0.0
triggers:
  - search
  - find
---

# Test Skill

This is the skill body."#;

        let (fm, body) = parse_skill_md(content).unwrap();
        assert_eq!(fm.name, "test-skill");
        assert_eq!(fm.description, "A test skill");
        assert_eq!(fm.version, "1.0.0");
        assert_eq!(fm.triggers, vec!["search", "find"]);
        assert!(body.contains("# Test Skill"));
    }

    #[test]
    fn test_parse_no_triggers() {
        let content = r#"---
name: minimal
description: Minimal skill
version: 0.1.0
---

Minimal body."#;

        let (fm, body) = parse_skill_md(content).unwrap();
        assert_eq!(fm.name, "minimal");
        assert!(fm.triggers.is_empty());
        assert_eq!(body, "Minimal body.");
    }
}
