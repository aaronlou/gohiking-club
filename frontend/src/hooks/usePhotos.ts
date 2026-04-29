import { useQuery, useMutation, useQueryClient } from "@tanstack/react-query";
import * as api from "@/api/client";
import type { PhotoFilter } from "@/types";

export function usePhotos(filter: PhotoFilter = {}) {
  return useQuery({
    queryKey: ["photos", filter],
    queryFn: () => api.listPhotos(filter),
    staleTime: 10_000,
  });
}

export function usePhoto(id: string) {
  return useQuery({
    queryKey: ["photos", id],
    queryFn: () => api.getPhoto(id),
    enabled: !!id,
  });
}

export function useDeletePhoto() {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: (id: string) => api.deletePhoto(id),
    onSuccess: () => {
      qc.invalidateQueries({ queryKey: ["photos"] });
    },
  });
}
