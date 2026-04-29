import { useState } from "react";
import { useSearchParams } from "react-router-dom";
import { UploadZone } from "@/components/UploadZone";
import { useUpload } from "@/hooks/useUpload";
import { useEvents } from "@/hooks/useEvents";
import { ScoreBadge, StatusBadge } from "@/components/ScoreBadge";
import { CheckCircle, AlertCircle, Loader2 } from "lucide-react";

export default function UploadPage() {
  const { state, upload, reset } = useUpload();
  const [searchParams] = useSearchParams();
  const preselectedEvent = searchParams.get("event_id");

  const [title, setTitle] = useState("");
  const [description, setDescription] = useState("");
  const [eventId, setEventId] = useState(preselectedEvent ?? "");

  const { data: events = [] } = useEvents({ limit: 100 });

  const handleDrop = (files: File[]) => {
    const file = files[0];
    if (file) {
      upload(file, title || undefined, description || undefined, eventId || undefined);
    }
  };

  if (state.status === "success" && state.photo) {
    return (
      <div className="mx-auto max-w-lg px-4 text-center">
        <div className="mb-6 inline-flex h-20 w-20 items-center justify-center rounded-full bg-green-100">
          <CheckCircle className="h-10 w-10 text-green-600" />
        </div>
        <h2 className="mb-2 text-2xl font-bold text-gray-900">上传成功！</h2>
        <p className="mb-2 text-gray-600">
          你的照片已通过 AI 评分
        </p>
        <div className="mb-8 flex items-center justify-center gap-3">
          <ScoreBadge score={state.photo.ai_score} size="lg" />
          <StatusBadge status={state.photo.status} />
        </div>
        <div className="flex flex-col gap-3 sm:flex-row sm:justify-center">
          <button onClick={reset} className="btn-primary">
            继续上传
          </button>
        </div>
      </div>
    );
  }

  const inputClass =
    "w-full rounded-lg border border-gray-300 px-3 py-2 text-sm focus:border-brand-500 focus:outline-none focus:ring-1 focus:ring-brand-500";

  return (
    <div className="mx-auto max-w-2xl px-4">
      <h1 className="mb-2 text-2xl font-bold text-gray-900">上传照片</h1>
      <p className="mb-6 text-gray-600">
        上传你的徒步照片，AI 会自动评分筛选。评分超过 60 分的照片将展示在画廊中。
      </p>

      <div className="mb-6 space-y-4">
        {/* Event selector */}
        <div>
          <label className="mb-1 block text-sm font-medium text-gray-700">
            所属活动（可选）
          </label>
          <select
            value={eventId}
            onChange={(e) => setEventId(e.target.value)}
            className={inputClass}
            disabled={state.status === "uploading" || !!preselectedEvent}
          >
            <option value="">不归属活动</option>
            {events.map((e) => (
              <option key={e.id} value={e.id}>
                {e.title}
              </option>
            ))}
          </select>
        </div>
        <div>
          <label className="mb-1 block text-sm font-medium text-gray-700">
            标题（可选）
          </label>
          <input
            type="text"
            value={title}
            onChange={(e) => setTitle(e.target.value)}
            placeholder="给这张照片起个名字..."
            className={inputClass}
            disabled={state.status === "uploading"}
          />
        </div>
        <div>
          <label className="mb-1 block text-sm font-medium text-gray-700">
            描述（可选）
          </label>
          <textarea
            value={description}
            onChange={(e) => setDescription(e.target.value)}
            placeholder="说说这张照片背后的故事..."
            rows={3}
            className={inputClass}
            disabled={state.status === "uploading"}
          />
        </div>
      </div>

      <UploadZone onDrop={handleDrop} disabled={state.status === "uploading"} />

      {/* Upload progress */}
      {state.status === "uploading" && (
        <div className="mt-6 text-center">
          <Loader2 className="mx-auto mb-2 h-8 w-8 animate-spin text-brand-600" />
          <p className="text-sm text-gray-600">
            正在上传... {state.progress}%
          </p>
          <div className="mt-2 h-2 w-full rounded-full bg-gray-200">
            <div
              className="h-full rounded-full bg-brand-500 transition-all"
              style={{ width: `${state.progress}%` }}
            />
          </div>
        </div>
      )}

      {/* Scoring in progress */}
      {state.status === "scoring" && (
        <div className="mt-6 text-center">
          <Loader2 className="mx-auto mb-2 h-8 w-8 animate-spin text-yellow-500" />
          <p className="text-sm text-gray-600">AI 正在分析照片质量...</p>
        </div>
      )}

      {/* Error */}
      {state.status === "error" && (
        <div className="mt-6 flex items-center justify-center gap-2 text-red-600">
          <AlertCircle className="h-5 w-5 shrink-0" />
          <span className="text-sm">{state.error}</span>
        </div>
      )}
    </div>
  );
}
