import { useParams, Link } from "react-router-dom";
import {
  Calendar,
  MapPin,
  Users,
  Camera,
  ArrowLeft,
  Loader2,
  UserPlus,
} from "lucide-react";
import { useEvent, useEventPhotos, useJoinEvent } from "@/hooks/useEvents";
import { PhotoGrid } from "@/components/PhotoGrid";

export default function EventDetail() {
  const { id } = useParams<{ id: string }>();
  const { data: event, isLoading } = useEvent(id!);
  const { data: photos = [], isLoading: photosLoading } = useEventPhotos(id!);
  const joinMutation = useJoinEvent();

  if (isLoading) {
    return (
      <div className="flex justify-center py-20">
        <Loader2 className="h-8 w-8 animate-spin text-forest-600" />
      </div>
    );
  }

  if (!event) {
    return (
      <div className="py-20 text-center animate-fade-in">
        <p className="font-display text-xl text-clay-500">活动不存在</p>
        <Link
          to="/events"
          className="mt-4 inline-flex items-center gap-1.5 text-sm text-forest-600 hover:text-forest-700 transition-colors"
        >
          <ArrowLeft className="h-4 w-4" />
          返回活动列表
        </Link>
      </div>
    );
  }

  const formatDate = (d: string | null) => {
    if (!d) return "待定";
    try {
      return new Date(d).toLocaleDateString("zh-CN", {
        year: "numeric",
        month: "long",
        day: "numeric",
      });
    } catch {
      return d;
    }
  };

  return (
    <div className="animate-fade-in">
      <Link
        to="/events"
        className="mb-5 inline-flex items-center gap-1.5 text-sm text-clay-500 hover:text-clay-700 transition-colors"
      >
        <ArrowLeft className="h-4 w-4" />
        返回活动列表
      </Link>

      {/* Event hero */}
      <div className="overflow-hidden rounded-2xl border border-clay-200 bg-white shadow-sm mb-6">
        {event.cover_url ? (
          <div className="aspect-[3/1] overflow-hidden bg-clay-100 sm:aspect-[4/1]">
            <img
              src={event.cover_url}
              alt={event.title}
              className="h-full w-full object-cover"
            />
          </div>
        ) : (
          <div className="aspect-[3/1] bg-gradient-to-br from-forest-100 via-cream-50 to-earth-100 sm:aspect-[4/1] flex items-center justify-center">
            <span className="font-display text-5xl text-forest-200">
              {event.title.charAt(0)}
            </span>
          </div>
        )}

        <div className="p-5 sm:p-7">
          <h1 className="font-display text-2xl sm:text-3xl font-semibold text-clay-900 mb-3">
            {event.title}
          </h1>

          <div className="flex flex-wrap gap-3 sm:gap-5 text-sm text-clay-500 mb-4">
            {event.date && (
              <span className="inline-flex items-center gap-1.5">
                <Calendar className="h-4 w-4" />
                {formatDate(event.date)}
              </span>
            )}
            {event.location && (
              <span className="inline-flex items-center gap-1.5">
                <MapPin className="h-4 w-4" />
                {event.location}
              </span>
            )}
            <span className="inline-flex items-center gap-1.5">
              <Users className="h-4 w-4" />
              {event.member_count} 人参与
            </span>
            <span className="inline-flex items-center gap-1.5">
              <Camera className="h-4 w-4" />
              {event.photo_count} 张照片
            </span>
          </div>

          {event.description && (
            <p className="text-clay-600 leading-relaxed mb-6 max-w-2xl">
              {event.description}
            </p>
          )}

          <div className="flex flex-col gap-3 sm:flex-row">
            <button
              onClick={() => joinMutation.mutate(event.id)}
              className="btn-primary"
              disabled={joinMutation.isPending}
            >
              <UserPlus className="h-4 w-4" />
              {joinMutation.isPending ? "加入中..." : "加入活动"}
            </button>
            <Link
              to={`/upload?event_id=${event.id}`}
              className="btn-secondary"
            >
              <Camera className="h-4 w-4" />
              上传照片到本活动
            </Link>
          </div>
        </div>
      </div>

      {/* Photos section */}
      <div className="flex items-center gap-3 mb-5">
        <h2 className="font-display text-xl font-semibold text-clay-900">
          活动照片
        </h2>
        <span className="h-px flex-1 bg-clay-200" />
      </div>

      <PhotoGrid photos={photos} isLoading={photosLoading} />
    </div>
  );
}
