import { Link } from "react-router-dom";
import { User, Camera, CalendarDays, LogOut } from "lucide-react";
import { useAuth } from "@/hooks/useAuth";

export default function Profile() {
  const user = useAuth((s) => s.user);
  const logout = useAuth((s) => s.logout);

  if (!user) {
    return (
      <div className="mx-auto max-w-lg text-center py-8 sm:py-12">
        <div className="mb-6 inline-flex h-28 w-28 items-center justify-center rounded-full bg-clay-100 border-2 border-clay-200">
          <User className="h-14 w-14 text-clay-400" />
        </div>
        <h1 className="font-display text-3xl font-semibold text-clay-900">
          个人中心
        </h1>
        <p className="mt-3 text-clay-500 max-w-sm mx-auto leading-relaxed">
          登录后可查看你的徒步照片和活动记录。
        </p>
        <div className="mt-8 flex justify-center gap-3">
          <Link to="/login" className="btn-primary">
            登录
          </Link>
          <Link to="/register" className="btn-secondary">
            注册
          </Link>
        </div>
      </div>
    );
  }

  const initial = user.username.charAt(0).toUpperCase();

  return (
    <div className="mx-auto max-w-lg py-8 sm:py-12">
      {/* Profile card */}
      <div className="rounded-2xl border border-clay-200 bg-white p-6 sm:p-8 shadow-sm text-center">
        <div className="mb-4 inline-flex h-24 w-24 items-center justify-center rounded-full bg-forest-100 border-2 border-forest-200">
          {user.avatar_url ? (
            <img
              src={user.avatar_url}
              alt={user.username}
              className="h-full w-full rounded-full object-cover"
            />
          ) : (
            <span className="font-display text-3xl font-semibold text-forest-700">
              {initial}
            </span>
          )}
        </div>

        <h1 className="font-display text-2xl font-semibold text-clay-900">
          {user.username}
        </h1>
        <p className="mt-1 text-sm text-clay-500">{user.email}</p>
        {user.bio && (
          <p className="mt-3 text-sm text-clay-600 max-w-sm mx-auto leading-relaxed">
            {user.bio}
          </p>
        )}

        <div className="mt-6 flex justify-center gap-6">
          <div className="text-center">
            <div className="font-display text-2xl font-semibold text-clay-900">
              {user.photo_count}
            </div>
            <div className="text-xs text-clay-500 mt-0.5">照片</div>
          </div>
          <div className="text-center">
            <div className="font-display text-2xl font-semibold text-clay-900">
              {new Date(user.created_at).toLocaleDateString("zh-CN")}
            </div>
            <div className="text-xs text-clay-500 mt-0.5">加入日期</div>
          </div>
        </div>
      </div>

      {/* Quick links */}
      <div className="mt-6 grid gap-3">
        <Link
          to="/upload"
          className="flex items-center gap-4 rounded-2xl border border-clay-200 bg-white p-4 shadow-sm hover:shadow-md transition-all hover:-translate-y-0.5"
        >
          <div className="flex h-12 w-12 items-center justify-center rounded-xl bg-earth-100 text-earth-700">
            <Camera className="h-6 w-6" />
          </div>
          <div className="text-left">
            <div className="font-medium text-clay-900">上传照片</div>
            <div className="text-sm text-clay-500">
              分享你的徒步精彩瞬间
            </div>
          </div>
        </Link>

        <Link
          to="/events"
          className="flex items-center gap-4 rounded-2xl border border-clay-200 bg-white p-4 shadow-sm hover:shadow-md transition-all hover:-translate-y-0.5"
        >
          <div className="flex h-12 w-12 items-center justify-center rounded-xl bg-forest-100 text-forest-700">
            <CalendarDays className="h-6 w-6" />
          </div>
          <div className="text-left">
            <div className="font-medium text-clay-900">浏览活动</div>
            <div className="text-sm text-clay-500">
              发现附近的徒步活动
            </div>
          </div>
        </Link>
      </div>

      {/* Logout */}
      <div className="mt-6 text-center">
        <button
          onClick={logout}
          className="inline-flex items-center gap-2 text-sm text-clay-400 hover:text-red-600 transition-colors"
        >
          <LogOut className="h-4 w-4" />
          退出登录
        </button>
      </div>
    </div>
  );
}
