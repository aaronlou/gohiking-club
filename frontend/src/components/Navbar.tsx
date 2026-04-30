import { Link, useLocation } from "react-router-dom";
import { Mountain, Image, Upload, User, CalendarDays, Menu, X, LogOut } from "lucide-react";
import { useState, useEffect } from "react";
import { useAuth } from "@/hooks/useAuth";

const links = [
  { to: "/", label: "首页", icon: Mountain },
  { to: "/events", label: "活动", icon: CalendarDays },
  { to: "/gallery", label: "画廊", icon: Image },
  { to: "/upload", label: "上传", icon: Upload },
] as const;

export function Navbar() {
  const { pathname } = useLocation();
  const [menuOpen, setMenuOpen] = useState(false);
  const [scrolled, setScrolled] = useState(false);
  const user = useAuth((s) => s.user);
  const logout = useAuth((s) => s.logout);

  useEffect(() => {
    const onScroll = () => setScrolled(window.scrollY > 20);
    window.addEventListener("scroll", onScroll, { passive: true });
    return () => window.removeEventListener("scroll", onScroll);
  }, []);

  const initial = user?.username?.charAt(0).toUpperCase() ?? "?";

  return (
    <header
      className={`sticky top-0 z-50 transition-all duration-300 ${
        scrolled
          ? "bg-cream-50/85 backdrop-blur-lg shadow-sm border-b border-clay-200"
          : "bg-transparent"
      }`}
    >
      <nav className="mx-auto flex h-16 max-w-7xl items-center justify-between px-4 sm:px-6 lg:px-8">
        <Link
          to="/"
          className="flex items-center gap-2 shrink-0 group"
          onClick={() => setMenuOpen(false)}
        >
          <span className="flex h-8 w-8 items-center justify-center rounded-lg bg-forest-700 text-cream-50 transition-colors group-hover:bg-forest-600">
            <Mountain className="h-4 w-4" />
          </span>
          <span className="font-display text-lg font-semibold text-forest-900">
            GoHiking.
          </span>
        </Link>

        {/* Desktop nav */}
        <div className="hidden sm:flex items-center gap-1">
          {links.map(({ to, label, icon: Icon }) => {
            const active = pathname === to;
            return (
              <Link
                key={to}
                to={to}
                className={`relative flex items-center gap-1.5 rounded-lg px-3 py-2 text-sm font-medium transition-colors ${
                  active
                    ? "text-forest-700"
                    : "text-clay-600 hover:text-clay-900"
                }`}
              >
                <Icon className="h-4 w-4" />
                {label}
                {active && (
                  <span className="absolute -bottom-0.5 left-2 right-2 h-0.5 rounded-full bg-forest-500" />
                )}
              </Link>
            );
          })}

          {/* Auth */}
          {user ? (
            <div className="flex items-center gap-1 ml-2 pl-2 border-l border-clay-200">
              <Link
                to="/profile"
                className={`relative flex items-center gap-1.5 rounded-lg px-3 py-2 text-sm font-medium transition-colors ${
                  pathname === "/profile"
                    ? "text-forest-700"
                    : "text-clay-600 hover:text-clay-900"
                }`}
              >
                <span className="flex h-6 w-6 items-center justify-center rounded-full bg-forest-100 text-xs font-semibold text-forest-700">
                  {initial}
                </span>
                {user.username}
              </Link>
              <button
                onClick={logout}
                className="rounded-lg px-2 py-2 text-clay-400 hover:text-clay-600 transition-colors"
                title="退出登录"
              >
                <LogOut className="h-4 w-4" />
              </button>
            </div>
          ) : (
            <Link
              to="/login"
              className="ml-2 inline-flex items-center gap-1.5 rounded-full bg-forest-700 px-4 py-2 text-sm font-medium text-cream-50 hover:bg-forest-600 transition-colors"
            >
              <User className="h-4 w-4" />
              登录
            </Link>
          )}
        </div>

        {/* Mobile hamburger */}
        <button
          onClick={() => setMenuOpen(!menuOpen)}
          className="sm:hidden inline-flex items-center justify-center rounded-lg p-2 text-clay-600 hover:bg-clay-100 transition-colors"
          aria-label="菜单"
        >
          {menuOpen ? <X className="h-5 w-5" /> : <Menu className="h-5 w-5" />}
        </button>
      </nav>

      {/* Mobile drawer */}
      {menuOpen && (
        <>
          <div
            className="fixed inset-0 z-40 bg-black/20 backdrop-blur-sm sm:hidden animate-fade-in"
            onClick={() => setMenuOpen(false)}
          />
          <div className="absolute left-0 right-0 z-50 border-b border-clay-200 bg-cream-50 shadow-xl sm:hidden animate-slide-up">
            <div className="flex flex-col gap-1 px-4 pb-6 pt-2">
              {links.map(({ to, label, icon: Icon }) => {
                const active = pathname === to;
                return (
                  <Link
                    key={to}
                    to={to}
                    onClick={() => setMenuOpen(false)}
                    className={`flex items-center gap-3 rounded-xl px-4 py-3 text-base font-medium transition-colors ${
                      active
                        ? "bg-forest-100 text-forest-800"
                        : "text-clay-700 hover:bg-clay-100"
                    }`}
                  >
                    <Icon className="h-5 w-5" />
                    {label}
                  </Link>
                );
              })}

              {/* Mobile auth */}
              <hr className="my-2 border-clay-200" />
              {user ? (
                <>
                  <Link
                    to="/profile"
                    onClick={() => setMenuOpen(false)}
                    className="flex items-center gap-3 rounded-xl px-4 py-3 text-base font-medium text-clay-700 hover:bg-clay-100 transition-colors"
                  >
                    <span className="flex h-6 w-6 items-center justify-center rounded-full bg-forest-100 text-xs font-semibold text-forest-700">
                      {initial}
                    </span>
                    {user.username}
                  </Link>
                  <button
                    onClick={() => {
                      logout();
                      setMenuOpen(false);
                    }}
                    className="flex items-center gap-3 rounded-xl px-4 py-3 text-base font-medium text-clay-500 hover:bg-clay-100 transition-colors"
                  >
                    <LogOut className="h-5 w-5" />
                    退出登录
                  </button>
                </>
              ) : (
                <Link
                  to="/login"
                  onClick={() => setMenuOpen(false)}
                  className="flex items-center gap-3 rounded-xl bg-forest-700 px-4 py-3 text-base font-medium text-cream-50 hover:bg-forest-600 transition-colors"
                >
                  <User className="h-5 w-5" />
                  登录 / 注册
                </Link>
              )}
            </div>
          </div>
        </>
      )}
    </header>
  );
}
