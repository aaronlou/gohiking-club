import { Link } from "react-router-dom";
import { CalendarDays, Plus, Loader2 } from "lucide-react";
import { useEvents } from "@/hooks/useEvents";
import { EventCard } from "@/components/EventCard";

export default function Events() {
  const { data: events = [], isLoading } = useEvents({ limit: 50 });

  return (
    <div>
      <div className="mb-6 flex items-center justify-between">
        <div>
          <h1 className="text-2xl font-bold text-gray-900">徒步活动</h1>
          <p className="mt-1 text-sm text-gray-500">创建或加入活动，一起分享徒步照片</p>
        </div>
        <Link to="/events/new" className="btn-primary gap-2">
          <Plus className="h-4 w-4" />
          <span className="hidden sm:inline">创建活动</span>
        </Link>
      </div>

      {isLoading ? (
        <div className="flex justify-center py-20">
          <Loader2 className="h-8 w-8 animate-spin text-brand-600" />
        </div>
      ) : events.length === 0 ? (
        <div className="flex flex-col items-center justify-center py-20 text-gray-400">
          <CalendarDays className="mb-4 h-16 w-16" />
          <p className="text-lg font-medium">还没有活动</p>
          <p className="mt-1 mb-6 text-sm">创建一个新活动，开始组织徒步吧！</p>
          <Link to="/events/new" className="btn-primary gap-2">
            <Plus className="h-4 w-4" />
            创建第一个活动
          </Link>
        </div>
      ) : (
        <div className="grid gap-4">
          {events.map((event) => (
            <EventCard key={event.id} event={event} />
          ))}
        </div>
      )}
    </div>
  );
}
