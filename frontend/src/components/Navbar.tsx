import { Link, useLocation } from "react-router-dom";
import { Mountain, Image, Upload, User, CalendarDays, Menu, X } from "lucide-react";
import { useState } from "react";

const links = [
  { to: "/", label: "首页", icon: Mountain },
  { to: "/events", label: "活动", icon: CalendarDays },
  { to: "/gallery", label: "画廊", icon: Image },
  { to: "/upload", label: "上传", icon: Upload },
  { to: "/profile", label: "我的", icon: User },
] as const;

export function Navbar() {
  const { pathname } = useLocation();
  const [menuOpen, setMenuOpen] = useState(false);

  return (
    <header className="sticky top-0 z-50 border-b border-gray-200 bg-white/80 backdrop-blur-md">
      <nav className="mx-auto flex h-16 max-w-6xl items-center justify-between px-4">
        <Link
          to="/"
          className="flex items-center gap-2 text-lg font-bold text-brand-700 shrink-0"
          onClick={() => setMenuOpen(false)}
        >
          <Mountain className="h-6 w-6" />
          <span>GoHiking</span>
        </Link>

        {/* Desktop nav */}
        <div className="hidden sm:flex items-center gap-1">
          {links.map(({ to, label, icon: Icon }) => {
            const active = pathname === to;
            return (
              <Link
                key={to}
                to={to}
                className={`flex items-center gap-1.5 rounded-lg px-3 py-2 text-sm font-medium transition-colors ${
                  active
                    ? "bg-brand-50 text-brand-700"
                    : "text-gray-600 hover:bg-gray-100 hover:text-gray-900"
                }`}
              >
                <Icon className="h-4 w-4" />
                {label}
              </Link>
            );
          })}
        </div>

        {/* Mobile hamburger */}
        <button
          onClick={() => setMenuOpen(!menuOpen)}
          className="sm:hidden inline-flex items-center justify-center rounded-lg p-2 text-gray-600 hover:bg-gray-100"
          aria-label="菜单"
        >
          {menuOpen ? <X className="h-6 w-6" /> : <Menu className="h-6 w-6" />}
        </button>
      </nav>

      {/* Mobile drawer */}
      {menuOpen && (
        <>
          <div
            className="fixed inset-0 z-40 bg-black/20 sm:hidden"
            onClick={() => setMenuOpen(false)}
          />
          <div className="absolute left-0 right-0 z-50 border-b border-gray-200 bg-white shadow-lg sm:hidden">
            <div className="flex flex-col gap-1 px-4 pb-4 pt-2">
              {links.map(({ to, label, icon: Icon }) => {
                const active = pathname === to;
                return (
                  <Link
                    key={to}
                    to={to}
                    onClick={() => setMenuOpen(false)}
                    className={`flex items-center gap-3 rounded-lg px-4 py-3 text-base font-medium transition-colors ${
                      active
                        ? "bg-brand-50 text-brand-700"
                        : "text-gray-600 hover:bg-gray-50"
                    }`}
                  >
                    <Icon className="h-5 w-5" />
                    {label}
                  </Link>
                );
              })}
            </div>
          </div>
        </>
      )}
    </header>
  );
}
