import { useState } from "react";
import { useSearchParams, Link } from "react-router-dom";
import { UploadZone } from "@/components/UploadZone";
import { useUpload } from "@/hooks/useUpload";
import { useEvents } from "@/hooks/useEvents";
import { useAuth } from "@/hooks/useAuth";
import { ScoreBadge, StatusBadge } from "@/components/ScoreBadge";
import { CheckCircle, AlertCircle, Loader2, Upload as UploadIcon, LogIn } from "lucide-react";

export default function UploadPage() {
  const { user } = useAuth();
  const { state, upload, reset } = useUpload();
  const [searchParams] = useSearchParams();
  const preselectedEvent = searchParams.get("event_id");

  const [title, setTitle] = useState("");
  const [description, setDescription] = useState("");
  const [eventId, setEventId] = useState(preselectedEvent ?? "");

  const { data: events = [] } = useEvents({ limit: 100 });

  if (!user) {
    return (
      <div className="mx-auto max-w-lg px-4 text-center py-16 sm:py-24">
        <div className="mb-6 inline-flex h-24 w-24 items-center justify-center rounded-full bg-clay-100">
          <LogIn className="h-12 w-12 text-clay-500" />
        </div>
        <h2 className="font-display text-2xl font-semibold text-clay-900 mb-2">
          请先登录
        </h2>
        <p className="text-clay-500 mb-8">
          登录后才能上传照片
        </p>
        <Link to="/login" className="btn-primary inline-flex items-center gap-2">
          <LogIn className="h-4 w-4" />
          去登录
        </Link>
      </div>
    );
  }

  const handleDrop = (files: File[]) => {
    const file = files[0];
    if (file) {
      upload(file, title || undefined, description || undefined, eventId || undefined);
    }
  };

  if (state.status === "success" && state.photo) {
    return (
      <div className="mx-auto max-w-lg px-4 text-center py-8 sm:py-12 animate-scale-in">
        {/* Success state */}
        <div className="mb-6 inline-flex h-24 w-24 items-center justify-center rounded-full bg-forest-100">
          <CheckCircle className="h-12 w-12 text-forest-600" />
        </div>
        <h2 className="font-display text-2xl font-semibold text-clay-900 mb-2">
          上传成功！
        </h2>
        <p className="text-clay-500 mb-3">
          你的照片已通过 AI 评分
        </p>
        <div className="mb-8 inline-flex items-center gap-4 rounded-full border border-clay-200 bg-cream-50 px-5 py-2">
          <ScoreBadge score={state.photo.ai_score} size="lg" />
          <span className="text-clay-300">|</span>
          <StatusBadge status={state.photo.status} />
        </div>
        <div>
          <button onClick={reset} className="btn-primary">
            <UploadIcon className="h-4 w-4" />
            继续上传
          </button>
        </div>
      </div>
    );
  }

  return (
    <div className="mx-auto max-w-2xl px-4">
      {/* Header */}
      <div className="mb-8">
        <h1 className="font-display text-3xl font-semibold text-clay-900">
          上传照片
        </h1>
        <p className="mt-2 text-clay-500">
          上传你的徒步照片，AI 会自动评分筛选。评分超过 60 分的照片将展示在画廊中。
        </p>
      </div>

      {/* Form fields */}
      <div className="mb-6 space-y-5 rounded-2xl border border-clay-200 bg-white p-6 sm:p-8 shadow-sm">
        <div>
          <label className="mb-1.5 block text-sm font-medium text-clay-700">
            所属活动（可选）
          </label>
          <select
            value={eventId}
            onChange={(e) => setEventId(e.target.value)}
            className="input-field"
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
          <label className="mb-1.5 block text-sm font-medium text-clay-700">
            标题（可选）
          </label>
          <input
            type="text"
            value={title}
            onChange={(e) => setTitle(e.target.value)}
            placeholder="给这张照片起个名字..."
            className="input-field"
            disabled={state.status === "uploading"}
          />
        </div>
        <div>
          <label className="mb-1.5 block text-sm font-medium text-clay-700">
            描述（可选）
          </label>
          <textarea
            value={description}
            onChange={(e) => setDescription(e.target.value)}
            placeholder="说说这张照片背后的故事..."
            rows={3}
            className="input-field"
            disabled={state.status === "uploading"}
          />
        </div>
      </div>

      {/* Upload zone */}
      <UploadZone onDrop={handleDrop} disabled={state.status === "uploading"} />

      {/* Upload progress */}
      {state.status === "uploading" && (
        <div className="mt-6 rounded-2xl border border-clay-200 bg-white p-6 text-center shadow-sm animate-fade-in">
          <Loader2 className="mx-auto mb-3 h-8 w-8 animate-spin text-forest-600" />
          <p className="text-sm text-clay-600">
            正在上传... {state.progress}%
          </p>
          <div className="mt-3 h-2 w-full max-w-sm mx-auto overflow-hidden rounded-full bg-clay-200">
            <div
              className="h-full rounded-full bg-forest-500 transition-all duration-300 ease-out"
              style={{ width: `${state.progress}%` }}
            />
          </div>
        </div>
      )}

      {/* Scoring in progress */}
      {state.status === "scoring" && (
        <div className="mt-6 rounded-2xl border border-clay-200 bg-white p-6 text-center shadow-sm animate-fade-in">
          <Loader2 className="mx-auto mb-3 h-8 w-8 animate-spin text-earth-500" />
          <p className="text-sm text-clay-600">AI 正在分析照片质量...</p>
          <div className="mt-3 flex justify-center gap-2">
            <span className="h-1.5 w-1.5 rounded-full bg-earth-400 animate-bounce" style={{ animationDelay: "0ms" }} />
            <span className="h-1.5 w-1.5 rounded-full bg-earth-400 animate-bounce" style={{ animationDelay: "150ms" }} />
            <span className="h-1.5 w-1.5 rounded-full bg-earth-400 animate-bounce" style={{ animationDelay: "300ms" }} />
          </div>
        </div>
      )}

      {/* Error */}
      {state.status === "error" && (
        <div className="mt-6 flex items-center justify-center gap-2 rounded-2xl border border-red-200 bg-red-50 px-5 py-4 text-red-700 animate-fade-in">
          <AlertCircle className="h-5 w-5 shrink-0" />
          <span className="text-sm">{state.error}</span>
        </div>
      )}
    </div>
  );
}
