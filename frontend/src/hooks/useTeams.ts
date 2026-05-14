import { useQuery, useMutation, useQueryClient } from "@tanstack/react-query";
import * as api from "@/api/client";
import type { CreateTeamRequest } from "@/types";

export function useTeams(filter?: { status?: string; limit?: number }) {
  return useQuery({
    queryKey: ["teams", filter],
    queryFn: () => api.listTeams(filter),
    staleTime: 10_000,
  });
}

export function useTeam(id: string) {
  return useQuery({
    queryKey: ["teams", id],
    queryFn: () => api.getTeam(id),
    enabled: !!id,
  });
}

export function useTeamMembers(id: string) {
  return useQuery({
    queryKey: ["teams", id, "members"],
    queryFn: () => api.getTeamMembers(id),
    enabled: !!id,
  });
}

export function useTeamEvents(id: string) {
  return useQuery({
    queryKey: ["teams", id, "events"],
    queryFn: () => api.getTeamEvents(id),
    enabled: !!id,
  });
}

export function useCreateTeam() {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: (req: CreateTeamRequest) => api.createTeam(req),
    onSuccess: () => {
      qc.invalidateQueries({ queryKey: ["teams"] });
    },
  });
}

export function useJoinTeam() {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: (id: string) => api.joinTeam(id),
    onSuccess: (_data, id) => {
      qc.invalidateQueries({ queryKey: ["teams", id] });
      qc.invalidateQueries({ queryKey: ["teams", id, "members"] });
    },
  });
}

export function useLeaveTeam() {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: (id: string) => api.leaveTeam(id),
    onSuccess: (_data, id) => {
      qc.invalidateQueries({ queryKey: ["teams", id] });
      qc.invalidateQueries({ queryKey: ["teams", id, "members"] });
    },
  });
}
