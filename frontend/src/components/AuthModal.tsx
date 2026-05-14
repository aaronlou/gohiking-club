import { useState } from "react";
import { Mountain, X, Loader2, LogIn, UserPlus } from "lucide-react";
import { useAuth } from "@/hooks/useAuth";

interface AuthModalProps {
  isOpen: boolean;
  onClose: () => void;
  defaultMode?: "login" | "register";
}

export function AuthModal({ isOpen, onClose, defaultMode = "login" }: AuthModalProps) {
  const [mode, setMode] = useState<"login" | "register">(defaultMode);
  const [username, setUsername] = useState("");
  const [password, setPassword] = useState("");
  const [error, setError] = useState("");
  const [loading, setLoading] = useState(false);

  const login = useAuth((s) => s.login);
  const register = useAuth((s) => s.register);

  if (!isOpen) return null;

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setError("");
    setLoading(true);
    try {
      if (mode === "login") {
        await login(username, password);
      } else {
        await register(username, password);
      }
      onClose();
      setUsername("");
      setPassword("");
    } catch (err: any) {
      setError(err.response?.data?.error || (mode === "login" ? "登录失败" : "注册失败"));
    } finally {
      setLoading(false);
    }
  };

  const toggleMode = () => {
    setMode((m) => (m === "login" ? "register" : "login"));
    setError("");
  };

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center p-4">
      {/* Backdrop */}
      <div
        className="absolute inset-0 bg-forest-950/40 backdrop-blur-sm transition-opacity"
        onClick={onClose}
      />

      {/* Modal */}
      <div className="relative w-full max-w-sm animate-scale-in">
        <div className="rounded-2xl border border-clay-200 bg-white p-8 shadow-[0_20px_60px_rgba(0,0,0,0.15)]">
          {/* Close button */}
          <button
            onClick={onClose}
            className="absolute right-4 top-4 rounded-lg p-1 text-clay-400 hover:bg-clay-100 hover:text-clay-600 transition-colors"
          >
            <X className="h-4 w-4" />
          </button>

          {/* Header */}
          <div className="mb-6 text-center">
            <div className="mx-auto mb-4 flex h-12 w-12 items-center justify-center rounded-xl bg-gradient-to-br from-forest-100 to-earth-100">
              <Mountain className="h-6 w-6 text-forest-700" />
            </div>
            <h2 className="font-display text-xl font-semibold text-clay-900">
              {mode === "login" ? "欢迎回来" : "加入 GoHiking"}
            </h2>
            <p className="mt-1 text-sm text-clay-500">
              {mode === "login" ? "登录后继续探索徒步世界" : "两步即可开始你的徒步之旅"}
            </p>
          </div>

          {/* Error */}
          {error && (
            <div className="mb-4 rounded-xl bg-red-50 px-4 py-3 text-sm text-red-600">
              {error}
            </div>
          )}

          {/* Form */}
          <form onSubmit={handleSubmit} className="space-y-4">
            <div>
              <label className="mb-1.5 block text-sm font-medium text-clay-700">
                用户名
              </label>
              <input
                type="text"
                required
                autoFocus
                value={username}
                onChange={(e) => setUsername(e.target.value)}
                className="input-field w-full"
                placeholder="hiker_42"
              />
            </div>
            <div>
              <label className="mb-1.5 block text-sm font-medium text-clay-700">
                密码
              </label>
              <input
                type="password"
                required
                minLength={6}
                value={password}
                onChange={(e) => setPassword(e.target.value)}
                className="input-field w-full"
                placeholder={mode === "register" ? "至少 6 位字符" : "••••••"}
              />
            </div>
            <button
              type="submit"
              disabled={loading}
              className="btn-primary w-full justify-center"
            >
              {loading ? (
                <Loader2 className="h-4 w-4 animate-spin" />
              ) : mode === "login" ? (
                <>
                  <LogIn className="h-4 w-4" />
                  登录
                </>
              ) : (
                <>
                  <UserPlus className="h-4 w-4" />
                  注册
                </>
              )}
            </button>
          </form>

          {/* Toggle */}
          <p className="mt-6 text-center text-sm text-clay-500">
            {mode === "login" ? "还没有账号？" : "已有账号？"}{" "}
            <button
              onClick={toggleMode}
              className="font-medium text-forest-600 hover:text-forest-700 transition-colors"
            >
              {mode === "login" ? "立即注册" : "直接登录"}
            </button>
          </p>
        </div>
      </div>
    </div>
  );
}
