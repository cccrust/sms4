import { useState, type FormEvent } from "react";
import { Link, useNavigate } from "react-router-dom";
import { useAuth } from "../contexts/AuthContext";

export default function LoginPage() {
  const { login } = useAuth();
  const navigate = useNavigate();
  const [username, setUsername] = useState("");
  const [password, setPassword] = useState("");
  const [error, setError] = useState("");
  const [loading, setLoading] = useState(false);

  const handleSubmit = async (e: FormEvent) => {
    e.preventDefault();
    setError("");
    if (!username.trim() || !password) {
      setError("請填寫帳號與密碼");
      return;
    }
    setLoading(true);
    try {
      await login(username.trim(), password);
      navigate("/", { replace: true });
    } catch (err) {
      setError(err instanceof Error ? err.message : "登入失敗");
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="min-h-full flex flex-col items-center justify-center px-6 bg-black">
      <div className="w-full max-w-sm">
        <h1 className="text-3xl font-bold text-white text-center mb-2">SMS4</h1>
        <p className="text-gray-500 text-center mb-8">登入你的帳號</p>

        {error && (
          <div className="bg-red-500/10 border border-red-500/30 text-red-400 text-sm rounded-lg px-4 py-2 mb-4">
            {error}
          </div>
        )}

        <form onSubmit={handleSubmit} className="space-y-4">
          <input
            type="text"
            placeholder="帳號"
            value={username}
            onChange={(e) => setUsername(e.target.value)}
            className="w-full bg-gray-900 text-white rounded-lg px-4 py-3 text-sm border border-gray-700 focus:border-blue-500 outline-none"
            autoFocus
          />
          <input
            type="password"
            placeholder="密碼"
            value={password}
            onChange={(e) => setPassword(e.target.value)}
            className="w-full bg-gray-900 text-white rounded-lg px-4 py-3 text-sm border border-gray-700 focus:border-blue-500 outline-none"
          />
          <button
            type="submit"
            disabled={loading}
            className="w-full bg-blue-500 text-white font-bold rounded-full py-3 text-sm hover:bg-blue-600 disabled:opacity-50 transition"
          >
            {loading ? "登入中..." : "登入"}
          </button>
        </form>

        <p className="text-gray-500 text-sm text-center mt-6">
          還沒有帳號？{" "}
          <Link to="/register" className="text-blue-500 hover:underline">
            註冊
          </Link>
        </p>
      </div>
    </div>
  );
}
