import { Link } from "react-router-dom";
import { Users, Plus, ArrowRight, Mountain, Compass } from "lucide-react";
import { useState } from "react";
import { useAuth } from "@/hooks/useAuth";
import { AuthModal } from "@/components/AuthModal";
import { useTeams } from "@/hooks/useTeams";

export default function Teams() {
  const { data: teams = [], isLoading } = useTeams();
  const user = useAuth((s) => s.user);
  const [authModalOpen, setAuthModalOpen] = useState(false);

  return (
    <div className="animate-fade-in">
      {/* Hero header with gradient */}
      <div className="relative -mx-4 -mt-8 sm:-mx-6 sm:-mt-12 lg:-mx-8 mb-8 px-4 sm:px-6 lg:px-8 pt-8 sm:pt-12 pb-8 overflow-hidden">
        <div className="absolute inset-0 bg-gradient-to-br from-forest-50 via-cream-50 to-earth-50" />
        <div className="absolute inset-0 bg-[radial-gradient(ellipse_at_top_right,_rgba(22,57,38,0.04)_0%,_transparent_50%)]" />
        <div className="relative flex items-end justify-between gap-4">
          <div>
            <div className="inline-flex items-center gap-2 rounded-full bg-forest-100/80 px-3 py-1 text-xs font-medium text-forest-700 mb-3">
              <Compass className="h-3.5 w-3.5" />
              探索团队
            </div>
            <h1 className="font-display text-3xl font-semibold text-clay-900">
              徒步团队
            </h1>
            <p className="mt-1.5 text-clay-500 max-w-md">
              加入或创建团队，和志同道合的伙伴一起探索山野
            </p>
          </div>
          {user ? (
            <Link to="/teams/new" className="btn-primary shrink-0">
              <Plus className="h-4 w-4" />
              创建团队
            </Link>
          ) : (
            <button
              onClick={() => setAuthModalOpen(true)}
              className="btn-primary shrink-0"
            >
              <Plus className="h-4 w-4" />
              创建团队
            </button>
          )}
        </div>
      </div>

      {isLoading ? (
        <div className="flex justify-center py-20">
          <div className="h-8 w-8 animate-spin rounded-full border-2 border-forest-600 border-t-transparent" />
        </div>
      ) : teams.length === 0 ? (
        <div className="relative rounded-2xl border border-clay-100 bg-white p-12 text-center overflow-hidden">
          <div className="absolute inset-0 bg-gradient-to-b from-forest-50/50 to-transparent" />
          <div className="relative">
            <div className="mx-auto mb-5 inline-flex h-20 w-20 items-center justify-center rounded-2xl bg-gradient-to-br from-forest-100 to-earth-100">
              <Mountain className="h-10 w-10 text-forest-600" />
            </div>
            <h3 className="font-display text-xl font-semibold text-clay-800 mb-1">
              还没有团队
            </h3>
            <p className="text-clay-500 mb-8 max-w-xs mx-auto">
              成为第一个创建徒步团队的人吧，召集志同道合的伙伴
            </p>
            {user ? (
              <Link
                to="/teams/new"
                className="btn-primary inline-flex items-center gap-2"
              >
                <Plus className="h-4 w-4" />
                创建团队
              </Link>
            ) : (
              <button
                onClick={() => setAuthModalOpen(true)}
                className="btn-primary inline-flex items-center gap-2"
              >
                <Plus className="h-4 w-4" />
                创建团队
              </button>
            )}
          </div>
        </div>
      ) : (
        <div className="grid gap-4 sm:grid-cols-2 lg:grid-cols-3">
          {teams.map((team) => (
            <Link
              key={team.id}
              to={`/teams/${team.id}`}
              className="group card-hover p-5"
            >
              <div className="flex items-start gap-4">
                <div className="flex h-12 w-12 shrink-0 items-center justify-center rounded-xl bg-gradient-to-br from-forest-100 to-forest-50 text-forest-700">
                  {team.logo_url ? (
                    <img src={team.logo_url} alt={team.name} className="h-8 w-8 rounded-lg object-cover" />
                  ) : (
                    <Mountain className="h-6 w-6" />
                  )}
                </div>
                <div className="min-w-0 flex-1">
                  <h3 className="font-display text-base font-semibold text-clay-900 group-hover:text-forest-700 transition-colors truncate">
                    {team.name}
                  </h3>
                  {team.description && (
                    <p className="mt-0.5 text-sm text-clay-500 line-clamp-2">
                      {team.description}
                    </p>
                  )}
                  <div className="mt-2 flex items-center gap-3 text-xs text-clay-400">
                    <span className="inline-flex items-center gap-1">
                      <Users className="h-3.5 w-3.5" />
                      {team.member_count} 成员
                    </span>
                    <span className="inline-flex items-center gap-1">
                      {team.event_count} 活动
                    </span>
                  </div>
                </div>
                <ArrowRight className="h-4 w-4 text-clay-300 group-hover:text-forest-600 group-hover:translate-x-0.5 transition-all shrink-0 mt-1" />
              </div>
            </Link>
          ))}
        </div>
      )}
      <AuthModal isOpen={authModalOpen} onClose={() => setAuthModalOpen(false)} defaultMode="register" />
    </div>
  );
}
