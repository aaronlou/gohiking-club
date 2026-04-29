import { useQuery, useMutation, useQueryClient } from "@tanstack/react-query";
import * as api from "@/api/client";
import type { CreateEventRequest } from "@/types";

export function useEvents(filter?: { status?: string; limit?: number }) {
  return useQuery({
    queryKey: ["events", filter],
    queryFn: () => api.listEvents(filter),
    staleTime: 10_000,
  });
}

export function useEvent(id: string) {
  return useQuery({
    queryKey: ["events", id],
    queryFn: () => api.getEvent(id),
    enabled: !!id,
  });
}

export function useEventPhotos(id: string) {
  return useQuery({
    queryKey: ["events", id, "photos"],
    queryFn: () => api.getEventPhotos(id),
    enabled: !!id,
  });
}

export function useCreateEvent() {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: (req: CreateEventRequest) => api.createEvent(req),
    onSuccess: () => {
      qc.invalidateQueries({ queryKey: ["events"] });
    },
  });
}

export function useJoinEvent() {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: (id: string) => api.joinEvent(id),
    onSuccess: (_data, id) => {
      qc.invalidateQueries({ queryKey: ["events", id] });
    },
  });
}
