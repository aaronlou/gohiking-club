import axios from "axios";
import type { Photo, PhotoFilter, User, Event, CreateEventRequest } from "@/types";

const api = axios.create({
  baseURL: "/api",
  headers: {
    "Content-Type": "application/json",
  },
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

// ── Users ──

export async function registerUser(
  username: string,
  email: string,
): Promise<User> {
  const { data } = await api.post<User>("/auth/register", { username, email });
  return data;
}

export async function getUser(id: string): Promise<User> {
  const { data } = await api.get<User>(`/users/${id}`);
  return data;
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
