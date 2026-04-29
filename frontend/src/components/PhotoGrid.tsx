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
          <div key={i} className="card overflow-hidden">
            <div className="aspect-[4/3] animate-pulse bg-gray-200" />
            <div className="space-y-2 p-3">
              <div className="h-4 w-2/3 animate-pulse rounded bg-gray-200" />
              <div className="h-3 w-full animate-pulse rounded bg-gray-100" />
            </div>
          </div>
        ))}
      </div>
    );
  }

  if (photos.length === 0) {
    return (
      <div className="flex flex-col items-center justify-center py-20 text-gray-400">
        <ImageIcon className="mb-4 h-16 w-16" />
        <p className="text-lg font-medium">还没有照片</p>
        <p className="mt-1 text-sm">上传你的第一张徒步照片吧！</p>
      </div>
    );
  }

  return (
    <div className="grid grid-cols-1 gap-4 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4">
      {photos.map((photo) => (
        <PhotoCard key={photo.id} photo={photo} onDelete={onDelete} />
      ))}
    </div>
  );
}
