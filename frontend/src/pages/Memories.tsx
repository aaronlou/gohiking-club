import { Link } from "react-router-dom";
import { User, Users, CalendarDays, Camera, MessageSquare, ArrowRight, LogIn } from "lucide-react";
import { useAuth } from "@/hooks/useAuth";

const statCards = [
  { key: "team_count" as const, label: "加入的团队", icon: Users, color: "bg-sky-100 text-sky-700" },
  { key: "event_count" as const, label: "参加的活动", icon: CalendarDays, color: "bg-amber-100 text-amber-700" },
  { key: "photo_count" as const, label: "上传的照片", icon: Camera, color: "bg-forest-100 text-forest-700" },
  { key: "review_count" as const, label: "发表的感想", icon: MessageSquare, color: "bg-earth-100 text-earth-700" },
] as const;

const quickLinks = [
  { to: "/teams", label: "我的团队", desc: "查看你加入的徒步团队", icon: Users, color: "bg-sky-100 text-sky-700" },
  { to: "/events", label: "我的活动", desc: "回顾你参加的徒步活动", icon: CalendarDays, color: "bg-amber-100 text-amber-700" },
  { to: "/gallery", label: "我的照片", desc: "浏览你上传的徒步影像", icon: Camera, color: "bg-forest-100 text-forest-700" },
];

export default function Memories() {
  const user = useAuth((s) => s.user);

  if (!user) {
    return (
      <div className="mx-auto max-w-lg text-center py-12 sm:py-16 animate-fade-in">
        <div className="mb-6 inline-flex h-28 w-28 items-center justify-center rounded-full bg-clay-100 border-2 border-clay-200">
          <User className="h-14 w-14 text-clay-400" />
        </div>
        <h1 className="font-display text-3xl font-semibold text-clay-900">
          我的回忆
        </h1>
        <p className="mt-3 text-clay-500 max-w-sm mx-auto leading-relaxed">
          登录后查看你的徒步足迹：加入的团队、参加的活动、上传的照片和发表的感想。
        </p>
        <div className="mt-8 flex justify-center gap-3">
          <Link to="/login" className="btn-primary inline-flex items-center gap-2">
            <LogIn className="h-4 w-4" />
            登录
          </Link>
        </div>
      </div>
    );
  }

  const initial = user.username.charAt(0).toUpperCase();
  const joinDate = new Date(user.created_at).toLocaleDateString("zh-CN", {
    year: "numeric",
    month: "long",
    day: "numeric",
  });

  return (
    <div className="animate-fade-in">
      {/* Header */}
      <div className="rounded-2xl border border-clay-200 bg-white p-6 sm:p-8 shadow-sm mb-8">
        <div className="flex flex-col items-center sm:flex-row sm:gap-6 text-center sm:text-left">
          <div className="mb-4 sm:mb-0 inline-flex h-24 w-24 items-center justify-center rounded-full bg-forest-100 border-2 border-forest-200 shrink-0">
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
          <div>
            <h1 className="font-display text-2xl font-semibold text-clay-900">
              我的回忆
            </h1>
            <p className="mt-1 text-clay-500">
              {user.username}，感谢你的每一次出发
            </p>
            <p className="text-sm text-clay-400 mt-1">
              {joinDate} 加入 GoHiking
            </p>
          </div>
        </div>
      </div>

      {/* Stats grid */}
      <div className="grid grid-cols-2 sm:grid-cols-4 gap-4 mb-8">
        {statCards.map(({ key, label, icon: Icon, color }) => (
          <div
            key={key}
            className="rounded-2xl border border-clay-200 bg-white p-5 shadow-sm text-center animate-slide-up"
          >
            <div className={`mb-3 inline-flex h-12 w-12 items-center justify-center rounded-xl ${color}`}>
              <Icon className="h-6 w-6" />
            </div>
            <div className="font-display text-3xl font-semibold text-clay-900">
              {user[key]}
            </div>
            <div className="text-sm text-clay-500 mt-1">{label}</div>
          </div>
        ))}
      </div>

      {/* Quick links */}
      <div className="mb-8">
        <h2 className="font-display text-lg font-semibold text-clay-900 mb-4">
          快速入口
        </h2>
        <div className="space-y-3">
          {quickLinks.map(({ to, label, desc, icon: Icon, color }) => (
            <Link
              key={to}
              to={to}
              className="flex items-center gap-4 rounded-2xl border border-clay-200 bg-white p-5 shadow-sm hover:shadow-md hover:-translate-y-0.5 transition-all"
            >
              <div className={`flex h-12 w-12 items-center justify-center rounded-xl ${color} shrink-0`}>
                <Icon className="h-6 w-6" />
              </div>
              <div className="flex-1 min-w-0">
                <div className="font-medium text-clay-900">{label}</div>
                <div className="text-sm text-clay-500">{desc}</div>
              </div>
              <ArrowRight className="h-4 w-4 text-clay-300 shrink-0" />
            </Link>
          ))}
        </div>
      </div>
    </div>
  );
}
