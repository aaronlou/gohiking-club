import { Link } from "react-router-dom";
import { CalendarDays, Plus, Loader2, MapPin } from "lucide-react";
import { useState } from "react";
import { useAuth } from "@/hooks/useAuth";
import { AuthModal } from "@/components/AuthModal";
import { useEvents } from "@/hooks/useEvents";
import { EventCard } from "@/components/EventCard";

export default function Events() {
  const { data: events = [], isLoading } = useEvents({ limit: 50 });
  const user = useAuth((s) => s.user);
  const [authModalOpen, setAuthModalOpen] = useState(false);

  return (
    <div className="animate-fade-in">
      {/* Hero header with gradient */}
      <div className="relative -mx-4 -mt-8 sm:-mx-6 sm:-mt-12 lg:-mx-8 mb-8 px-4 sm:px-6 lg:px-8 pt-8 sm:pt-12 pb-8 overflow-hidden">
        <div className="absolute inset-0 bg-gradient-to-br from-forest-50 via-cream-50 to-earth-50" />
        <div className="absolute inset-0 bg-[radial-gradient(ellipse_at_top_right,_rgba(22,57,38,0.04)_0%,_transparent_50%)]" />
        <div className="relative flex flex-col gap-4 sm:flex-row sm:items-end sm:justify-between">
          <div>
            <div className="inline-flex items-center gap-2 rounded-full bg-earth-100/80 px-3 py-1 text-xs font-medium text-earth-700 mb-3">
              <MapPin className="h-3.5 w-3.5" />
              探索活动
            </div>
            <h1 className="font-display text-3xl font-semibold text-clay-900">
              徒步活动
            </h1>
            <p className="mt-1.5 text-clay-500 max-w-md">
              创建或加入活动，一起分享徒步照片
            </p>
          </div>
          {user ? (
            <Link to="/events/new" className="btn-primary shrink-0 self-start">
              <Plus className="h-4 w-4" />
              <span>创建活动</span>
            </Link>
          ) : (
            <button
              onClick={() => setAuthModalOpen(true)}
              className="btn-primary shrink-0 self-start"
            >
              <Plus className="h-4 w-4" />
              <span>创建活动</span>
            </button>
          )}
        </div>
      </div>

      {/* Decorative line */}
      <div className="mb-8 h-px bg-gradient-to-r from-forest-200 via-earth-300 to-transparent" />

      {isLoading ? (
        <div className="flex justify-center py-20">
          <Loader2 className="h-8 w-8 animate-spin text-forest-600" />
        </div>
      ) : events.length === 0 ? (
        <div className="relative rounded-2xl border border-clay-100 bg-white p-12 text-center overflow-hidden">
          <div className="absolute inset-0 bg-gradient-to-b from-earth-50/50 to-transparent" />
          <div className="relative">
            <div className="mx-auto mb-5 inline-flex h-20 w-20 items-center justify-center rounded-2xl bg-gradient-to-br from-earth-100 to-forest-50">
              <CalendarDays className="h-10 w-10 text-earth-600" />
            </div>
            <p className="font-display text-xl font-semibold text-clay-800 mb-1">
              还没有活动
            </p>
            <p className="text-clay-500 mb-8 max-w-xs mx-auto">
              创建一个新活动，开始组织徒步吧！
            </p>
            <button
              onClick={() => setAuthModalOpen(true)}
              className="btn-primary"
            >
              <Plus className="h-4 w-4" />
              创建第一个活动
            </button>
          </div>
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
      <AuthModal isOpen={authModalOpen} onClose={() => setAuthModalOpen(false)} defaultMode="register" />
    </div>
  );
}
