import { useState, useCallback } from "react";
import { useQueryClient } from "@tanstack/react-query";
import * as api from "@/api/client";
import type { Photo } from "@/types";

interface UploadState {
  status: "idle" | "uploading" | "scoring" | "success" | "error";
  progress: number;
  photo: Photo | null;
  error: string | null;
}

export function useUpload() {
  const [state, setState] = useState<UploadState>({
    status: "idle",
    progress: 0,
    photo: null,
    error: null,
  });
  const qc = useQueryClient();

  const upload = useCallback(
    async (
      file: File,
      title?: string,
      description?: string,
      event_id?: string,
    ) => {
      setState({
        status: "uploading",
        progress: 0,
        photo: null,
        error: null,
      });

      try {
        const photo = await api.uploadPhoto(
          file,
          title,
          description,
          event_id,
          (p) => {
            setState((s) => ({ ...s, progress: p }));
          },
        );

        setState({
          status: photo.status === "approved" ? "success" : "scoring",
          progress: 100,
          photo,
          error: null,
        });

        qc.invalidateQueries({ queryKey: ["photos"] });
        qc.invalidateQueries({ queryKey: ["events"] });
      } catch (e: unknown) {
        const msg = e instanceof Error ? e.message : "Upload failed";
        setState({ status: "error", progress: 0, photo: null, error: msg });
      }
    },
    [qc],
  );

  const reset = useCallback(() => {
    setState({
      status: "idle",
      progress: 0,
      photo: null,
      error: null,
    });
  }, []);

  return { state, upload, reset };
}
