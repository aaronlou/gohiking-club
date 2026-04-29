import { Link } from "react-router-dom";
import { Mountain, Upload, Sparkles, CalendarDays } from "lucide-react";
import { usePhotos } from "@/hooks/usePhotos";
import { useEvents } from "@/hooks/useEvents";
import { PhotoGrid } from "@/components/PhotoGrid";
import { EventCard } from "@/components/EventCard";

export default function Home() {
  const { data: photos = [], isLoading } = usePhotos({
    status: "approved",
    limit: 8,
  });
  const { data: events = [] } = useEvents({ limit: 3 });

  return (
    <div>
      {/* Hero */}
      <section className="mb-12 text-center">
        <div className="mb-6 inline-flex h-16 w-16 items-center justify-center rounded-2xl bg-brand-100">
          <Mountain className="h-8 w-8 text-brand-600" />
        </div>
        <h1 className="mb-3 text-3xl font-bold text-gray-900 sm:text-4xl">
          发现徒步之美
        </h1>
        <p className="mx-auto mb-8 max-w-xl px-4 text-base text-gray-600 sm:text-lg">
          GoHiking.Club
          是一个徒步爱好者社区。上传你的徒步照片，AI
          会帮你自动评分筛选，只留下最美的瞬间。
        </p>
        <div className="flex flex-col items-center justify-center gap-3 sm:flex-row">
          <Link to="/upload" className="btn-primary gap-2 w-full sm:w-auto px-6 py-3 text-base">
            <Upload className="h-5 w-5" />
            上传照片
          </Link>
          <Link to="/events" className="btn-secondary gap-2 w-full sm:w-auto px-6 py-3 text-base">
            <CalendarDays className="h-5 w-5" />
            浏览活动
          </Link>
          <Link to="/gallery" className="btn-secondary gap-2 w-full sm:w-auto px-6 py-3 text-base">
            <Sparkles className="h-5 w-5" />
            浏览画廊
          </Link>
        </div>
      </section>

      {/* 功能说明 */}
      <section className="mb-12 grid gap-4 sm:gap-6 grid-cols-1 sm:grid-cols-3">
        {[
          {
            icon: Upload,
            title: "上传照片",
            desc: "支持 JPG / PNG / WebP 格式，拖拽即可上传",
          },
          {
            icon: Sparkles,
            title: "AI 智能评分",
            desc: "从构图、光线、清晰度、主题等多维度自动评分",
          },
          {
            icon: CalendarDays,
            title: "活动组织",
            desc: "创建徒步活动，邀请朋友一起分享照片",
          },
        ].map(({ icon: Icon, title, desc }) => (
          <div key={title} className="card p-5 sm:p-6 text-center">
            <div className="mb-3 inline-flex h-12 w-12 items-center justify-center rounded-xl bg-brand-50">
              <Icon className="h-6 w-6 text-brand-600" />
            </div>
            <h3 className="mb-2 font-semibold text-gray-900">{title}</h3>
            <p className="text-sm text-gray-500">{desc}</p>
          </div>
        ))}
      </section>

      {/* 最新活动 */}
      {events.length > 0 && (
        <section className="mb-12">
          <div className="mb-4 flex items-center justify-between">
            <h2 className="text-xl font-bold text-gray-900">最新活动</h2>
            <Link to="/events" className="text-sm font-medium text-brand-600 hover:text-brand-700">
              查看全部 →
            </Link>
          </div>
          <div className="grid gap-4">
            {events.map((event) => (
              <EventCard key={event.id} event={event} />
            ))}
          </div>
        </section>
      )}

      {/* 最新精选 */}
      <section>
        <div className="mb-4 flex items-center justify-between">
          <h2 className="text-xl font-bold text-gray-900">最新精选</h2>
          <Link to="/gallery" className="text-sm font-medium text-brand-600 hover:text-brand-700">
            查看全部 →
          </Link>
        </div>
        <PhotoGrid photos={photos} isLoading={isLoading} />
      </section>
    </div>
  );
}
