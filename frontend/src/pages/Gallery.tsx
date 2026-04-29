import { useState } from "react";
import { PhotoGrid } from "@/components/PhotoGrid";
import { usePhotos, useDeletePhoto } from "@/hooks/usePhotos";
import { useEvents } from "@/hooks/useEvents";
import type { PhotoStatus } from "@/types";

const tabs: { label: string; value: PhotoStatus | "all" }[] = [
  { label: "全部", value: "all" },
  { label: "已通过", value: "approved" },
  { label: "审核中", value: "pending" },
  { label: "未通过", value: "rejected" },
];

export default function Gallery() {
  const [tab, setTab] = useState<PhotoStatus | "all">("approved");
  const [minScore, setMinScore] = useState<number | undefined>();
  const [eventId, setEventId] = useState<string>("");
  const [showFilters, setShowFilters] = useState(false);

  const { data: events = [] } = useEvents({ limit: 100 });

  const filter = {
    ...(tab !== "all" ? { status: tab as PhotoStatus } : {}),
    ...(minScore !== undefined ? { min_score: minScore } : {}),
    ...(eventId ? { event_id: eventId } : {}),
    limit: 50,
  };

  const { data: photos = [], isLoading } = usePhotos(filter);
  const deleteMutation = useDeletePhoto();

  return (
    <div>
      <div className="mb-6 flex flex-col gap-4 sm:flex-row sm:items-center sm:justify-between">
        <div>
          <h1 className="text-2xl font-bold text-gray-900">照片画廊</h1>
          <p className="text-sm text-gray-500">
            浏览徒步社区的高质量照片
          </p>
        </div>
        <button
          onClick={() => setShowFilters(!showFilters)}
          className="btn-secondary text-sm w-full sm:w-auto"
        >
          {showFilters ? "收起筛选" : "展开筛选"}
        </button>
      </div>

      {/* Tabs */}
      <div className="mb-6 flex gap-1 rounded-lg bg-gray-100 p-1 overflow-x-auto">
        {tabs.map(({ label, value }) => (
          <button
            key={value}
            onClick={() => setTab(value)}
            className={`flex-1 whitespace-nowrap rounded-md px-4 py-2 text-sm font-medium transition-colors ${
              tab === value
                ? "bg-white text-gray-900 shadow-sm"
                : "text-gray-600 hover:text-gray-900"
            }`}
          >
            {label}
          </button>
        ))}
      </div>

      {/* Filters */}
      {showFilters && (
        <div className="mb-6 card space-y-4 p-4">
          <div>
            <label className="block text-sm font-medium text-gray-700">
              最低评分：{minScore ?? "不限"}
            </label>
            <input
              type="range"
              min={0}
              max={100}
              step={5}
              value={minScore ?? 0}
              onChange={(e) => {
                const v = Number(e.target.value);
                setMinScore(v > 0 ? v : undefined);
              }}
              className="mt-1 w-full accent-brand-600"
            />
            <div className="mt-1 flex justify-between text-xs text-gray-400">
              <span>0</span>
              <span>50</span>
              <span>100</span>
            </div>
          </div>
          <div>
            <label className="block text-sm font-medium text-gray-700">
              所属活动
            </label>
            <select
              value={eventId}
              onChange={(e) => setEventId(e.target.value)}
              className="mt-1 w-full rounded-lg border border-gray-300 px-3 py-2 text-sm"
            >
              <option value="">全部活动</option>
              {events.map((e) => (
                <option key={e.id} value={e.id}>
                  {e.title} ({e.photo_count})
                </option>
              ))}
            </select>
          </div>
        </div>
      )}

      <PhotoGrid
        photos={photos}
        isLoading={isLoading}
        onDelete={(id) => deleteMutation.mutate(id)}
      />
    </div>
  );
}
