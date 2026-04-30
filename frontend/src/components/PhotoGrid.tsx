import { ImageIcon } from "lucide-react";
import type { Photo } from "@/types";
import { PhotoCard } from "./PhotoCard";

interface PhotoGridProps {
  photos: Photo[];
  isLoading: boolean;
  onDelete?: (id: string) => void;
}

export function PhotoGrid({ photos, isLoading, onDelete }: PhotoGridProps) {
  if (isLoading) {
    return (
      <div className="grid grid-cols-1 gap-4 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4">
        {Array.from({ length: 8 }).map((_, i) => (
          <div key={i} className="overflow-hidden rounded-2xl border border-clay-200 bg-white shadow-sm">
            <div className="aspect-[4/3] animate-pulse bg-clay-200" />
            <div className="space-y-2 p-4">
              <div className="h-4 w-2/3 animate-pulse rounded bg-clay-200" />
              <div className="h-3 w-full animate-pulse rounded bg-clay-100" />
            </div>
          </div>
        ))}
      </div>
    );
  }

  if (photos.length === 0) {
    return (
      <div className="flex flex-col items-center justify-center py-20 text-clay-400">
        <div className="mb-6 inline-flex h-20 w-20 items-center justify-center rounded-2xl bg-clay-100">
          <ImageIcon className="h-10 w-10" />
        </div>
        <p className="font-display text-xl text-clay-600">还没有照片</p>
        <p className="mt-1 text-sm text-clay-400">
          上传你的第一张徒步照片吧！
        </p>
      </div>
    );
  }

  return (
    <div className="grid grid-cols-1 gap-4 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4">
      {photos.map((photo, i) => (
        <div
          key={photo.id}
          className="animate-fade-in"
          style={{ animationDelay: `${(i % 8) * 60}ms` }}
        >
          <PhotoCard photo={photo} onDelete={onDelete} />
        </div>
      ))}
    </div>
  );
}
