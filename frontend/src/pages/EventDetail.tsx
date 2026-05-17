import { useState } from "react";
import { useParams, Link, useNavigate } from "react-router-dom";
import {
  Calendar,
  MapPin,
  Users,
  Camera,
  ArrowLeft,
  Loader2,
  UserPlus,
  LogIn,
  MessageSquare,
  Star,
  Send,
  TrendingUp,
  Mountain,
  FileText,
  Lock,
} from "lucide-react";
import { useEvent, useEventPhotos, useJoinEvent, useEventReviews, useCreateEventReview } from "@/hooks/useEvents";
import { useAuth } from "@/hooks/useAuth";
import { PhotoGrid } from "@/components/PhotoGrid";

export default function EventDetail() {
  const { user } = useAuth();
  const navigate = useNavigate();
  const { id } = useParams<{ id: string }>();
  const { data: event, isLoading } = useEvent(id!);
  const { data: photos = [], isLoading: photosLoading } = useEventPhotos(id!);
  const { data: reviews = [], isLoading: reviewsLoading } = useEventReviews(id!);
  const joinMutation = useJoinEvent();
  const reviewMutation = useCreateEventReview();

  const [reviewContent, setReviewContent] = useState("");
  const [reviewRating, setReviewRating] = useState(5);

  const handleJoin = () => {
    if (!user) {
      navigate("/login");
      return;
    }
    joinMutation.mutate(event!.id, {
      onError: () => navigate("/login"),
    });
  };

  const handleSubmitReview = (e: React.FormEvent) => {
    e.preventDefault();
    if (!reviewContent.trim() || !user) return;
    reviewMutation.mutate(
      { eventId: event!.id, content: reviewContent, rating: reviewRating },
      {
        onSuccess: () => {
          setReviewContent("");
          setReviewRating(5);
        },
      }
    );
  };

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
            <img src={event.cover_url} alt={event.title} className="h-full w-full object-cover" />
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
            {event.distance_km && (
              <span className="inline-flex items-center gap-1.5">
                <TrendingUp className="h-4 w-4" />
                {event.distance_km} km
              </span>
            )}
            {event.elevation_gain_m && (
              <span className="inline-flex items-center gap-1.5">
                <Mountain className="h-4 w-4" />
                爬升 {event.elevation_gain_m} m
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
            <span className="inline-flex items-center gap-1.5">
              <MessageSquare className="h-4 w-4" />
              {event.review_count} 条感想
            </span>
          </div>

          {event.description && (
            <p className="text-clay-600 leading-relaxed mb-6 max-w-2xl">
              {event.description}
            </p>
          )}

          <div className="flex flex-col gap-3 sm:flex-row">
            {event.team_id && !event.is_team_member ? (
              <div className="inline-flex items-center gap-2 rounded-xl bg-clay-100 px-4 py-2.5 text-sm text-clay-500">
                <Lock className="h-4 w-4" />
                仅团队成员可报名
              </div>
            ) : (
              <button
                onClick={handleJoin}
                className="btn-primary"
                disabled={joinMutation.isPending}
              >
                {user ? (
                  <UserPlus className="h-4 w-4" />
                ) : (
                  <LogIn className="h-4 w-4" />
                )}
                {user
                  ? joinMutation.isPending ? "加入中..." : "加入活动"
                  : "登录后加入"
                }
              </button>
            )}
            <Link to={`/upload?event_id=${event.id}`} className="btn-secondary">
              <Camera className="h-4 w-4" />
              上传照片到本活动
            </Link>
          </div>
        </div>
      </div>

      {/* Disclaimer */}
      {event.disclaimer && (
        <div className="mb-6 rounded-2xl border border-amber-200 bg-amber-50 p-5">
          <div className="flex items-center gap-2 mb-2">
            <FileText className="h-4 w-4 text-amber-700" />
            <h3 className="text-sm font-semibold text-amber-800">免责声明</h3>
          </div>
          <p className="text-sm text-amber-700 leading-relaxed whitespace-pre-line">{event.disclaimer}</p>
        </div>
      )}

      {/* Photos section */}
      <div className="flex items-center gap-3 mb-5">
        <h2 className="font-display text-xl font-semibold text-clay-900">
          活动照片
        </h2>
        <span className="h-px flex-1 bg-clay-200" />
      </div>
      <PhotoGrid photos={photos} isLoading={photosLoading} />

      {/* Reviews section */}
      <div className="mt-12">
        <div className="flex items-center gap-3 mb-5">
          <h2 className="font-display text-xl font-semibold text-clay-900">
            活动感想
          </h2>
          <span className="h-px flex-1 bg-clay-200" />
        </div>

        {/* Write review */}
        {user && (
          <form onSubmit={handleSubmitReview} className="mb-6 rounded-2xl border border-clay-200 bg-white p-5 shadow-sm">
            <div className="flex items-center gap-1 mb-3">
              {[1, 2, 3, 4, 5].map((n) => (
                <button
                  key={n}
                  type="button"
                  onClick={() => setReviewRating(n)}
                  className="p-0.5 transition-colors"
                >
                  <Star
                    className={`h-5 w-5 ${n <= reviewRating ? "fill-amber-400 text-amber-400" : "text-clay-300"}`}
                  />
                </button>
              ))}
              <span className="ml-2 text-sm text-clay-500">{reviewRating} 星</span>
            </div>
            <textarea
              value={reviewContent}
              onChange={(e) => setReviewContent(e.target.value)}
              placeholder="分享你的徒步感受..."
              rows={3}
              className="input-field mb-3"
              required
            />
            <div className="flex justify-end">
              <button
                type="submit"
                disabled={!reviewContent.trim() || reviewMutation.isPending}
                className="btn-primary px-5 py-2 text-sm"
              >
                <Send className="h-3.5 w-3.5" />
                {reviewMutation.isPending ? "发布中..." : "发布感想"}
              </button>
            </div>
          </form>
        )}

        {/* Review list */}
        {reviewsLoading ? (
          <div className="flex justify-center py-8">
            <Loader2 className="h-6 w-6 animate-spin text-forest-600" />
          </div>
        ) : reviews.length === 0 ? (
          <div className="rounded-2xl border border-clay-200 bg-white p-8 text-center">
            <MessageSquare className="mx-auto h-10 w-10 text-clay-300 mb-3" />
            <p className="text-clay-500">还没有人分享感想</p>
            {!user && (
              <p className="mt-1 text-sm text-clay-400">
                登录并加入活动后可以写感想
              </p>
            )}
          </div>
        ) : (
          <div className="space-y-4">
            {reviews.map((review) => (
              <div key={review.id} className="rounded-2xl border border-clay-200 bg-white p-5 shadow-sm">
                <div className="flex items-start justify-between gap-3">
                  <div className="flex items-center gap-3">
                    <div className="h-9 w-9 rounded-full bg-forest-100 flex items-center justify-center text-forest-700 text-sm font-medium">
                      {review.avatar_url ? (
                        <img src={review.avatar_url} alt={review.username} className="h-9 w-9 rounded-full object-cover" />
                      ) : (
                        review.username.charAt(0).toUpperCase()
                      )}
                    </div>
                    <div>
                      <p className="text-sm font-medium text-clay-900">{review.username}</p>
                      <div className="flex items-center gap-1">
                        {review.rating && Array.from({ length: 5 }).map((_, i) => (
                          <Star
                            key={i}
                            className={`h-3 w-3 ${i < review.rating! ? "fill-amber-400 text-amber-400" : "text-clay-200"}`}
                          />
                        ))}
                        <span className="ml-1 text-xs text-clay-400">
                          {new Date(review.created_at).toLocaleDateString("zh-CN")}
                        </span>
                      </div>
                    </div>
                  </div>
                </div>
                <p className="mt-3 text-sm text-clay-700 leading-relaxed">
                  {review.content}
                </p>
              </div>
            ))}
          </div>
        )}
      </div>
    </div>
  );
}
