import { useState } from "react";
import { useParams, Link, useNavigate } from "react-router-dom";
import {
  ArrowLeft,
  Loader2,
  LogIn,
  Users,
  Mountain,
  CalendarDays,
  Send,
  CheckCircle,
} from "lucide-react";
import { useInvitationByCode, useApplyJoinTeam } from "@/hooks/useTeams";
import { useAuth } from "@/hooks/useAuth";

export default function TeamJoin() {
  const { code } = useParams<{ code: string }>();
  const navigate = useNavigate();
  const { user } = useAuth();
  const { data, isLoading, error } = useInvitationByCode(code!);
  const applyMutation = useApplyJoinTeam();
  const [message, setMessage] = useState("");
  const [applied, setApplied] = useState(false);

  const handleApply = async () => {
    if (!code) return;
    await applyMutation.mutateAsync({ code, message: message.trim() || undefined });
    setApplied(true);
  };

  if (isLoading) {
    return (
      <div className="flex justify-center py-20">
        <Loader2 className="h-8 w-8 animate-spin text-forest-600" />
      </div>
    );
  }

  if (error || !data) {
    return (
      <div className="mx-auto max-w-lg px-4 text-center py-16 sm:py-24 animate-fade-in">
        <div className="mb-6 inline-flex h-24 w-24 items-center justify-center rounded-full bg-clay-100">
          <Mountain className="h-12 w-12 text-clay-500" />
        </div>
        <h2 className="font-display text-2xl font-semibold text-clay-900 mb-2">
          邀请链接无效
        </h2>
        <p className="text-clay-500 mb-8">
          该邀请链接已过期、达到使用上限或不存在
        </p>
        <Link to="/teams" className="btn-primary inline-flex items-center gap-2">
          <ArrowLeft className="h-4 w-4" />
          返回团队列表
        </Link>
      </div>
    );
  }

  const { team } = data;

  if (applied) {
    return (
      <div className="mx-auto max-w-lg px-4 text-center py-16 sm:py-24 animate-fade-in">
        <div className="mb-6 inline-flex h-24 w-24 items-center justify-center rounded-full bg-forest-100">
          <CheckCircle className="h-12 w-12 text-forest-600" />
        </div>
        <h2 className="font-display text-2xl font-semibold text-clay-900 mb-2">
          申请已提交
        </h2>
        <p className="text-clay-500 mb-8">
          请等待团队管理员审批，审批通过后将自动加入团队
        </p>
        <Link to={`/teams/${team.id}`} className="btn-primary inline-flex items-center gap-2">
          <ArrowLeft className="h-4 w-4" />
          查看团队
        </Link>
      </div>
    );
  }

  return (
    <div className="mx-auto max-w-xl animate-fade-in">
      <Link
        to="/teams"
        className="mb-5 inline-flex items-center gap-1.5 text-sm text-clay-500 hover:text-clay-700 transition-colors"
      >
        <ArrowLeft className="h-4 w-4" />
        返回团队列表
      </Link>

      <div className="overflow-hidden rounded-2xl border border-clay-200 bg-white shadow-sm mb-6">
        <div className="aspect-[3/1] bg-gradient-to-br from-forest-100 via-cream-50 to-earth-100 flex items-center justify-center">
          <Mountain className="h-12 w-12 text-forest-200" />
        </div>
        <div className="p-6">
          <h1 className="font-display text-2xl font-semibold text-clay-900 mb-2">
            申请加入 {team.name}
          </h1>
          <div className="flex flex-wrap gap-3 text-sm text-clay-500 mb-4">
            <span className="inline-flex items-center gap-1.5">
              <Users className="h-4 w-4" />
              {team.member_count} 成员
            </span>
            <span className="inline-flex items-center gap-1.5">
              <CalendarDays className="h-4 w-4" />
              {team.event_count} 活动
            </span>
          </div>
          {team.description && (
            <p className="text-clay-600 leading-relaxed text-sm">{team.description}</p>
          )}
        </div>
      </div>

      {!user ? (
        <div className="rounded-2xl border border-clay-200 bg-white p-6 text-center">
          <LogIn className="mx-auto h-8 w-8 text-clay-400 mb-3" />
          <p className="text-clay-600 mb-4">请先登录后再申请加入团队</p>
          <button
            onClick={() => navigate("/login", { state: { from: `/teams/join/${code}` } })}
            className="btn-primary inline-flex items-center gap-2"
          >
            <LogIn className="h-4 w-4" />
            去登录
          </button>
        </div>
      ) : (
        <div className="rounded-2xl border border-clay-200 bg-white p-6 shadow-sm space-y-4">
          <div>
            <label className="mb-1.5 block text-sm font-medium text-clay-700">
              申请留言（可选）
            </label>
            <textarea
              value={message}
              onChange={(e) => setMessage(e.target.value)}
              placeholder="介绍一下自己，让管理员更快了解你..."
              rows={4}
              className="input-field"
            />
          </div>
          <button
            onClick={handleApply}
            disabled={applyMutation.isPending}
            className="btn-primary w-full justify-center"
          >
            {applyMutation.isPending ? (
              <Loader2 className="h-4 w-4 animate-spin" />
            ) : (
              <>
                <Send className="h-4 w-4" />
                提交申请
              </>
            )}
          </button>
        </div>
      )}
    </div>
  );
}
