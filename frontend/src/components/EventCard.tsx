import { Link } from "react-router-dom";
import { Calendar, MapPin, Users, Camera, ArrowRight } from "lucide-react";
import type { Event } from "@/types";

interface EventCardProps {
  event: Event;
}

export function EventCard({ event }: EventCardProps) {
  const formatDate = (d: string | null) => {
    if (!d) return "待定";
    try {
      return new Date(d).toLocaleDateString("zh-CN", {
        month: "long",
        day: "numeric",
      });
    } catch {
      return d;
    }
  };

  return (
    <Link
      to={`/events/${event.id}`}
      className="card group flex overflow-hidden transition-shadow hover:shadow-md"
    >
      <div className="flex flex-1 flex-col p-4 sm:p-5">
        <h3 className="mb-1 text-lg font-semibold text-gray-900 group-hover:text-brand-600 transition-colors">
          {event.title}
        </h3>
        {event.description && (
          <p className="mb-3 line-clamp-2 text-sm text-gray-500">
            {event.description}
          </p>
        )}
        <div className="mt-auto flex flex-wrap items-center gap-3 text-xs text-gray-400">
          {event.date && (
            <span className="inline-flex items-center gap-1">
              <Calendar className="h-3.5 w-3.5" />
              {formatDate(event.date)}
            </span>
          )}
          {event.location && (
            <span className="inline-flex items-center gap-1">
              <MapPin className="h-3.5 w-3.5" />
              {event.location}
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
      <div className="hidden items-center pr-4 sm:flex">
        <ArrowRight className="h-5 w-5 text-gray-300 transition-colors group-hover:text-brand-500" />
      </div>
    </Link>
  );
}
