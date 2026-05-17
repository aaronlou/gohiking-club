import { useState } from "react";
import { Link, useNavigate, useLocation } from "react-router-dom";
import { Mountain, Loader2, LogIn, UserPlus } from "lucide-react";
import { useAuth } from "@/hooks/useAuth";

export default function Login() {
  const navigate = useNavigate();
  const location = useLocation();
  const login = useAuth((s) => s.login);
  const register = useAuth((s) => s.register);
  const from = (location.state as any)?.from || "/";
  const [mode, setMode] = useState<"login" | "register">("login");
  const [username, setUsername] = useState("");
  const [password, setPassword] = useState("");
  const [error, setError] = useState("");
  const [loading, setLoading] = useState(false);

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
      navigate(from, { replace: true });
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
    <div className="mx-auto max-w-md pt-8">
      <div className="rounded-2xl border border-clay-100 bg-white p-8 shadow-[0_8px_32px_rgba(0,0,0,0.06)]">
        <div className="mb-6 text-center">
          <div className="mx-auto mb-4 flex h-12 w-12 items-center justify-center rounded-xl bg-gradient-to-br from-forest-100 to-earth-100">
            <Mountain className="h-6 w-6 text-forest-700" />
          </div>
          <h1 className="font-display text-2xl font-semibold text-clay-900">
            {mode === "login" ? "欢迎回来" : "加入 GoHiking"}
          </h1>
          <p className="mt-1 text-sm text-clay-500">
            {mode === "login"
              ? "登录你的账号继续探索"
              : "两步即可开始你的徒步之旅"}
          </p>
        </div>

        {error && (
          <div className="mb-4 rounded-xl bg-red-50 px-4 py-3 text-sm text-red-600">
            {error}
          </div>
        )}

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

        <p className="mt-6 text-center text-sm text-clay-500">
          {mode === "login" ? "还没有账号？" : "已有账号？"}{" "}
          <button
            onClick={toggleMode}
            className="font-medium text-forest-600 hover:text-forest-700 transition-colors"
          >
            {mode === "login" ? "立即注册" : "直接登录"}
          </button>
        </p>

        <div className="mt-4 text-center">
          <Link
            to="/"
            className="text-xs text-clay-400 hover:text-clay-600 transition-colors"
          >
            ← 返回首页
          </Link>
        </div>
      </div>
    </div>
  );
}
