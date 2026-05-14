import { useState } from "react";
import { useNavigate, Link } from "react-router-dom";
import { ArrowLeft, Loader2, LogIn } from "lucide-react";
import { useCreateTeam } from "@/hooks/useTeams";
import { useAuth } from "@/hooks/useAuth";

export default function CreateTeam() {
  const { user } = useAuth();
  const navigate = useNavigate();
  const createMutation = useCreateTeam();

  const [name, setName] = useState("");
  const [slug, setSlug] = useState("");
  const [description, setDescription] = useState("");

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
          登录后才能创建团队
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
    if (!name.trim() || !slug.trim()) return;

    const team = await createMutation.mutateAsync({
      name: name.trim(),
      slug: slug.trim(),
      description: description.trim() || undefined,
    });

    navigate(`/teams/${team.id}`);
  };

  // Auto-generate slug from name
  const handleNameChange = (value: string) => {
    setName(value);
    if (!slug || slug === name.toLowerCase().replace(/\s+/g, "-")) {
      setSlug(value.toLowerCase().replace(/\s+/g, "-").replace(/[^a-z0-9-]/g, ""));
    }
  };

  return (
    <div className="mx-auto max-w-2xl">
      <Link
        to="/teams"
        className="mb-4 inline-flex items-center gap-1.5 text-sm text-clay-500 hover:text-clay-700 transition-colors"
      >
        <ArrowLeft className="h-4 w-4" />
        返回团队列表
      </Link>

      <h1 className="font-display text-3xl font-semibold text-clay-900">
        创建徒步团队
      </h1>
      <p className="mt-2 text-clay-500">
        组建你的徒步小队，发起活动，分享山野故事
      </p>

      <form onSubmit={handleSubmit} className="mt-8 space-y-6">
        <div className="rounded-2xl border border-clay-200 bg-white p-6 sm:p-8 shadow-sm space-y-6">
          <div>
            <label className="mb-1.5 block text-sm font-medium text-clay-700">
              团队名称 <span className="text-red-500">*</span>
            </label>
            <input
              type="text"
              value={name}
              onChange={(e) => handleNameChange(e.target.value)}
              placeholder="例如：深圳徒步爱好者"
              className="input-field"
              required
            />
          </div>

          <div>
            <label className="mb-1.5 block text-sm font-medium text-clay-700">
              团队标识 <span className="text-red-500">*</span>
            </label>
            <div className="flex items-center gap-2">
              <span className="text-sm text-clay-400">gohiking.club/teams/</span>
              <input
                type="text"
                value={slug}
                onChange={(e) => setSlug(e.target.value.toLowerCase().replace(/[^a-z0-9-]/g, ""))}
                placeholder="your-team"
                className="input-field flex-1"
                required
              />
            </div>
            <p className="mt-1 text-xs text-clay-400">
              仅支持小写字母、数字和连字符
            </p>
          </div>

          <div>
            <label className="mb-1.5 block text-sm font-medium text-clay-700">
              团队介绍
            </label>
            <textarea
              value={description}
              onChange={(e) => setDescription(e.target.value)}
              placeholder="介绍一下你们的团队，比如常去的路线、活动频率..."
              rows={4}
              className="input-field"
            />
          </div>
        </div>

        <div className="flex flex-col gap-3 sm:flex-row-reverse">
          <button
            type="submit"
            disabled={!name.trim() || !slug.trim() || createMutation.isPending}
            className="btn-primary px-8 py-3"
          >
            {createMutation.isPending && (
              <Loader2 className="h-4 w-4 animate-spin" />
            )}
            创建团队
          </button>
          <Link to="/teams" className="btn-secondary px-8 py-3 text-center">
            取消
          </Link>
        </div>
      </form>
    </div>
  );
}
