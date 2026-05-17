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

// ── Invitations ──

export function useTeamInvitations(teamId: string) {
  return useQuery({
    queryKey: ["teams", teamId, "invitations"],
    queryFn: () => api.listTeamInvitations(teamId),
    enabled: !!teamId,
  });
}

export function useCreateTeamInvitation() {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: ({ teamId, maxUses, expiresAt }: { teamId: string; maxUses?: number; expiresAt?: string }) =>
      api.createTeamInvitation(teamId, maxUses, expiresAt),
    onSuccess: (_data, vars) => {
      qc.invalidateQueries({ queryKey: ["teams", vars.teamId, "invitations"] });
    },
  });
}

export function useInvitationByCode(code: string) {
  return useQuery({
    queryKey: ["invitations", code],
    queryFn: () => api.getInvitationByCode(code),
    enabled: !!code,
  });
}

export function useApplyJoinTeam() {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: ({ code, message }: { code: string; message?: string }) => api.applyJoinTeam(code, message),
    onSuccess: (_data, vars) => {
      qc.invalidateQueries({ queryKey: ["invitations", vars.code] });
    },
  });
}

// ── Join Requests ──

export function useJoinRequests(teamId: string) {
  return useQuery({
    queryKey: ["teams", teamId, "join-requests"],
    queryFn: () => api.listJoinRequests(teamId),
    enabled: !!teamId,
  });
}

export function useApproveJoinRequest() {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: ({ teamId, requestId }: { teamId: string; requestId: string }) =>
      api.approveJoinRequest(teamId, requestId),
    onSuccess: (_data, vars) => {
      qc.invalidateQueries({ queryKey: ["teams", vars.teamId, "join-requests"] });
      qc.invalidateQueries({ queryKey: ["teams", vars.teamId, "members"] });
    },
  });
}

export function useRejectJoinRequest() {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: ({ teamId, requestId }: { teamId: string; requestId: string }) =>
      api.rejectJoinRequest(teamId, requestId),
    onSuccess: (_data, vars) => {
      qc.invalidateQueries({ queryKey: ["teams", vars.teamId, "join-requests"] });
    },
  });
}
