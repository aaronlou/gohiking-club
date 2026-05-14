import axios from "axios";
import type {
  Photo, PhotoFilter, User, Event, CreateEventRequest,
  AuthResponse, RegisterRequest, LoginRequest,
  Team, CreateTeamRequest, TeamMember, EventReview,
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
    headers: { "Content-Type": "multipart/form-data" },
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

// ── Event Reviews ──

export async function createEventReview(
  eventId: string,
  content: string,
  rating?: number,
): Promise<EventReview> {
  const { data } = await api.post<EventReview>(`/events/${eventId}/reviews`, {
    content,
    rating,
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

// ── Auth ──

export async function registerUser(req: RegisterRequest): Promise<AuthResponse> {
  const { data } = await api.post<AuthResponse>("/auth/register", req);
  return data;
}

export async function loginUser(req: LoginRequest): Promise<AuthResponse> {
  const { data } = await api.post<AuthResponse>("/auth/login", req);
  return data;
}

export async function getMe(): Promise<User> {
  const { data } = await api.get<User>("/auth/me");
  return data;
}
