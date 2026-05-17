import axios from "axios";
import type {
  Photo, PhotoFilter, User, Event, CreateEventRequest,
  AuthResponse, RegisterRequest, LoginRequest,
  Team, CreateTeamRequest, TeamMember, EventReview,
  TeamInvitation, TeamJoinRequest,
} from "@/types";

const api = axios.create({
  baseURL: "/api",
  headers: {
    "Content-Type": "application/json",
  },
});

// ── Auth interceptor ──

api.interceptors.request.use((config) => {
  const token = localStorage.getItem("auth-token");
  if (token) {
    config.headers.Authorization = `Bearer ${token}`;
  }
  return config;
});

// ── Photos ──

export async function uploadPhoto(
  file: File,
  title?: string,
  description?: string,
  event_id?: string,
  onProgress?: (progress: number) => void,
): Promise<Photo> {
  const form = new FormData();
  form.append("photo", file);
  if (title) form.append("title", title);
  if (description) form.append("description", description);
  if (event_id) form.append("event_id", event_id);

  const { data } = await api.post<Photo>("/photos", form, {
    onUploadProgress(e) {
      if (e.total && onProgress) {
        onProgress(Math.round((e.loaded * 100) / e.total));
      }
    },
  });
  return data;
}

export async function listPhotos(filter: PhotoFilter = {}): Promise<Photo[]> {
  const { data } = await api.get<Photo[]>("/photos", { params: filter });
  return data;
}

export async function getPhoto(id: string): Promise<Photo> {
  const { data } = await api.get<Photo>(`/photos/${id}`);
  return data;
}

export async function deletePhoto(id: string): Promise<void> {
  await api.delete(`/photos/${id}`);
}

// ── Events ──

export async function createEvent(req: CreateEventRequest): Promise<Event> {
  const { data } = await api.post<Event>("/events", req);
  return data;
}



export async function listEvents(filter?: {
  status?: string;
  limit?: number;
}): Promise<Event[]> {
  const { data } = await api.get<Event[]>("/events", { params: filter });
  return data;
}

export async function getEvent(id: string): Promise<Event> {
  const { data } = await api.get<Event>(`/events/${id}`);
  return data;
}

export async function joinEvent(id: string): Promise<void> {
  await api.post(`/events/${id}/join`);
}

export async function getEventPhotos(id: string): Promise<Photo[]> {
  const { data } = await api.get<Photo[]>(`/events/${id}/photos`);
  return data;
}

// ── Teams ──

export async function createTeam(req: CreateTeamRequest): Promise<Team> {
  const { data } = await api.post<Team>("/teams", req);
  return data;
}

export async function listTeams(filter?: {
  status?: string;
  limit?: number;
}): Promise<Team[]> {
  const { data } = await api.get<Team[]>("/teams", { params: filter });
  return data;
}

export async function getTeam(id: string): Promise<Team> {
  const { data } = await api.get<Team>(`/teams/${id}`);
  return data;
}

export async function joinTeam(id: string): Promise<void> {
  await api.post(`/teams/${id}/join`);
}

export async function leaveTeam(id: string): Promise<void> {
  await api.post(`/teams/${id}/leave`);
}

export async function getTeamMembers(id: string): Promise<TeamMember[]> {
  const { data } = await api.get<TeamMember[]>(`/teams/${id}/members`);
  return data;
}

export async function getTeamEvents(id: string): Promise<Event[]> {
  const { data } = await api.get<Event[]>(`/teams/${id}/events`);
  return data;
}

export async function updateTeam(id: string, updates: Partial<{ name: string; description: string; logo_url: string; cover_url: string; default_disclaimer: string }>): Promise<Team> {
  const { data } = await api.put<Team>(`/teams/${id}`, updates);
  return data;
}

// ── Event Reviews ──

export async function createEventReview(
  eventId: string,
  content: string,
): Promise<EventReview> {
  const { data } = await api.post<EventReview>(`/events/${eventId}/reviews`, {
    content,
  });
  return data;
}

export async function listEventReviews(eventId: string): Promise<EventReview[]> {
  const { data } = await api.get<EventReview[]>(`/events/${eventId}/reviews`);
  return data;
}

export async function deleteEventReview(eventId: string, reviewId: string): Promise<void> {
  await api.post(`/events/${eventId}/reviews/${reviewId}`);
}

// ── Team Invitations ──

export async function createTeamInvitation(teamId: string, maxUses?: number, expiresAt?: string): Promise<TeamInvitation> {
  const { data } = await api.post<TeamInvitation>(`/teams/${teamId}/invitations`, { max_uses: maxUses, expires_at: expiresAt });
  return data;
}

export async function listTeamInvitations(teamId: string): Promise<TeamInvitation[]> {
  const { data } = await api.get<TeamInvitation[]>(`/teams/${teamId}/invitations`);
  return data;
}

export async function getInvitationByCode(code: string): Promise<{ invitation: TeamInvitation; team: Team }> {
  const { data } = await api.get<{ invitation: TeamInvitation; team: Team }>(`/teams/invitations/${code}`);
  return data;
}

export async function applyJoinTeam(code: string, message?: string): Promise<TeamJoinRequest> {
  const { data } = await api.post<TeamJoinRequest>(`/teams/invitations/${code}/apply`, { message });
  return data;
}

export async function listJoinRequests(teamId: string): Promise<TeamJoinRequest[]> {
  const { data } = await api.get<TeamJoinRequest[]>(`/teams/${teamId}/join-requests`);
  return data;
}

export async function approveJoinRequest(teamId: string, requestId: string): Promise<void> {
  await api.post(`/teams/${teamId}/join-requests/approve`, { request_id: requestId });
}

export async function rejectJoinRequest(teamId: string, requestId: string): Promise<void> {
  await api.post(`/teams/${teamId}/join-requests/reject`, { request_id: requestId });
}

// ── Auth ──

export async function registerUser(req: RegisterRequest): Promise<AuthResponse> {
  const { data } = await api.post<AuthResponse>("/auth/register", req);
  return data;
}

export async function loginUser(req: LoginRequest): Promise<AuthResponse> {
  const { data } = await api.post<AuthResponse>("/auth/login", req);
  return data;
}

export async function updateMemberRole(teamId: string, userId: string, role: "admin" | "member"): Promise<void> {
  await api.put(`/teams/${teamId}/members/${userId}/role`, { role });
}

export async function getMe(): Promise<User> {
  const { data } = await api.get<User>("/auth/me");
  return data;
}
