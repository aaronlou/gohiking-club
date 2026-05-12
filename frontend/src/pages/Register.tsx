import { useState } from "react";
import { Link, useNavigate } from "react-router-dom";
import { Mountain, Loader2 } from "lucide-react";
import { useAuth } from "@/hooks/useAuth";

export default function Register() {
  const navigate = useNavigate();
  const register = useAuth((s) => s.register);
  const [username, setUsername] = useState("");
  const [email, setEmail] = useState("");
  const [password, setPassword] = useState("");
  const [error, setError] = useState("");
  const [loading, setLoading] = useState(false);

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setError("");
    setLoading(true);
    try {
      await register(username, email, password);
      navigate("/");
    } catch (err: any) {
      setError(err.response?.data?.error || "注册失败，请稍后重试");
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
          <h1 className="font-display text-2xl font-semibold text-clay-900">创建账号</h1>
          <p className="mt-1 text-sm text-clay-500">加入 GoHiking 徒步社区</p>
        </div>

        {error && (
          <div className="mb-4 rounded-xl bg-red-50 px-4 py-3 text-sm text-red-600">
            {error}
          </div>
        )}

        <form onSubmit={handleSubmit} className="space-y-4">
          <div>
            <label className="mb-1.5 block text-sm font-medium text-clay-700">用户名</label>
            <input
              type="text"
              required
              value={username}
              onChange={(e) => setUsername(e.target.value)}
              className="input-field w-full"
              placeholder="hiker_42"
            />
          </div>
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
              placeholder="至少 6 位字符"
            />
          </div>
          <button
            type="submit"
            disabled={loading}
            className="btn-primary w-full justify-center"
          >
            {loading ? <Loader2 className="h-4 w-4 animate-spin" /> : "注册"}
          </button>
        </form>

        <p className="mt-6 text-center text-sm text-clay-500">
          已有账号？{" "}
          <Link to="/login" className="font-medium text-forest-600 hover:text-forest-700">
            直接登录
          </Link>
        </p>
      </div>
    </div>
  );
}
