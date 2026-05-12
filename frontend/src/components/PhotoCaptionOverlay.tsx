import type { Photo } from "@/types";

interface PhotoCaptionOverlayProps {
  photo: Photo;
  visible: boolean;
}

export function PhotoCaptionOverlay({ photo, visible }: PhotoCaptionOverlayProps) {
  if (!visible) return null;

  return (
    <div className="absolute inset-0 flex flex-col justify-end bg-gradient-to-t from-black/70 via-black/20 to-transparent p-4 transition-opacity duration-300">
      {photo.title && (
        <p className="text-sm font-medium text-white drop-shadow-sm">
          {photo.title}
        </p>
      )}
      {photo.ai_feedback && (
        <div className="mt-1 text-xs text-white/80">
          <span className="inline-flex items-center gap-1">
            构图 {photo.ai_feedback.dimensions.composition.toFixed(0)} ·
            光线 {photo.ai_feedback.dimensions.lighting.toFixed(0)} ·
            清晰度 {photo.ai_feedback.dimensions.clarity.toFixed(0)}
          </span>
        </div>
      )}
    </div>
  );
}
