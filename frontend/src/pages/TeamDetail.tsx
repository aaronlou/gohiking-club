import { useParams, Link, useNavigate } from "react-router-dom";
import {
  ArrowLeft,
  Loader2,
  Users,
  Mountain,
  CalendarDays,
  UserPlus,
  LogIn,
  ArrowRight,
} from "lucide-react";
import { useTeam, useTeamMembers, useTeamEvents, useJoinTeam, useLeaveTeam } from "@/hooks/useTeams";
import { useAuth } from "@/hooks/useAuth";
import { EventCard } from "@/components/EventCard";

export default function TeamDetail() {
  const { user } = useAuth();
  const navigate = useNavigate();
  const { id } = useParams<{ id: string }>();
  const { data: team, isLoading } = useTeam(id!);
  const { data: members = [] } = useTeamMembers(id!);
  const { data: events = [] } = useTeamEvents(id!);
  const joinMutation = useJoinTeam();
  const leaveMutation = useLeaveTeam();

  const isMember = user ? members.some((m) => m.user_id === user.id) : false;

  const handleJoin = () => {
    if (!user) {
      navigate("/login");
      return;
    }
    joinMutation.mutate(team!.id);
  };

  const handleLeave = () => {
    leaveMutation.mutate(team!.id);
  };

  if (isLoading) {
    return (
      <div className="flex justify-center py-20">
        <Loader2 className="h-8 w-8 animate-spin text-forest-600" />
      </div>
    );
  }

  if (!team) {
    return (
      <div className="py-20 text-center animate-fade-in">
        <p className="font-display text-xl text-clay-500">团队不存在</p>
        <Link
          to="/teams"
          className="mt-4 inline-flex items-center gap-1.5 text-sm text-forest-600 hover:text-forest-700 transition-colors"
        >
          <ArrowLeft className="h-4 w-4" />
          返回团队列表
        </Link>
      </div>
    );
  }

  return (
    <div className="animate-fade-in">
      <Link
        to="/teams"
        className="mb-5 inline-flex items-center gap-1.5 text-sm text-clay-500 hover:text-clay-700 transition-colors"
      >
        <ArrowLeft className="h-4 w-4" />
        返回团队列表
      </Link>

      {/* Team hero */}
      <div className="overflow-hidden rounded-2xl border border-clay-200 bg-white shadow-sm mb-6">
        {team.cover_url ? (
          <div className="aspect-[3/1] overflow-hidden bg-clay-100 sm:aspect-[4/1]">
            <img src={team.cover_url} alt={team.name} className="h-full w-full object-cover" />
          </div>
        ) : (
          <div className="aspect-[3/1] bg-gradient-to-br from-forest-100 via-cream-50 to-earth-100 sm:aspect-[4/1] flex items-center justify-center">
            <Mountain className="h-16 w-16 text-forest-200" />
          </div>
        )}

        <div className="p-5 sm:p-7">
          <div className="flex items-start justify-between gap-4">
            <div>
              <h1 className="font-display text-2xl sm:text-3xl font-semibold text-clay-900 mb-2">
                {team.name}
              </h1>
              <div className="flex flex-wrap gap-3 text-sm text-clay-500">
                <span className="inline-flex items-center gap-1.5">
                  <Users className="h-4 w-4" />
                  {team.member_count} 成员
                </span>
                <span className="inline-flex items-center gap-1.5">
                  <CalendarDays className="h-4 w-4" />
                  {team.event_count} 活动
                </span>
              </div>
            </div>

            {isMember ? (
              <button
                onClick={handleLeave}
                disabled={leaveMutation.isPending}
                className="btn-secondary shrink-0"
              >
                {leaveMutation.isPending ? "处理中..." : "退出团队"}
              </button>
            ) : (
              <button
                onClick={handleJoin}
                disabled={joinMutation.isPending}
                className="btn-primary shrink-0"
              >
                {user ? (
                  <UserPlus className="h-4 w-4" />
                ) : (
                  <LogIn className="h-4 w-4" />
                )}
                {user ? (joinMutation.isPending ? "加入中..." : "加入团队") : "登录后加入"}
              </button>
            )}
          </div>

          {team.description && (
            <p className="mt-4 text-clay-600 leading-relaxed max-w-2xl">
              {team.description}
            </p>
          )}
        </div>
      </div>

      <div className="grid gap-6 lg:grid-cols-3">
        {/* Members */}
        <div className="lg:col-span-1">
          <div className="flex items-center gap-3 mb-4">
            <h2 className="font-display text-lg font-semibold text-clay-900">
              团队成员
            </h2>
            <span className="h-px flex-1 bg-clay-200" />
          </div>
          <div className="rounded-2xl border border-clay-200 bg-white shadow-sm divide-y divide-clay-100">
            {members.map((member) => (
              <div key={member.user_id} className="flex items-center gap-3 p-4">
                <div className="h-9 w-9 rounded-full bg-forest-100 flex items-center justify-center text-forest-700 text-sm font-medium">
                  {member.avatar_url ? (
                    <img src={member.avatar_url} alt={member.username} className="h-9 w-9 rounded-full object-cover" />
                  ) : (
                    member.username.charAt(0).toUpperCase()
                  )}
                </div>
                <div className="flex-1 min-w-0">
                  <p className="text-sm font-medium text-clay-900 truncate">{member.username}</p>
                  {member.role === "admin" && (
                    <span className="text-xs text-forest-600">管理员</span>
                  )}
                </div>
              </div>
            ))}
            {members.length === 0 && (
              <div className="p-6 text-center text-sm text-clay-400">
                暂无成员
              </div>
            )}
          </div>
        </div>

        {/* Events */}
        <div className="lg:col-span-2">
          <div className="flex items-center gap-3 mb-4">
            <h2 className="font-display text-lg font-semibold text-clay-900">
              团队活动
            </h2>
            <span className="h-px flex-1 bg-clay-200" />
            <Link
              to="/events/new"
              className="text-sm font-medium text-forest-600 hover:text-forest-700 transition-colors inline-flex items-center gap-1"
            >
              发起活动
              <ArrowRight className="h-3.5 w-3.5" />
            </Link>
          </div>

          {events.length === 0 ? (
            <div className="rounded-2xl border border-clay-200 bg-white p-8 text-center">
              <CalendarDays className="mx-auto h-10 w-10 text-clay-300 mb-3" />
              <p className="text-clay-500">该团队还没有发起活动</p>
            </div>
          ) : (
            <div className="grid gap-4">
              {events.map((event) => (
                <EventCard key={event.id} event={event} />
              ))}
            </div>
          )}
        </div>
      </div>
    </div>
  );
}
