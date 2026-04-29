import { Trash2 } from "lucide-react";
import type { Photo } from "@/types";
import { ScoreBadge, StatusBadge } from "./ScoreBadge";
import { useState } from "react";

interface PhotoCardProps {
  photo: Photo;
  onDelete?: (id: string) => void;
}

export function PhotoCard({ photo, onDelete }: PhotoCardProps) {
  const [loaded, setLoaded] = useState(false);

  return (
    <div className="card group overflow-hidden transition-shadow hover:shadow-md">
      <div className="relative aspect-[4/3] overflow-hidden bg-gray-100">
        {!loaded && (
          <div className="absolute inset-0 animate-pulse bg-gray-200" />
        )}
        <img
          src={photo.thumbnail_url ?? photo.url}
          alt={photo.title ?? "徒步照片"}
          className={`h-full w-full object-cover transition-all duration-300 group-hover:scale-105 ${
            loaded ? "opacity-100" : "opacity-0"
          }`}
          onLoad={() => setLoaded(true)}
          loading="lazy"
        />
        <div className="absolute right-2 top-2 flex gap-1">
          <ScoreBadge score={photo.ai_score} />
        </div>

        {onDelete && (
          <button
            onClick={() => onDelete(photo.id)}
            className="absolute left-2 top-2 rounded-full bg-white/80 p-1.5 text-gray-600 opacity-0 transition-opacity hover:bg-white hover:text-red-600 group-hover:opacity-100"
          >
            <Trash2 className="h-4 w-4" />
          </button>
        )}
      </div>

      <div className="p-3">
        <div className="flex items-center justify-between">
          <h3 className="truncate text-sm font-medium text-gray-900">
            {photo.title ?? "未命名"}
          </h3>
          <StatusBadge status={photo.status} />
        </div>
        {photo.description && (
          <p className="mt-1 line-clamp-2 text-xs text-gray-500">
            {photo.description}
          </p>
        )}
        <p className="mt-2 text-xs text-gray-400">
          {new Date(photo.created_at).toLocaleDateString("zh-CN")}
        </p>
      </div>
    </div>
  );
}
