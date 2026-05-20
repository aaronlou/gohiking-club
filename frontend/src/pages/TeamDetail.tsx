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
  ImagePlus,
  ShieldCheck,
  User,
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
  useUpdateTeam,
  useUpdateMemberRole,
} from "@/hooks/useTeams";
import { useAuth } from "@/hooks/useAuth";
import { EventCard } from "@/components/EventCard";
import { uploadPhoto } from "@/api/client";

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
  const updateTeam = useUpdateTeam();
  const updateMemberRole = useUpdateMemberRole();

  const [copiedCode, setCopiedCode] = useState<string | null>(null);
  const [activeTab, setActiveTab] = useState<"events" | "members" | "settings">("events");
  const [coverUploading, setCoverUploading] = useState(false);
  const [coverLoaded, setCoverLoaded] = useState(false);

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

  const handleCoverUpload = async (e: React.ChangeEvent<HTMLInputElement>) => {
    const file = e.target.files?.[0];
    if (!file || !id) return;
    setCoverUploading(true);
    try {
      const photo = await uploadPhoto(file);
      await updateTeam.mutateAsync({ id, updates: { cover_url: photo.url } });
    } catch {
      // ignore
    } finally {
      setCoverUploading(false);
    }
  };

  const handleRoleChange = (userId: string, newRole: "admin" | "member") => {
    if (!id) return;
    const member = members.find((m) => m.user_id === userId);
    if (!member) return;
    const actionText = newRole === "admin" ? "设为管理员" : "降为普通成员";
    if (!confirm(`确定要将 "${member.username}" ${actionText}吗？`)) return;
    updateMemberRole.mutate({ teamId: id, userId, role: newRole });
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
          <div className="aspect-[3/1] overflow-hidden bg-clay-100 sm:aspect-[4/1] relative">
            {!coverLoaded && (
              <div className="absolute inset-0 animate-pulse bg-clay-200" />
            )}
            <img
              src={team.cover_url}
              alt={team.name}
              className="h-full w-full object-cover"
              loading="lazy"
              onLoad={() => setCoverLoaded(true)}
            />
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
                <div className="flex items-center gap-2">
                  {member.role === "admin" ? (
                    <span className="inline-flex items-center gap-0.5 text-xs text-forest-600">
                      <ShieldCheck className="h-3 w-3" />
                      管理员
                    </span>
                  ) : (
                    <span className="inline-flex items-center gap-0.5 text-xs text-clay-400">
                      <User className="h-3 w-3" />
                      成员
                    </span>
                  )}
                </div>
              </div>
              {isAdmin && member.user_id !== user?.id && (
                <button
                  onClick={() =>
                    handleRoleChange(member.user_id, member.role === "admin" ? "member" : "admin")
                  }
                  disabled={updateMemberRole.isPending}
                  className={`shrink-0 inline-flex items-center gap-1 rounded-lg px-2.5 py-1 text-xs font-medium transition-colors ${
                    member.role === "admin"
                      ? "text-clay-600 bg-clay-100 hover:bg-clay-200"
                      : "text-forest-700 bg-forest-50 hover:bg-forest-100"
                  }`}
                >
                  {member.role === "admin" ? "降为成员" : "设为管理员"}
                </button>
              )}
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
          {/* Cover Image */}
          <div className="rounded-2xl border border-clay-200 bg-white p-6 shadow-sm">
            <h3 className="font-display text-lg font-semibold text-clay-900 flex items-center gap-2 mb-4">
              <ImagePlus className="h-5 w-5 text-forest-600" />
              团队背景图
            </h3>
            <div className="relative aspect-[4/1] rounded-xl overflow-hidden bg-clay-100 mb-3">
              {team.cover_url ? (
                <>
                  {!coverLoaded && (
                    <div className="absolute inset-0 animate-pulse bg-clay-200" />
                  )}
                  <img
                    src={team.cover_url}
                    alt="cover"
                    className="h-full w-full object-cover"
                    loading="lazy"
                    onLoad={() => setCoverLoaded(true)}
                  />
                </>
              ) : (
                <div className="h-full w-full flex items-center justify-center text-clay-300 text-sm">
                  暂无背景图
                </div>
              )}
            </div>
            <label className="btn-secondary inline-flex items-center gap-2 cursor-pointer">
              <ImagePlus className="h-4 w-4" />
              {coverUploading ? "上传中..." : "上传新背景图"}
              <input
                type="file"
                accept="image/*"
                className="hidden"
                onChange={handleCoverUpload}
                disabled={coverUploading}
              />
            </label>
          </div>

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

          {/* Member Role Management */}
          <div className="rounded-2xl border border-clay-200 bg-white p-6 shadow-sm">
            <h3 className="font-display text-lg font-semibold text-clay-900 flex items-center gap-2 mb-4">
              <Users className="h-5 w-5 text-forest-600" />
              成员角色管理
            </h3>

            <div className="space-y-2">
              {members.map((member) => (
                <div
                  key={member.user_id}
                  className="flex items-center justify-between gap-3 rounded-xl border border-clay-100 bg-cream-50 p-3"
                >
                  <div className="flex items-center gap-3 min-w-0 flex-1">
                    <div className="h-9 w-9 rounded-full bg-forest-100 flex items-center justify-center text-forest-700 text-sm font-medium shrink-0">
                      {member.avatar_url ? (
                        <img src={member.avatar_url} alt={member.username} className="h-9 w-9 rounded-full object-cover" />
                      ) : (
                        member.username.charAt(0).toUpperCase()
                      )}
                    </div>
                    <div className="min-w-0">
                      <p className="text-sm font-medium text-clay-900">{member.username}</p>
                      <span
                        className={`inline-flex items-center gap-0.5 text-xs ${
                          member.role === "admin" ? "text-forest-600" : "text-clay-400"
                        }`}
                      >
                        {member.role === "admin" ? (
                          <>
                            <ShieldCheck className="h-3 w-3" />
                            管理员
                          </>
                        ) : (
                          <>
                            <User className="h-3 w-3" />
                            普通成员
                          </>
                        )}
                      </span>
                    </div>
                  </div>
                  {member.user_id === user?.id ? (
                    <span className="text-xs text-clay-400 shrink-0">你自己</span>
                  ) : (
                    <button
                      onClick={() =>
                        handleRoleChange(member.user_id, member.role === "admin" ? "member" : "admin")
                      }
                      disabled={updateMemberRole.isPending}
                      className={`shrink-0 inline-flex items-center gap-1 rounded-lg px-3 py-1.5 text-xs font-medium transition-colors ${
                        member.role === "admin"
                          ? "text-clay-700 bg-clay-100 hover:bg-clay-200"
                          : "text-white bg-forest-600 hover:bg-forest-700"
                      }`}
                    >
                      {member.role === "admin" ? "降为成员" : "设为管理员"}
                    </button>
                  )}
                </div>
              ))}
              {members.length === 0 && (
                <p className="text-sm text-clay-400">暂无成员</p>
              )}
            </div>
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
