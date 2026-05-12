import { useState } from "react";
import { Link, useNavigate } from "react-router-dom";
import { Mountain, Loader2 } from "lucide-react";
import { useAuth } from "@/hooks/useAuth";

export default function Login() {
  const navigate = useNavigate();
  const login = useAuth((s) => s.login);
  const [email, setEmail] = useState("");
  const [password, setPassword] = useState("");
  const [error, setError] = useState("");
  const [loading, setLoading] = useState(false);

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setError("");
    setLoading(true);
    try {
      await login(email, password);
      navigate("/");
    } catch (err: any) {
      setError(err.response?.data?.error || "登录失败，请检查邮箱和密码");
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="mx-auto max-w-md">
      <div className="rounded-2xl border border-clay-200 bg-white p-8 shadow-sm">
        <div className="mb-6 text-center">
          <div className="mx-auto mb-4 flex h-12 w-12 items-center justify-center rounded-xl bg-forest-100">
            <Mountain className="h-6 w-6 text-forest-700" />
          </div>
          <h1 className="font-display text-2xl font-semibold text-clay-900">欢迎回来</h1>
          <p className="mt-1 text-sm text-clay-500">登录你的 GoHiking 账号</p>
        </div>

        {error && (
          <div className="mb-4 rounded-xl bg-red-50 px-4 py-3 text-sm text-red-600">
            {error}
          </div>
        )}

        <form onSubmit={handleSubmit} className="space-y-4">
          <div>
            <label className="mb-1.5 block text-sm font-medium text-clay-700">邮箱</label>
            <input
              type="email"
              required
              value={email}
              onChange={(e) => setEmail(e.target.value)}
              className="input-field w-full"
              placeholder="your@email.com"
            />
          </div>
          <div>
            <label className="mb-1.5 block text-sm font-medium text-clay-700">密码</label>
            <input
              type="password"
              required
              minLength={6}
              value={password}
              onChange={(e) => setPassword(e.target.value)}
              className="input-field w-full"
              placeholder="••••••"
            />
          </div>
          <button
            type="submit"
            disabled={loading}
            className="btn-primary w-full justify-center"
          >
            {loading ? <Loader2 className="h-4 w-4 animate-spin" /> : "登录"}
          </button>
        </form>

        <p className="mt-6 text-center text-sm text-clay-500">
          还没有账号？{" "}
          <Link to="/register" className="font-medium text-forest-600 hover:text-forest-700">
            立即注册
          </Link>
        </p>
      </div>
    </div>
  );
}
