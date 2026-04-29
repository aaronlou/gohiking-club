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

  const inputClass =
    "w-full rounded-lg border border-gray-300 px-3 py-2.5 text-sm focus:border-brand-500 focus:outline-none focus:ring-1 focus:ring-brand-500";

  return (
    <div className="mx-auto max-w-2xl">
      <Link
        to="/events"
        className="mb-4 inline-flex items-center gap-1 text-sm text-gray-500 hover:text-gray-700"
      >
        <ArrowLeft className="h-4 w-4" />
        返回活动列表
      </Link>

      <h1 className="mb-2 text-2xl font-bold text-gray-900">创建徒步活动</h1>
      <p className="mb-8 text-gray-600">
        创建一个活动，让其他人加入并分享徒步照片
      </p>

      <form onSubmit={handleSubmit} className="space-y-5">
        <div>
          <label className="mb-1.5 block text-sm font-medium text-gray-700">
            活动名称 <span className="text-red-500">*</span>
          </label>
          <input
            type="text"
            value={title}
            onChange={(e) => setTitle(e.target.value)}
            placeholder="例如：周末梧桐山徒步"
            className={inputClass}
            required
          />
        </div>

        <div>
          <label className="mb-1.5 block text-sm font-medium text-gray-700">
            活动描述
          </label>
          <textarea
            value={description}
            onChange={(e) => setDescription(e.target.value)}
            placeholder="描述一下这次活动的路线、难度、集合地点等信息..."
            rows={4}
            className={inputClass}
          />
        </div>

        <div className="grid gap-5 sm:grid-cols-2">
          <div>
            <label className="mb-1.5 block text-sm font-medium text-gray-700">
              活动地点
            </label>
            <input
              type="text"
              value={location}
              onChange={(e) => setLocation(e.target.value)}
              placeholder="例如：深圳梧桐山"
              className={inputClass}
            />
          </div>
          <div>
            <label className="mb-1.5 block text-sm font-medium text-gray-700">
              活动日期
            </label>
            <input
              type="date"
              value={date}
              onChange={(e) => setDate(e.target.value)}
              className={inputClass}
            />
          </div>
        </div>

        <div className="flex flex-col gap-3 pt-2 sm:flex-row-reverse">
          <button
            type="submit"
            disabled={!title.trim() || createMutation.isPending}
            className="btn-primary gap-2 px-6 py-2.5"
          >
            {createMutation.isPending && (
              <Loader2 className="h-4 w-4 animate-spin" />
            )}
            创建活动
          </button>
          <Link to="/events" className="btn-secondary px-6 py-2.5 text-center">
            取消
          </Link>
        </div>
      </form>
    </div>
  );
}
