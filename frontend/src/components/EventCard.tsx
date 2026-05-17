import { Link } from "react-router-dom";
import { Calendar, MapPin, Users, Camera, ArrowRight, TrendingUp } from "lucide-react";
import type { Event } from "@/types";

interface EventCardProps {
  event: Event;
}

export function EventCard({ event }: EventCardProps) {
  const dateObj = event.date ? new Date(event.date) : null;

  return (
    <Link
      to={`/events/${event.id}`}
      className="group relative flex overflow-hidden rounded-2xl border border-clay-200 bg-white shadow-sm transition-all duration-300 hover:shadow-lg hover:-translate-y-0.5"
    >
      {/* Date sidebar */}
      {dateObj && (
        <div className="hidden sm:flex w-20 flex-col items-center justify-center bg-forest-50 border-r border-clay-200 shrink-0">
          <span className="font-display text-2xl font-semibold text-forest-700">
            {dateObj.getDate()}
          </span>
          <span className="text-xs text-forest-600">
            {dateObj.toLocaleDateString("zh-CN", { month: "short" })}
          </span>
        </div>
      )}

      <div className="flex flex-1 flex-col p-4 sm:p-5">
        <h3 className="font-display text-lg font-semibold text-clay-900 group-hover:text-forest-700 transition-colors">
          {event.title}
        </h3>
        {event.description && (
          <p className="mt-1 mb-3 line-clamp-2 text-sm text-clay-500">
            {event.description}
          </p>
        )}
        <div className="mt-auto flex flex-wrap items-center gap-3 text-xs text-clay-400">
          {!event.date && (
            <span className="inline-flex items-center gap-1">
              <Calendar className="h-3.5 w-3.5" />
              待定
            </span>
          )}
          {event.location && (
            <span className="inline-flex items-center gap-1">
              <MapPin className="h-3.5 w-3.5" />
              {event.location}
            </span>
          )}
          {event.distance_km && (
            <span className="inline-flex items-center gap-1">
              <TrendingUp className="h-3.5 w-3.5" />
              {event.distance_km}km
            </span>
          )}
          <span className="inline-flex items-center gap-1">
            <Users className="h-3.5 w-3.5" />
            {event.member_count}
          </span>
          <span className="inline-flex items-center gap-1">
            <Camera className="h-3.5 w-3.5" />
            {event.photo_count}
          </span>
        </div>
      </div>

      <div className="hidden items-center pr-5 sm:flex">
        <span className="flex h-8 w-8 items-center justify-center rounded-full bg-clay-100 text-clay-400 transition-colors group-hover:bg-forest-100 group-hover:text-forest-600">
          <ArrowRight className="h-4 w-4" />
        </span>
      </div>
    </Link>
  );
}
