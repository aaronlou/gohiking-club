import { useEffect } from "react";
import { BrowserRouter, Routes, Route } from "react-router-dom";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { Layout } from "@/components/Layout";
import { RequireAuth } from "@/components/RequireAuth";
import { useAuth } from "@/hooks/useAuth";
import Home from "@/pages/Home";
import Events from "@/pages/Events";
import EventDetail from "@/pages/EventDetail";
import CreateEvent from "@/pages/CreateEvent";
import Teams from "@/pages/Teams";
import TeamDetail from "@/pages/TeamDetail";
import CreateTeam from "@/pages/CreateTeam";
import Gallery from "@/pages/Gallery";
import UploadPage from "@/pages/Upload";
import Profile from "@/pages/Profile";
import Login from "@/pages/Login";

const queryClient = new QueryClient({
  defaultOptions: {
    queries: {
      retry: 1,
      refetchOnWindowFocus: false,
    },
  },
});

function AppRoutes() {
  const init = useAuth((s) => s.init);
  const initialized = useAuth((s) => s.initialized);

  useEffect(() => {
    init();
  }, [init]);

  if (!initialized) {
    return (
      <div className="flex h-screen items-center justify-center bg-cream-50">
        <div className="h-8 w-8 animate-spin rounded-full border-2 border-forest-600 border-t-transparent" />
      </div>
    );
  }

  return (
    <Routes>
      <Route element={<Layout />}>
        <Route path="/" element={<Home />} />
        <Route path="/events" element={<Events />} />
        <Route path="/events/:id" element={<EventDetail />} />
        <Route path="/teams" element={<Teams />} />
        <Route path="/teams/:id" element={<TeamDetail />} />
        <Route path="/gallery" element={<Gallery />} />
        <Route path="/login" element={<Login />} />

        {/* Protected routes */}
        <Route
          path="/events/new"
          element={
            <RequireAuth>
              <CreateEvent />
            </RequireAuth>
          }
        />
        <Route
          path="/teams/new"
          element={
            <RequireAuth>
              <CreateTeam />
            </RequireAuth>
          }
        />
        <Route
          path="/upload"
          element={
            <RequireAuth>
              <UploadPage />
            </RequireAuth>
          }
        />
        <Route
          path="/profile"
          element={
            <RequireAuth>
              <Profile />
            </RequireAuth>
          }
        />
      </Route>
    </Routes>
  );
}

export default function App() {
  return (
    <QueryClientProvider client={queryClient}>
      <BrowserRouter>
        <AppRoutes />
      </BrowserRouter>
    </QueryClientProvider>
  );
}
