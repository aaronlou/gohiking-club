import { useCallback } from "react";
import { useDropzone } from "react-dropzone";
import { Upload, Image } from "lucide-react";

interface UploadZoneProps {
  onDrop: (files: File[]) => void;
  disabled?: boolean;
}

export function UploadZone({ onDrop, disabled }: UploadZoneProps) {
  const handleDrop = useCallback(
    (accepted: File[]) => {
      if (accepted.length > 0) onDrop(accepted);
    },
    [onDrop],
  );

  const { getRootProps, getInputProps, isDragActive } = useDropzone({
    onDrop: handleDrop,
    accept: { "image/*": [".jpg", ".jpeg", ".png", ".webp", ".heic"] },
    maxFiles: 1,
    maxSize: 20 * 1024 * 1024,
    disabled,
  });

  return (
    <div
      {...getRootProps()}
      className={`group relative flex cursor-pointer flex-col items-center justify-center overflow-hidden rounded-2xl border-2 border-dashed p-14 sm:p-16 transition-all duration-300 ${
        isDragActive
          ? "border-forest-500 bg-forest-50/80"
          : "border-clay-300 hover:border-forest-400 hover:bg-cream-50"
      } ${disabled ? "cursor-not-allowed opacity-50" : ""}`}
    >
      <input {...getInputProps()} />

      <div className="relative z-10 flex flex-col items-center">
        <div
          className={`mb-5 inline-flex h-16 w-16 items-center justify-center rounded-2xl transition-all duration-300 ${
            isDragActive
              ? "bg-forest-100 scale-110"
              : "bg-clay-100 group-hover:bg-forest-50"
          }`}
        >
          {isDragActive ? (
            <Upload className="h-8 w-8 text-forest-600" />
          ) : (
            <Image className="h-8 w-8 text-clay-500" />
          )}
        </div>

        {isDragActive ? (
          <p className="font-display text-lg text-forest-700">松手以上传</p>
        ) : (
          <>
            <p className="font-display text-lg text-clay-700">
              拖拽照片到此处
            </p>
            <p className="mt-1 text-sm text-clay-400">或点击选择文件</p>
            <p className="mt-3 text-xs text-clay-400">
              支持 JPG / PNG / WebP，最大 20MB
            </p>
          </>
        )}
      </div>
    </div>
  );
}
