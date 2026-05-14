import { useState, useEffect } from "react";
import { useAuth } from "@/hooks/useAuth";
import { AuthModal } from "./AuthModal";

interface RequireAuthProps {
  children: React.ReactNode;
}

export function RequireAuth({ children }: RequireAuthProps) {
  const user = useAuth((s) => s.user);
  const initialized = useAuth((s) => s.initialized);
  const [showModal, setShowModal] = useState(false);

  useEffect(() => {
    if (initialized && !user) {
      setShowModal(true);
    }
  }, [initialized, user]);

  if (!initialized) {
    return (
      <div className="flex h-[60vh] items-center justify-center">
        <div className="h-8 w-8 animate-spin rounded-full border-2 border-forest-600 border-t-transparent" />
      </div>
    );
  }

  if (!user) {
    return (
      <AuthModal
        isOpen={showModal}
        onClose={() => setShowModal(false)}
        defaultMode="login"
      />
    );
  }

  return <>{children}</>;
}
