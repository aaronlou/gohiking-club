-- ═══════════════════════════════════════════════════════════════
-- 005: Team Invitations, Join Requests, Enhanced Events
-- ═══════════════════════════════════════════════════════════════

-- ── Enhanced Events ──
ALTER TABLE events
    ADD COLUMN IF NOT EXISTS distance_km DECIMAL(10,2),
    ADD COLUMN IF NOT EXISTS elevation_gain_m INTEGER,
    ADD COLUMN IF NOT EXISTS disclaimer TEXT;

-- ── Team Default Disclaimer Template ──
ALTER TABLE teams
    ADD COLUMN IF NOT EXISTS default_disclaimer TEXT;

-- ── Team Invitations ──
CREATE TABLE IF NOT EXISTS team_invitations (
    id          UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    team_id     UUID NOT NULL REFERENCES teams(id) ON DELETE CASCADE,
    code        VARCHAR(64) NOT NULL UNIQUE,
    created_by  UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    max_uses    INTEGER DEFAULT 1,
    used_count  INTEGER NOT NULL DEFAULT 0,
    expires_at  TIMESTAMPTZ,
    status      VARCHAR(20) NOT NULL DEFAULT 'active',
    created_at  TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_team_invitations_code ON team_invitations(code);
CREATE INDEX IF NOT EXISTS idx_team_invitations_team ON team_invitations(team_id);

-- ── Team Join Requests ──
CREATE TABLE IF NOT EXISTS team_join_requests (
    id              UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    team_id         UUID NOT NULL REFERENCES teams(id) ON DELETE CASCADE,
    user_id         UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    invitation_code VARCHAR(64) REFERENCES team_invitations(code) ON DELETE SET NULL,
    message         TEXT,
    status          VARCHAR(20) NOT NULL DEFAULT 'pending',
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(team_id, user_id)
);

CREATE INDEX IF NOT EXISTS idx_team_join_requests_team ON team_join_requests(team_id);
CREATE INDEX IF NOT EXISTS idx_team_join_requests_user ON team_join_requests(user_id);
CREATE INDEX IF NOT EXISTS idx_team_join_requests_status ON team_join_requests(status);
