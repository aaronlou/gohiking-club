import { Camera } from "lucide-react";
import type { Photo } from "@/types";

const colorMap: Record<string, string> = {
  approved: "bg-green-100 text-green-800",
  rejected: "bg-red-100 text-red-800",
  pending: "bg-yellow-100 text-yellow-800",
};

export function ScoreBadge({
  score,
  size = "sm",
}: {
  score: number;
  size?: "sm" | "lg";
}) {
  const color =
    score >= 80
      ? "text-green-600"
      : score >= 60
        ? "text-yellow-600"
        : "text-red-600";

  return (
    <span
      className={`inline-flex items-center gap-1 font-semibold ${color} ${
        size === "lg" ? "text-lg" : "text-sm"
      }`}
    >
      <Camera className={size === "lg" ? "h-5 w-5" : "h-3.5 w-3.5"} />
      {score.toFixed(0)}
    </span>
  );
}

export function StatusBadge({ status }: { status: Photo["status"] }) {
  return (
    <span
      className={`inline-flex items-center rounded-full px-2 py-0.5 text-xs font-medium ${
        colorMap[status] ?? "bg-gray-100 text-gray-800"
      }`}
    >
      {status === "approved"
        ? "已通过"
        : status === "rejected"
          ? "未通过"
          : "审核中"}
    </span>
  );
}
