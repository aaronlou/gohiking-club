import { useState, useEffect } from "react";
import { useNavigate, Link, useSearchParams } from "react-router-dom";
import { ArrowLeft, Loader2, LogIn, FileText } from "lucide-react";
import { useCreateEvent } from "@/hooks/useEvents";
import { useTeam } from "@/hooks/useTeams";
import { useAuth } from "@/hooks/useAuth";

export default function CreateEvent() {
  const { user } = useAuth();
  const navigate = useNavigate();
  const [searchParams] = useSearchParams();
  const teamId = searchParams.get("team_id");
  const { data: team } = useTeam(teamId || "");
  const createMutation = useCreateEvent();

  const [title, setTitle] = useState("");
  const [description, setDescription] = useState("");
  const [location, setLocation] = useState("");
  const [date, setDate] = useState("");
  const [distanceKm, setDistanceKm] = useState("");
  const [elevationGainM, setElevationGainM] = useState("");
  const [disclaimer, setDisclaimer] = useState("");

  useEffect(() => {
    if (team?.default_disclaimer) {
      setDisclaimer(team.default_disclaimer);
    }
  }, [team?.default_disclaimer]);

  if (!user) {
    return (
      <div className="mx-auto max-w-lg px-4 text-center py-16 sm:py-24">
        <div className="mb-6 inline-flex h-24 w-24 items-center justify-center rounded-full bg-clay-100">
          <LogIn className="h-12 w-12 text-clay-500" />
        </div>
        <h2 className="font-display text-2xl font-semibold text-clay-900 mb-2">
          请先登录
        </h2>
        <p className="text-clay-500 mb-8">
          登录后才能创建活动
        </p>
        <Link to="/login" className="btn-primary inline-flex items-center gap-2">
          <LogIn className="h-4 w-4" />
          去登录
        </Link>
      </div>
    );
  }

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!title.trim()) return;

    const event = await createMutation.mutateAsync({
      title: title.trim(),
      description: description.trim() || undefined,
      location: location.trim() || undefined,
      date: date || undefined,
      team_id: teamId || undefined,
      distance_km: distanceKm ? parseFloat(distanceKm) : undefined,
      elevation_gain_m: elevationGainM ? parseInt(elevationGainM) : undefined,
      disclaimer: disclaimer.trim() || undefined,
    });

    navigate(`/events/${event.id}`);
  };

  return (
    <div className="mx-auto max-w-2xl">
      <Link
        to={teamId ? `/teams/${teamId}` : "/events"}
        className="mb-4 inline-flex items-center gap-1.5 text-sm text-clay-500 hover:text-clay-700 transition-colors"
      >
        <ArrowLeft className="h-4 w-4" />
        {teamId ? "返回团队" : "返回活动列表"}
      </Link>

      <h1 className="font-display text-3xl font-semibold text-clay-900">
        {teamId ? `为「${team?.name || ""}」创建活动` : "创建徒步活动"}
      </h1>
      <p className="mt-2 text-clay-500">
        创建一个活动，让其他人加入并分享徒步照片
      </p>

      <form onSubmit={handleSubmit} className="mt-8 space-y-6">
        <div className="rounded-2xl border border-clay-200 bg-white p-6 sm:p-8 shadow-sm space-y-6">
          <div>
            <label className="mb-1.5 block text-sm font-medium text-clay-700">
              活动名称 <span className="text-red-500">*</span>
            </label>
            <input
              type="text"
              value={title}
              onChange={(e) => setTitle(e.target.value)}
              placeholder="例如：周末梧桐山徒步"
              className="input-field"
              required
            />
          </div>

          <div>
            <label className="mb-1.5 block text-sm font-medium text-clay-700">
              活动描述
            </label>
            <textarea
              value={description}
              onChange={(e) => setDescription(e.target.value)}
              placeholder="描述一下这次活动的路线、难度、集合地点等信息..."
              rows={4}
              className="input-field"
            />
          </div>

          <div className="grid gap-5 sm:grid-cols-2">
            <div>
              <label className="mb-1.5 block text-sm font-medium text-clay-700">
                活动地点
              </label>
              <input
                type="text"
                value={location}
                onChange={(e) => setLocation(e.target.value)}
                placeholder="例如：深圳梧桐山"
                className="input-field"
              />
            </div>
            <div>
              <label className="mb-1.5 block text-sm font-medium text-clay-700">
                活动日期
              </label>
              <input
                type="date"
                value={date}
                onChange={(e) => setDate(e.target.value)}
                className="input-field"
              />
            </div>
          </div>

          <div className="grid gap-5 sm:grid-cols-2">
            <div>
              <label className="mb-1.5 block text-sm font-medium text-clay-700">
                预估距离（KM）
              </label>
              <input
                type="number"
                step="0.1"
                min="0"
                value={distanceKm}
                onChange={(e) => setDistanceKm(e.target.value)}
                placeholder="例如：12.5"
                className="input-field"
              />
            </div>
            <div>
              <label className="mb-1.5 block text-sm font-medium text-clay-700">
                累计爬升（M）
              </label>
              <input
                type="number"
                min="0"
                value={elevationGainM}
                onChange={(e) => setElevationGainM(e.target.value)}
                placeholder="例如：850"
                className="input-field"
              />
            </div>
          </div>

          <div>
            <label className="mb-1.5 block text-sm font-medium text-clay-700 flex items-center gap-1.5">
              <FileText className="h-4 w-4" />
              免责声明
            </label>
            <textarea
              value={disclaimer}
              onChange={(e) => setDisclaimer(e.target.value)}
              placeholder="本活动为自愿参加，组织者不对任何意外事故承担责任..."
              rows={4}
              className="input-field"
            />
            {team?.default_disclaimer && disclaimer === team.default_disclaimer && (
              <p className="mt-1 text-xs text-forest-600">已自动引用团队默认免责声明模板</p>
            )}
          </div>
        </div>

        <div className="flex flex-col gap-3 sm:flex-row-reverse">
          <button
            type="submit"
            disabled={!title.trim() || createMutation.isPending}
            className="btn-primary px-8 py-3"
          >
            {createMutation.isPending && (
              <Loader2 className="h-4 w-4 animate-spin" />
            )}
            创建活动
          </button>
          <Link to={teamId ? `/teams/${teamId}` : "/events"} className="btn-secondary px-8 py-3 text-center">
            取消
          </Link>
        </div>
      </form>
    </div>
  );
}
