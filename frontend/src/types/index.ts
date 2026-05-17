export interface ScoreDimensions {
  composition: number;
  lighting: number;
  clarity: number;
  subject_interest: number;
}

export interface ScoreResult {
  overall: number;
  dimensions: ScoreDimensions;
  raw_feedback: string;
}

export type PhotoStatus = "pending" | "approved" | "rejected";

export interface Photo {
  id: string;
  user_id: string;
  url: string;
  thumbnail_url: string | null;
  title: string | null;
  description: string | null;
  ai_score: number;
  ai_feedback: ScoreResult | null;
  status: PhotoStatus;
  event_id: string | null;
  created_at: string;
}

export interface User {
  id: string;
  username: string;
  email: string;
  avatar_url: string | null;
  bio: string | null;
  photo_count: number;
  created_at: string;
}

export interface PhotoFilter {
  status?: PhotoStatus;
  min_score?: number;
  user_id?: string;
  event_id?: string;
  limit?: number;
  offset?: number;
}

export interface Event {
  id: string;
  title: string;
  description: string | null;
  location: string | null;
  date: string | null;
  cover_url: string | null;
  created_by: string;
  team_id: string | null;
  status: string;
  distance_km: number | null;
  elevation_gain_m: number | null;
  disclaimer: string | null;
  member_count: number;
  photo_count: number;
  review_count: number;
  is_team_member: boolean;
  created_at: string;
}

export interface CreateEventRequest {
  title: string;
  description?: string;
  location?: string;
  date?: string;
  team_id?: string;
  distance_km?: number;
  elevation_gain_m?: number;
  disclaimer?: string;
}

// ── Team ──

export interface Team {
  id: string;
  name: string;
  slug: string;
  description: string | null;
  logo_url: string | null;
  cover_url: string | null;
  created_by: string;
  status: string;
  member_count: number;
  event_count: number;
  default_disclaimer: string | null;
  created_at: string;
}

export type TeamMemberRole = "admin" | "member";

export interface TeamMember {
  user_id: string;
  username: string;
  avatar_url: string | null;
  role: TeamMemberRole;
  joined_at: string;
}

export interface CreateTeamRequest {
  name: string;
  slug: string;
  description?: string;
}

// ── Event Review ──

export interface EventReview {
  id: string;
  event_id: string;
  user_id: string;
  username: string;
  avatar_url: string | null;
  content: string;
  rating: number | null;
  created_at: string;
}

// ── Team Invitation ──

export interface TeamInvitation {
  id: string;
  team_id: string;
  code: string;
  max_uses: number | null;
  used_count: number;
  expires_at: string | null;
  status: string;
  created_at: string;
}

export interface TeamJoinRequest {
  id: string;
  team_id: string;
  user_id: string;
  username: string;
  avatar_url: string | null;
  invitation_code: string | null;
  message: string | null;
  status: string;
  created_at: string;
}

// ── Auth ──

export interface RegisterRequest {
  username: string;
  password: string;
}

export interface LoginRequest {
  username: string;
  password: string;
}

export interface AuthResponse {
  token: string;
  user: User;
}
