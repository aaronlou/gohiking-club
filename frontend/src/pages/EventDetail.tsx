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
        <Loader2 className="h-8 w-8 animate-spin text-brand-600" />
      </div>
    );
  }

  if (!event) {
    return (
      <div className="py-20 text-center">
        <p className="text-gray-500">活动不存在</p>
        <Link to="/events" className="mt-4 inline-block text-brand-600 hover:underline">
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
    <div>
      <Link
        to="/events"
        className="mb-4 inline-flex items-center gap-1 text-sm text-gray-500 hover:text-gray-700"
      >
        <ArrowLeft className="h-4 w-4" />
        返回活动列表
      </Link>

      <div className="card mb-6 overflow-hidden">
        {event.cover_url && (
          <div className="aspect-[3/1] overflow-hidden bg-gray-100 sm:aspect-[4/1]">
            <img
              src={event.cover_url}
              alt={event.title}
              className="h-full w-full object-cover"
            />
          </div>
        )}

        <div className="p-4 sm:p-6">
          <h1 className="mb-3 text-2xl font-bold text-gray-900">
            {event.title}
          </h1>

          <div className="mb-4 flex flex-wrap gap-3 text-sm text-gray-500">
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
            <p className="mb-6 text-gray-600">{event.description}</p>
          )}

          <div className="flex flex-col gap-2 sm:flex-row">
            <button
              onClick={() => joinMutation.mutate(event.id)}
              className="btn-primary gap-2"
              disabled={joinMutation.isPending}
            >
              <UserPlus className="h-4 w-4" />
              {joinMutation.isPending ? "加入中..." : "加入活动"}
            </button>
            <Link
              to={`/upload?event_id=${event.id}`}
              className="btn-secondary gap-2"
            >
              <Camera className="h-4 w-4" />
              上传照片到本活动
            </Link>
          </div>
        </div>
      </div>

      <h2 className="mb-4 text-lg font-bold text-gray-900">活动照片</h2>
      <PhotoGrid photos={photos} isLoading={photosLoading} />
    </div>
  );
}
