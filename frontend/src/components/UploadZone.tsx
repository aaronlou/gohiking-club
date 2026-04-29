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
    maxSize: 20 * 1024 * 1024, // 20MB
    disabled,
  });

  return (
    <div
      {...getRootProps()}
      className={`card flex cursor-pointer flex-col items-center justify-center border-2 border-dashed p-12 transition-colors ${
        isDragActive
          ? "border-brand-500 bg-brand-50"
          : "border-gray-300 hover:border-gray-400 hover:bg-gray-50"
      } ${disabled ? "cursor-not-allowed opacity-50" : ""}`}
    >
      <input {...getInputProps()} />
      {isDragActive ? (
        <>
          <Upload className="mb-4 h-12 w-12 text-brand-500" />
          <p className="text-lg font-medium text-brand-600">松手以上传</p>
        </>
      ) : (
        <>
          <Image className="mb-4 h-12 w-12 text-gray-400" />
          <p className="text-lg font-medium text-gray-700">
            拖拽照片到此处，或点击选择
          </p>
          <p className="mt-1 text-sm text-gray-500">
            支持 JPG / PNG / WebP，最大 20MB
          </p>
        </>
      )}
    </div>
  );
}
