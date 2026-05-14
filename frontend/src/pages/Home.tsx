import { Link } from "react-router-dom";
import { Upload, Sparkles, CalendarDays, ArrowRight, Compass, TreePine, Camera } from "lucide-react";
import { usePhotos } from "@/hooks/usePhotos";
import { useEvents } from "@/hooks/useEvents";
import { PhotoGrid } from "@/components/PhotoGrid";
import { EventCard } from "@/components/EventCard";
import { CanvasHero } from "@/components/CanvasHero";
import { MountainBackground } from "@/components/MountainBackground";

export default function Home() {
  const { data: photos = [], isLoading } = usePhotos({
    status: "approved",
    limit: 8,
  });
  const { data: events = [] } = useEvents({ limit: 3 });

  return (
    <div>
      {/* ── Hero ── */}
      <section className="relative mb-16 sm:mb-20 overflow-hidden rounded-3xl px-6 py-20 sm:px-12 sm:py-24 lg:px-16 min-h-[520px] flex items-center">
        <MountainBackground />

        <div className="relative z-10 w-full max-w-4xl mx-auto">
          <div className="mb-6 inline-flex items-center gap-2 rounded-full border border-amber-200/20 bg-amber-900/20 px-4 py-1.5 text-xs text-amber-200/80 backdrop-blur-sm">
            <Compass className="h-3.5 w-3.5" />
            徒步爱好者社区
          </div>

          <CanvasHero />

          <div className="mt-10 flex flex-col gap-3 sm:flex-row">
            <Link
              to="/upload"
              className="group inline-flex items-center justify-center gap-2 rounded-full bg-amber-600 px-7 py-3.5 text-sm font-medium text-cream-50 hover:bg-amber-500 active:bg-amber-700 transition-all duration-200 shadow-lg shadow-amber-900/30 hover:shadow-amber-800/40 hover:-translate-y-0.5"
            >
              <Camera className="h-4 w-4 group-hover:scale-110 transition-transform" />
              上传照片
            </Link>
            <Link
              to="/events"
              className="inline-flex items-center justify-center gap-2 rounded-full border border-white/15 bg-white/8 px-7 py-3.5 text-sm font-medium text-cream-100 hover:bg-white/15 hover:border-white/25 transition-all duration-200 backdrop-blur-sm hover:-translate-y-0.5"
            >
              <TreePine className="h-4 w-4" />
              浏览活动
            </Link>
            <Link
              to="/gallery"
              className="inline-flex items-center justify-center gap-2 rounded-full border border-white/8 bg-transparent px-7 py-3.5 text-sm font-medium text-cream-300/80 hover:text-cream-100 hover:border-white/15 hover:bg-white/5 transition-all duration-200 hover:-translate-y-0.5"
            >
              <Sparkles className="h-4 w-4" />
              浏览画廊
              <ArrowRight className="h-3.5 w-3.5" />
            </Link>
          </div>
        </div>
      </section>

      {/* ── Features ── */}
      <section className="mb-16 sm:mb-20">
        <div className="text-center mb-10">
          <p className="font-display text-xs tracking-widest uppercase text-clay-400">
            功能
          </p>
          <h2 className="font-display text-2xl sm:text-3xl font-semibold text-clay-900 mt-2">
            为什么选择 GoHiking
          </h2>
        </div>

        <div className="grid gap-5 sm:gap-6 grid-cols-1 sm:grid-cols-3">
          {[
            {
              icon: Upload,
              title: "上传照片",
              desc: "支持 JPG / PNG / WebP 格式，拖拽即可上传",
              color: "bg-forest-100 text-forest-700",
            },
            {
              icon: Sparkles,
              title: "AI 智能评分",
              desc: "从构图、光线、清晰度、主题等多维度自动评分",
              color: "bg-earth-100 text-earth-700",
            },
            {
              icon: CalendarDays,
              title: "活动组织",
              desc: "创建徒步活动，邀请朋友一起分享照片",
              color: "bg-clay-100 text-clay-700",
            },
          ].map(({ icon: Icon, title, desc, color }, i) => (
            <div
              key={title}
              className="rounded-2xl border border-clay-200 bg-white p-6 sm:p-8 shadow-sm transition-all duration-300 hover:shadow-lg hover:-translate-y-0.5 animate-slide-up"
              style={{ animationDelay: `${i * 100}ms` }}
            >
              <div
                className={`mb-4 inline-flex h-14 w-14 items-center justify-center rounded-2xl ${color}`}
              >
                <Icon className="h-7 w-7" />
              </div>
              <h3 className="font-display text-lg font-semibold text-clay-900 mb-2">
                {title}
              </h3>
              <p className="text-sm text-clay-500 leading-relaxed">{desc}</p>
            </div>
          ))}
        </div>
      </section>

      {/* ── Latest Events ── */}
      {events.length > 0 && (
        <section className="mb-16 sm:mb-20">
          <div className="flex items-center gap-3 mb-6">
            <h2 className="font-display text-xl sm:text-2xl font-semibold text-clay-900">
              最新活动
            </h2>
            <span className="h-px flex-1 bg-gradient-to-r from-clay-200 to-transparent" />
            <Link
              to="/events"
              className="inline-flex items-center gap-1 text-sm font-medium text-forest-600 hover:text-forest-700 transition-colors shrink-0"
            >
              查看全部
              <ArrowRight className="h-3.5 w-3.5" />
            </Link>
          </div>

          <div className="grid gap-4">
            {events.map((event, i) => (
              <div
                key={event.id}
                className="animate-slide-up"
                style={{ animationDelay: `${i * 80}ms` }}
              >
                <EventCard event={event} />
              </div>
            ))}
          </div>
        </section>
      )}

      {/* ── Gallery Preview ── */}
      <section className="mb-8">
        <div className="flex items-center gap-3 mb-6">
          <h2 className="font-display text-xl sm:text-2xl font-semibold text-clay-900">
            最新精选
          </h2>
          <span className="h-px flex-1 bg-gradient-to-r from-clay-200 to-transparent" />
          <Link
            to="/gallery"
            className="inline-flex items-center gap-1 text-sm font-medium text-forest-600 hover:text-forest-700 transition-colors shrink-0"
          >
            查看全部
            <ArrowRight className="h-3.5 w-3.5" />
          </Link>
        </div>

        <PhotoGrid photos={photos} isLoading={isLoading} />
      </section>
    </div>
  );
}
