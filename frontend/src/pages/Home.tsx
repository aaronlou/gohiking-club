import { Link } from "react-router-dom";
import { CalendarDays, ArrowRight, Compass, Users, Sparkles } from "lucide-react";
import { usePhotos } from "@/hooks/usePhotos";
import { useEvents } from "@/hooks/useEvents";
import { PhotoGrid } from "@/components/PhotoGrid";
import { EventCard } from "@/components/EventCard";
import { CanvasHero } from "@/components/CanvasHero";
import { MountainBackground } from "@/components/MountainBackground";

const entryCards = [
  {
    to: "/teams",
    icon: Users,
    title: "加入团队",
    desc: "找到志同道合的徒步伙伴，一起出发探索山野",
    color: "bg-sky-500",
    shadow: "shadow-sky-900/30 hover:shadow-sky-800/40",
  },
  {
    to: "/events",
    icon: CalendarDays,
    title: "活动报名",
    desc: "发现附近的徒步活动，报名参与精彩旅程",
    color: "bg-amber-500",
    shadow: "shadow-amber-900/30 hover:shadow-amber-800/40",
  },
  {
    to: "/memories",
    icon: Sparkles,
    title: "我的回忆",
    desc: "回顾你的徒步足迹、影像和感想",
    color: "bg-emerald-500",
    shadow: "shadow-emerald-900/30 hover:shadow-emerald-800/40",
  },
] as const;

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

          <div className="mt-10 grid gap-4 grid-cols-1 sm:grid-cols-3">
            {entryCards.map(({ to, icon: Icon, title, desc, color, shadow }) => (
              <Link
                key={to}
                to={to}
                className={`group relative overflow-hidden rounded-2xl ${color} px-5 py-5 text-cream-50 transition-all duration-200 shadow-lg ${shadow} hover:-translate-y-1`}
              >
                <div className="relative z-10">
                  <Icon className="h-7 w-7 mb-3 group-hover:scale-110 transition-transform" />
                  <h3 className="font-display text-lg font-semibold">{title}</h3>
                  <p className="mt-1 text-sm text-white/70">{desc}</p>
                </div>
                <div className="absolute top-3 right-3 opacity-0 group-hover:opacity-100 transition-opacity">
                  <ArrowRight className="h-4 w-4" />
                </div>
              </Link>
            ))}
          </div>
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
