import { useState } from "react";
import { useNavigate, Link } from "react-router-dom";
import { ArrowLeft, Loader2 } from "lucide-react";
import { useCreateEvent } from "@/hooks/useEvents";

export default function CreateEvent() {
  const navigate = useNavigate();
  const createMutation = useCreateEvent();

  const [title, setTitle] = useState("");
  const [description, setDescription] = useState("");
  const [location, setLocation] = useState("");
  const [date, setDate] = useState("");

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!title.trim()) return;

    const event = await createMutation.mutateAsync({
      title: title.trim(),
      description: description.trim() || undefined,
      location: location.trim() || undefined,
      date: date || undefined,
    });

    navigate(`/events/${event.id}`);
  };

  return (
    <div className="mx-auto max-w-2xl">
      <Link
        to="/events"
        className="mb-4 inline-flex items-center gap-1.5 text-sm text-clay-500 hover:text-clay-700 transition-colors"
      >
        <ArrowLeft className="h-4 w-4" />
        返回活动列表
      </Link>

      <h1 className="font-display text-3xl font-semibold text-clay-900">
        创建徒步活动
      </h1>
      <p className="mt-2 text-clay-500">
        创建一个活动，让其他人加入并分享徒步照片
      </p>

      <form onSubmit={handleSubmit} className="mt-8 space-y-6">
        <div className="rounded-2xl border border-clay-200 bg-white p-6 sm:p-8 shadow-sm space-y-6">
          <div>
            <label className="mb-1.5 block text-sm font-medium text-clay-700">
              活动名称 <span className="text-red-500">*</span>
            </label>
            <input
              type="text"
              value={title}
              onChange={(e) => setTitle(e.target.value)}
              placeholder="例如：周末梧桐山徒步"
              className="input-field"
              required
            />
          </div>

          <div>
            <label className="mb-1.5 block text-sm font-medium text-clay-700">
              活动描述
            </label>
            <textarea
              value={description}
              onChange={(e) => setDescription(e.target.value)}
              placeholder="描述一下这次活动的路线、难度、集合地点等信息..."
              rows={4}
              className="input-field"
            />
          </div>

          <div className="grid gap-5 sm:grid-cols-2">
            <div>
              <label className="mb-1.5 block text-sm font-medium text-clay-700">
                活动地点
              </label>
              <input
                type="text"
                value={location}
                onChange={(e) => setLocation(e.target.value)}
                placeholder="例如：深圳梧桐山"
                className="input-field"
              />
            </div>
            <div>
              <label className="mb-1.5 block text-sm font-medium text-clay-700">
                活动日期
              </label>
              <input
                type="date"
                value={date}
                onChange={(e) => setDate(e.target.value)}
                className="input-field"
              />
            </div>
          </div>
        </div>

        <div className="flex flex-col gap-3 sm:flex-row-reverse">
          <button
            type="submit"
            disabled={!title.trim() || createMutation.isPending}
            className="btn-primary px-8 py-3"
          >
            {createMutation.isPending && (
              <Loader2 className="h-4 w-4 animate-spin" />
            )}
            创建活动
          </button>
          <Link to="/events" className="btn-secondary px-8 py-3 text-center">
            取消
          </Link>
        </div>
      </form>
    </div>
  );
}
