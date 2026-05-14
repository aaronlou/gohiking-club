import { useState } from "react";
import { PhotoGrid } from "@/components/PhotoGrid";
import { usePhotos, useDeletePhoto } from "@/hooks/usePhotos";
import { useEvents } from "@/hooks/useEvents";
import { SlidersHorizontal, X } from "lucide-react";
import type { PhotoStatus } from "@/types";

const tabs: { label: string; value: PhotoStatus | "all" }[] = [
  { label: "精选", value: "approved" },
  { label: "全部", value: "all" },
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

  const hasActiveFilters = minScore !== undefined || eventId !== "";

  return (
    <div>
      {/* Hero header with gradient */}
      <div className="relative -mx-4 -mt-8 sm:-mx-6 sm:-mt-12 lg:-mx-8 mb-6 px-4 sm:px-6 lg:px-8 pt-8 sm:pt-12 pb-8 overflow-hidden">
        <div className="absolute inset-0 bg-gradient-to-br from-forest-50 via-cream-50 to-earth-50" />
        <div className="absolute inset-0 bg-[radial-gradient(ellipse_at_top_right,_rgba(22,57,38,0.04)_0%,_transparent_50%)]" />
        <div className="relative flex flex-col gap-4 sm:flex-row sm:items-end sm:justify-between">
          <div>
            <div className="inline-flex items-center gap-2 rounded-full bg-forest-100/80 px-3 py-1 text-xs font-medium text-forest-700 mb-3">
              <SlidersHorizontal className="h-3.5 w-3.5" />
              浏览照片
            </div>
            <h1 className="font-display text-3xl font-semibold text-clay-900">
              照片画廊
            </h1>
            <p className="mt-1.5 text-clay-500 max-w-md">
              徒步社区的高质量照片
            </p>
          </div>
          <button
            onClick={() => setShowFilters(!showFilters)}
            className={`inline-flex items-center gap-2 rounded-full px-4 py-2 text-sm font-medium transition-all shrink-0 self-start ${
              showFilters || hasActiveFilters
                ? "bg-forest-100 text-forest-700 border border-forest-200"
                : "bg-clay-100 text-clay-600 hover:bg-clay-200 border border-transparent"
            }`}
          >
            <SlidersHorizontal className="h-4 w-4" />
            筛选
            {hasActiveFilters && (
              <span className="inline-flex h-2 w-2 rounded-full bg-forest-500" />
            )}
          </button>
        </div>
      </div>

      {/* Tabs */}
      <div className="mb-6 flex gap-2 overflow-x-auto">
        {tabs.map(({ label, value }) => (
          <button
            key={value}
            onClick={() => setTab(value)}
            className={`whitespace-nowrap rounded-full px-5 py-2 text-sm font-medium transition-all ${
              tab === value
                ? "bg-forest-700 text-cream-50 shadow-sm"
                : "bg-white text-clay-600 border border-clay-200 hover:border-clay-300 hover:text-clay-800"
            }`}
          >
            {label}
          </button>
        ))}
      </div>

      {/* Filters panel */}
      {showFilters && (
        <div className="mb-6 rounded-2xl border border-clay-200 bg-white p-5 shadow-sm animate-slide-up-sm">
          <div className="flex items-center justify-between mb-4">
            <span className="text-sm font-medium text-clay-700">筛选条件</span>
            {hasActiveFilters && (
              <button
                onClick={() => {
                  setMinScore(undefined);
                  setEventId("");
                }}
                className="inline-flex items-center gap-1 text-xs text-clay-500 hover:text-clay-700 transition-colors"
              >
                <X className="h-3 w-3" />
                清除筛选
              </button>
            )}
          </div>

          <div className="grid gap-5 sm:grid-cols-2">
            <div>
              <label className="block text-sm text-clay-600 mb-2">
                最低评分：<span className="font-medium text-clay-800">{minScore ?? "不限"}</span>
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
                className="w-full accent-forest-600 h-2 rounded-full appearance-none bg-clay-200 cursor-pointer [&::-webkit-slider-thumb]:appearance-none [&::-webkit-slider-thumb]:h-4 [&::-webkit-slider-thumb]:w-4 [&::-webkit-slider-thumb]:rounded-full [&::-webkit-slider-thumb]:bg-forest-600 [&::-webkit-slider-thumb]:shadow-sm"
              />
              <div className="mt-1 flex justify-between text-xs text-clay-400">
                <span>0</span>
                <span>50</span>
                <span>100</span>
              </div>
            </div>

            <div>
              <label className="block text-sm text-clay-600 mb-2">
                所属活动
              </label>
              <select
                value={eventId}
                onChange={(e) => setEventId(e.target.value)}
                className="input-field"
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
