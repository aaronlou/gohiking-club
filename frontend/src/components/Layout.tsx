import { Outlet } from "react-router-dom";
import { Navbar } from "./Navbar";

export function Layout() {
  return (
    <div className="min-h-screen topo-bg">
      <Navbar />
      <main className="mx-auto max-w-7xl px-4 sm:px-6 lg:px-8 py-8 sm:py-12">
        <Outlet />
      </main>
      <footer className="relative mt-20 border-t border-clay-200 bg-forest-950 py-12 text-center overflow-hidden">
        <div className="absolute inset-0 topo-bg-dark opacity-30" />
        <div className="relative z-10">
          <p className="font-display text-lg text-cream-100">
            GoHiking.Club
          </p>
          <p className="mt-2 text-sm text-clay-400">
            &copy; {new Date().getFullYear()} — 发现徒步之美
          </p>
        </div>
      </footer>
    </div>
  );
}
