import { create } from "zustand";
import { persist } from "zustand/middleware";
import * as api from "@/api/client";
import type { User } from "@/types";

interface AuthState {
  user: User | null;
  token: string | null;
  initialized: boolean;
  init: () => Promise<void>;
  login: (username: string, password: string) => Promise<void>;
  register: (username: string, password: string) => Promise<void>;
  logout: () => void;
  setUser: (user: User) => void;
}

export const useAuth = create<AuthState>()(
  persist(
    (set, get) => ({
      user: null,
      token: null,
      initialized: false,

      init: async () => {
        const token = get().token;
        if (!token) {
          localStorage.removeItem("auth-token");
          set({ initialized: true });
          return;
        }
        try {
          const user = await api.getMe();
          set({ user, initialized: true });
        } catch {
          localStorage.removeItem("auth-token");
          set({ user: null, token: null, initialized: true });
        }
      },

      login: async (username: string, password: string) => {
        const res = await api.loginUser({ username, password });
        localStorage.setItem("auth-token", res.token);
        set({ user: res.user, token: res.token });
      },

      register: async (username: string, password: string) => {
        const res = await api.registerUser({ username, password });
        localStorage.setItem("auth-token", res.token);
        set({ user: res.user, token: res.token });
      },

      logout: () => {
        localStorage.removeItem("auth-token");
        set({ user: null, token: null });
      },

      setUser: (user: User) => {
        set({ user });
      },
    }),
    {
      name: "gohiking-auth",
      partialize: (state) => ({ token: state.token }),
    }
  )
);
