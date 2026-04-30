import { Link } from "react-router-dom";
import { CalendarDays, Plus, Loader2 } from "lucide-react";
import { useEvents } from "@/hooks/useEvents";
import { EventCard } from "@/components/EventCard";

export default function Events() {
  const { data: events = [], isLoading } = useEvents({ limit: 50 });

  return (
    <div>
      {/* Header */}
      <div className="mb-8 flex flex-col gap-4 sm:flex-row sm:items-end sm:justify-between">
        <div>
          <p className="font-display text-xs tracking-widest uppercase text-clay-400">
            探索
          </p>
          <h1 className="font-display text-3xl font-semibold text-clay-900 mt-1">
            徒步活动
          </h1>
          <p className="mt-1.5 text-clay-500">
            创建或加入活动，一起分享徒步照片
          </p>
        </div>
        <Link to="/events/new" className="btn-primary shrink-0 self-start">
          <Plus className="h-4 w-4" />
          <span>创建活动</span>
        </Link>
      </div>

      {/* Decorative line */}
      <div className="mb-8 h-px bg-gradient-to-r from-forest-200 via-earth-300 to-transparent" />

      {isLoading ? (
        <div className="flex justify-center py-20">
          <Loader2 className="h-8 w-8 animate-spin text-forest-600" />
        </div>
      ) : events.length === 0 ? (
        <div className="flex flex-col items-center justify-center py-20 text-clay-400">
          <div className="mb-6 inline-flex h-20 w-20 items-center justify-center rounded-2xl bg-clay-100">
            <CalendarDays className="h-10 w-10" />
          </div>
          <p className="font-display text-xl text-clay-600">还没有活动</p>
          <p className="mt-1 mb-6 text-sm text-clay-400">
            创建一个新活动，开始组织徒步吧！
          </p>
          <Link to="/events/new" className="btn-primary">
            <Plus className="h-4 w-4" />
            创建第一个活动
          </Link>
        </div>
      ) : (
        <div className="grid gap-4 sm:gap-5">
          {events.map((event, i) => (
            <div
              key={event.id}
              className="animate-slide-up"
              style={{ animationDelay: `${i * 60}ms` }}
            >
              <EventCard event={event} />
            </div>
          ))}
        </div>
      )}
    </div>
  );
}
