import { Link } from "react-router-dom";
import { Users, Plus, ArrowRight, Mountain } from "lucide-react";
import { useTeams } from "@/hooks/useTeams";

export default function Teams() {
  const { data: teams = [], isLoading } = useTeams();

  return (
    <div className="animate-fade-in">
      <div className="flex items-center justify-between mb-8">
        <div>
          <h1 className="font-display text-3xl font-semibold text-clay-900">
            徒步团队
          </h1>
          <p className="mt-1 text-clay-500">
            加入或创建团队，和志同道合的伙伴一起探索山野
          </p>
        </div>
        <Link
          to="/teams/new"
          className="btn-primary shrink-0"
        >
          <Plus className="h-4 w-4" />
          创建团队
        </Link>
      </div>

      {isLoading ? (
        <div className="flex justify-center py-20">
          <div className="h-8 w-8 animate-spin rounded-full border-2 border-forest-600 border-t-transparent" />
        </div>
      ) : teams.length === 0 ? (
        <div className="rounded-2xl border border-clay-200 bg-white p-12 text-center">
          <Mountain className="mx-auto h-12 w-12 text-clay-300 mb-4" />
          <h3 className="font-display text-lg font-semibold text-clay-700 mb-1">
            还没有团队
          </h3>
          <p className="text-clay-500 mb-6">
            成为第一个创建徒步团队的人吧
          </p>
          <Link to="/teams/new" className="btn-primary inline-flex items-center gap-2">
            <Plus className="h-4 w-4" />
            创建团队
          </Link>
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
                <div className="flex h-12 w-12 shrink-0 items-center justify-center rounded-xl bg-forest-100 text-forest-700">
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
    </div>
  );
}
