import { Trash2 } from "lucide-react";
import type { Photo } from "@/types";
import { ScoreBadge, StatusBadge } from "./ScoreBadge";
import { PhotoCaptionOverlay } from "./PhotoCaptionOverlay";
import { useState } from "react";

interface PhotoCardProps {
  photo: Photo;
  onDelete?: (id: string) => void;
}

export function PhotoCard({ photo, onDelete }: PhotoCardProps) {
  const [loaded, setLoaded] = useState(false);
  const [hovered, setHovered] = useState(false);

  return (
    <div
      className="group overflow-hidden rounded-2xl border border-clay-200 bg-white shadow-sm transition-all duration-300 hover:shadow-lg hover:-translate-y-0.5"
      onMouseEnter={() => setHovered(true)}
      onMouseLeave={() => setHovered(false)}
    >
      <div className="relative aspect-[4/3] overflow-hidden bg-clay-100">
        {!loaded && (
          <div className="absolute inset-0 animate-pulse bg-clay-200" />
        )}
        <img
          src={photo.thumbnail_url ?? photo.url}
          alt={photo.title ?? "徒步照片"}
          className={`h-full w-full object-cover transition-all duration-500 group-hover:scale-105 ${
            loaded ? "opacity-100" : "opacity-0"
          }`}
          onLoad={() => setLoaded(true)}
          loading="lazy"
        />

        {/* Canvas text overlay */}
        <PhotoCaptionOverlay photo={photo} visible={hovered} />

        {/* Score badge */}
        <div className="absolute right-2 top-2">
          <span className="inline-flex items-center gap-1 rounded-full bg-white/90 backdrop-blur-sm px-2.5 py-1 shadow-sm">
            <ScoreBadge score={photo.ai_score} />
          </span>
        </div>

        {/* Status badge */}
        <div className="absolute left-2 top-2">
          <StatusBadge status={photo.status} />
        </div>

        {onDelete && (
          <button
            onClick={(e) => {
              e.preventDefault();
              onDelete(photo.id);
            }}
            className="absolute left-2 bottom-2 rounded-full bg-white/80 p-1.5 text-clay-600 opacity-0 transition-all hover:bg-white hover:text-red-600 group-hover:opacity-100 backdrop-blur-sm"
          >
            <Trash2 className="h-4 w-4" />
          </button>
        )}
      </div>

      <div className="p-3 sm:p-4">
        <h3 className="truncate text-sm font-medium text-clay-900">
          {photo.title ?? "未命名"}
        </h3>
        {photo.description && (
          <p className="mt-1 line-clamp-2 text-xs text-clay-500">
            {photo.description}
          </p>
        )}
        <p className="mt-2 text-xs text-clay-400">
          {new Date(photo.created_at).toLocaleDateString("zh-CN")}
        </p>
      </div>
    </div>
  );
}
