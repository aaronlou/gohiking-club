import { Outlet } from "react-router-dom";
import { Navbar } from "./Navbar";

export function Layout() {
  return (
    <div className="min-h-screen bg-cream-50 flex flex-col">
      <Navbar />
      <main className="flex-1 mx-auto w-full max-w-7xl px-4 sm:px-6 lg:px-8 py-8 sm:py-12">
        <Outlet />
      </main>
      <footer className="relative mt-auto border-t border-clay-100 bg-forest-950 py-12 text-center">
        <div className="relative z-10">
          <p className="font-display text-lg text-cream-100">
            GoHiking.Club
          </p>
          <p className="mt-2 text-sm text-clay-400">
            &copy; {new Date().getFullYear()} — 发现徒步之美
          </p>
          <p className="mt-2 text-xs text-clay-500">
            浙ICP备2024126456号-4
          </p>
        </div>
      </footer>
    </div>
  );
}
