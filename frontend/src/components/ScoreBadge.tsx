import type { Photo } from "@/types";

const colorMap: Record<string, string> = {
  approved: "tag-green",
  rejected: "tag-red",
  pending: "tag-yellow",
};

export function ScoreBadge({
  score,
  size = "sm",
}: {
  score: number;
  size?: "sm" | "lg";
}) {
  const dotColor =
    score >= 80
      ? "bg-forest-500"
      : score >= 60
        ? "bg-earth-500"
        : "bg-red-500";

  return (
    <span
      className={`inline-flex items-center gap-1.5 font-semibold tabular-nums ${
        size === "lg" ? "text-lg" : "text-xs"
      }`}
    >
      <span className={`h-1.5 w-1.5 rounded-full ${dotColor}`} />
      <span
        className={
          score >= 80
            ? "text-forest-700"
            : score >= 60
              ? "text-earth-700"
              : "text-red-700"
        }
      >
        {score.toFixed(0)}
      </span>
    </span>
  );
}

export function StatusBadge({ status }: { status: Photo["status"] }) {
  return (
    <span className={colorMap[status] ?? "tag-gray"}>
      {status === "approved"
        ? "已通过"
        : status === "rejected"
          ? "未通过"
          : "审核中"}
    </span>
  );
}
