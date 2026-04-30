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
  status: string;
  member_count: number;
  photo_count: number;
  created_at: string;
}

export interface CreateEventRequest {
  title: string;
  description?: string;
  location?: string;
  date?: string;
}

// ── Auth ──

export interface RegisterRequest {
  username: string;
  email: string;
  password: string;
}

export interface LoginRequest {
  email: string;
  password: string;
}

export interface AuthResponse {
  token: string;
  user: User;
}
