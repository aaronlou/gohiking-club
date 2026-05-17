import { useState } from "react";
import { useParams, Link, useNavigate } from "react-router-dom";
import {
  ArrowLeft,
  Loader2,
  Users,
  Mountain,
  CalendarDays,
  UserPlus,
  LogIn,
  Link2,
  ClipboardCopy,
  CheckCircle,
  XCircle,
  Shield,
  Clock,
} from "lucide-react";
import {
  useTeam,
  useTeamMembers,
  useTeamEvents,
  useJoinTeam,
  useLeaveTeam,
  useTeamInvitations,
  useCreateTeamInvitation,
  useJoinRequests,
  useApproveJoinRequest,
  useRejectJoinRequest,
} from "@/hooks/useTeams";
import { useAuth } from "@/hooks/useAuth";
import { EventCard } from "@/components/EventCard";

export default function TeamDetail() {
  const { user } = useAuth();
  const navigate = useNavigate();
  const { id } = useParams<{ id: string }>();
  const { data: team, isLoading } = useTeam(id!);
  const { data: members = [] } = useTeamMembers(id!);
  const { data: events = [] } = useTeamEvents(id!);
  const { data: invitations = [] } = useTeamInvitations(id!);
  const { data: joinRequests = [] } = useJoinRequests(id!);
  const joinMutation = useJoinTeam();
  const leaveMutation = useLeaveTeam();
  const createInvitation = useCreateTeamInvitation();
  const approveRequest = useApproveJoinRequest();
  const rejectRequest = useRejectJoinRequest();

  const [copiedCode, setCopiedCode] = useState<string | null>(null);
  const [activeTab, setActiveTab] = useState<"events" | "members" | "settings">("events");

  const isMember = user ? members.some((m) => m.user_id === user.id) : false;
  const isAdmin = user ? members.some((m) => m.user_id === user.id && m.role === "admin") : false;

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

  const handleCreateInvitation = () => {
    if (!id) return;
    createInvitation.mutate({ teamId: id });
  };

  const handleCopyLink = (code: string) => {
    const url = `${window.location.origin}/teams/join/${code}`;
    navigator.clipboard.writeText(url);
    setCopiedCode(code);
    setTimeout(() => setCopiedCode(null), 2000);
  };

  const handleApprove = (requestId: string) => {
    if (!id) return;
    approveRequest.mutate({ teamId: id, requestId });
  };

  const handleReject = (requestId: string) => {
    if (!id) return;
    rejectRequest.mutate({ teamId: id, requestId });
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

      {/* Tabs */}
      <div className="flex gap-1 mb-6 border-b border-clay-200">
        {[
          { key: "events" as const, label: "团队活动" },
          { key: "members" as const, label: "成员" },
          ...(isAdmin ? [{ key: "settings" as const, label: "管理" }] : []),
        ].map((tab) => (
          <button
            key={tab.key}
            onClick={() => setActiveTab(tab.key)}
            className={`px-4 py-2.5 text-sm font-medium transition-colors border-b-2 -mb-px ${
              activeTab === tab.key
                ? "border-forest-500 text-forest-700"
                : "border-transparent text-clay-500 hover:text-clay-700"
            }`}
          >
            {tab.label}
          </button>
        ))}
      </div>

      {/* Events Tab */}
      {activeTab === "events" && (
        <div>
          {isMember && (
            <div className="mb-4 flex justify-end">
              <Link
                to={`/events/new?team_id=${team.id}`}
                className="btn-primary inline-flex items-center gap-1.5 text-sm"
              >
                <CalendarDays className="h-4 w-4" />
                发起活动
              </Link>
            </div>
          )}

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
      )}

      {/* Members Tab */}
      {activeTab === "members" && (
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
            <div className="p-6 text-center text-sm text-clay-400">暂无成员</div>
          )}
        </div>
      )}

      {/* Settings Tab (Admin only) */}
      {activeTab === "settings" && isAdmin && (
        <div className="space-y-8">
          {/* Invitations */}
          <div className="rounded-2xl border border-clay-200 bg-white p-6 shadow-sm">
            <div className="flex items-center justify-between mb-4">
              <h3 className="font-display text-lg font-semibold text-clay-900 flex items-center gap-2">
                <Link2 className="h-5 w-5 text-forest-600" />
                邀请链接
              </h3>
              <button
                onClick={handleCreateInvitation}
                disabled={createInvitation.isPending}
                className="btn-primary text-sm"
              >
                {createInvitation.isPending ? "生成中..." : "生成新链接"}
              </button>
            </div>

            {invitations.length === 0 ? (
              <p className="text-sm text-clay-400">还没有邀请链接</p>
            ) : (
              <div className="space-y-3">
                {invitations.map((inv) => (
                  <div
                    key={inv.id}
                    className="flex items-center justify-between gap-3 rounded-xl border border-clay-100 bg-cream-50 p-3"
                  >
                    <div className="min-w-0 flex-1">
                      <code className="text-xs text-clay-600 truncate block">{inv.code}</code>
                      <div className="flex gap-3 mt-1 text-xs text-clay-400">
                        <span>已用 {inv.used_count}{inv.max_uses ? ` / ${inv.max_uses}` : ""}</span>
                        {inv.expires_at && (
                          <span className="flex items-center gap-0.5">
                            <Clock className="h-3 w-3" />
                            {new Date(inv.expires_at).toLocaleDateString("zh-CN")} 过期
                          </span>
                        )}
                        <span className={inv.status === "active" ? "text-forest-600" : "text-clay-400"}>
                          {inv.status === "active" ? "有效" : "已失效"}
                        </span>
                      </div>
                    </div>
                    <button
                      onClick={() => handleCopyLink(inv.code)}
                      className="shrink-0 inline-flex items-center gap-1 rounded-lg px-3 py-1.5 text-xs font-medium text-forest-700 bg-forest-50 hover:bg-forest-100 transition-colors"
                    >
                      {copiedCode === inv.code ? (
                        <>
                          <CheckCircle className="h-3.5 w-3.5" />
                          已复制
                        </>
                      ) : (
                        <>
                          <ClipboardCopy className="h-3.5 w-3.5" />
                          复制链接
                        </>
                      )}
                    </button>
                  </div>
                ))}
              </div>
            )}
          </div>

          {/* Join Requests */}
          <div className="rounded-2xl border border-clay-200 bg-white p-6 shadow-sm">
            <h3 className="font-display text-lg font-semibold text-clay-900 flex items-center gap-2 mb-4">
              <Shield className="h-5 w-5 text-forest-600" />
              待审批加入申请
            </h3>

            {joinRequests.length === 0 ? (
              <p className="text-sm text-clay-400">暂时没有待审批的申请</p>
            ) : (
              <div className="space-y-3">
                {joinRequests.map((req) => (
                  <div
                    key={req.id}
                    className="flex items-center justify-between gap-3 rounded-xl border border-clay-100 bg-cream-50 p-4"
                  >
                    <div className="flex items-center gap-3 min-w-0 flex-1">
                      <div className="h-9 w-9 rounded-full bg-forest-100 flex items-center justify-center text-forest-700 text-sm font-medium shrink-0">
                        {req.avatar_url ? (
                          <img src={req.avatar_url} alt={req.username} className="h-9 w-9 rounded-full object-cover" />
                        ) : (
                          req.username.charAt(0).toUpperCase()
                        )}
                      </div>
                      <div className="min-w-0">
                        <p className="text-sm font-medium text-clay-900">{req.username}</p>
                        {req.message && (
                          <p className="text-xs text-clay-500 truncate">{req.message}</p>
                        )}
                      </div>
                    </div>
                    <div className="flex gap-2 shrink-0">
                      <button
                        onClick={() => handleApprove(req.id)}
                        disabled={approveRequest.isPending}
                        className="inline-flex items-center gap-1 rounded-lg px-3 py-1.5 text-xs font-medium text-white bg-forest-600 hover:bg-forest-700 transition-colors"
                      >
                        <CheckCircle className="h-3.5 w-3.5" />
                        通过
                      </button>
                      <button
                        onClick={() => handleReject(req.id)}
                        disabled={rejectRequest.isPending}
                        className="inline-flex items-center gap-1 rounded-lg px-3 py-1.5 text-xs font-medium text-clay-700 bg-clay-100 hover:bg-clay-200 transition-colors"
                      >
                        <XCircle className="h-3.5 w-3.5" />
                        拒绝
                      </button>
                    </div>
                  </div>
                ))}
              </div>
            )}
          </div>
        </div>
      )}
    </div>
  );
}
