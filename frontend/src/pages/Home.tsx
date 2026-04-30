import { Link } from "react-router-dom";
import { Mountain, Upload, Sparkles, CalendarDays, ArrowRight } from "lucide-react";
import { usePhotos } from "@/hooks/usePhotos";
import { useEvents } from "@/hooks/useEvents";
import { PhotoGrid } from "@/components/PhotoGrid";
import { EventCard } from "@/components/EventCard";
import { CanvasHero } from "@/components/CanvasHero";

export default function Home() {
  const { data: photos = [], isLoading } = usePhotos({
    status: "approved",
    limit: 8,
  });
  const { data: events = [] } = useEvents({ limit: 3 });

  return (
    <div>
      {/* ── Hero ── */}
      <section className="relative mb-16 sm:mb-20 overflow-hidden rounded-3xl bg-gradient-to-b from-forest-900 via-forest-800 to-forest-950 px-6 py-16 sm:px-12 sm:py-20 lg:px-16">
        {/* Topographic overlay */}
        <div className="absolute inset-0 opacity-[0.08] bg-[url('/topo-pattern-flip.svg')] bg-[length:300px_300px]" />

        {/* Decorative gradient orbs */}
        <div className="absolute -right-32 -top-32 h-64 w-64 rounded-full bg-earth-500/10 blur-3xl" />
        <div className="absolute -bottom-32 -left-32 h-64 w-64 rounded-full bg-forest-400/10 blur-3xl" />

        <div className="relative z-10">
          <div className="mb-5 inline-flex items-center gap-2 rounded-full border border-white/10 bg-white/5 px-4 py-1.5 text-xs text-cream-300 backdrop-blur-sm">
            <Mountain className="h-3.5 w-3.5" />
            徒步爱好者社区
          </div>

          <CanvasHero />

          <div className="mt-8 flex flex-col gap-3 sm:flex-row">
            <Link
              to="/upload"
              className="inline-flex items-center justify-center gap-2 rounded-full bg-earth-500 px-6 py-3 text-sm font-medium text-cream-50 hover:bg-earth-600 active:bg-earth-700 transition-all duration-200 shadow-lg shadow-earth-500/20"
            >
              <Upload className="h-4 w-4" />
              上传照片
            </Link>
            <Link
              to="/events"
              className="inline-flex items-center justify-center gap-2 rounded-full border border-white/20 bg-white/10 px-6 py-3 text-sm font-medium text-cream-100 hover:bg-white/20 transition-all duration-200 backdrop-blur-sm"
            >
              <CalendarDays className="h-4 w-4" />
              浏览活动
            </Link>
            <Link
              to="/gallery"
              className="inline-flex items-center justify-center gap-2 rounded-full border border-white/10 bg-transparent px-6 py-3 text-sm font-medium text-cream-300 hover:text-cream-100 hover:border-white/20 transition-all duration-200"
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
