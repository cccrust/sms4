import { useState, useEffect, useRef } from "react";
import { useParams, useSearchParams, useNavigate } from "react-router-dom";
import { api } from "../api/client";
import { useAuth } from "../contexts/AuthContext";

export default function ShopMessages() {
  const { shopId } = useParams();
  const [searchParams] = useSearchParams();
  const { user } = useAuth();
  const navigate = useNavigate();
  const [msgs, setMsgs] = useState<import("../types").ShopMessageWithUser[]>([]);
  const [shopName, setShopName] = useState("商店");
  const [loading, setLoading] = useState(true);
  const [input, setInput] = useState("");
  const bottomRef = useRef<HTMLDivElement>(null);

  const sid = parseInt(shopId || "0");
  const otherId = parseInt(searchParams.get("other_id") || "0");

  useEffect(() => {
    const fetch = async () => {
      if (!user || !sid || !otherId) return;
      try {
        setLoading(true);
        const [data, shopData] = await Promise.all([
          api.shopMessages.list(sid, user.id, otherId),
          api.shops.get(sid).catch(() => null),
        ]);
        setMsgs(data);
        if (shopData) setShopName(shopData.name);
      } catch {
        // ignore
      } finally {
        setLoading(false);
      }
    };
    fetch();
    const timer = setInterval(fetch, 3000);
    return () => clearInterval(timer);
  }, [sid, otherId, user]);

  useEffect(() => {
    bottomRef.current?.scrollIntoView({ behavior: "smooth" });
  }, [msgs]);

  const send = async () => {
    if (!user || !input.trim()) return;
    try {
      await api.shopMessages.send(user.id, otherId, sid, input.trim());
      setInput("");
      const data = await api.shopMessages.list(sid, user.id, otherId);
      setMsgs(data);
    } catch (e: unknown) {
      alert(e instanceof Error ? e.message : "傳送失敗");
    }
  };

  if (!user) return null;

  return (
    <div className="h-full flex flex-col">
      <div className="sticky top-0 z-10 bg-black/80 backdrop-blur border-b border-gray-800 px-4 py-3 flex items-center gap-3">
        <button onClick={() => navigate(-1)} className="text-white">
          <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M15 19l-7-7 7-7" />
          </svg>
        </button>
        <div>
          <h2 className="font-bold text-white text-sm">{shopName}</h2>
          <p className="text-xs text-gray-500">商店私訊</p>
        </div>
      </div>

      <div className="flex-1 overflow-y-auto px-4 py-3 space-y-3">
        {loading ? (
          <div className="text-center py-20 text-gray-500">載入中...</div>
        ) : msgs.length === 0 ? (
          <div className="text-center py-20 text-gray-500">
            <p>尚無訊息</p>
            <p className="text-sm mt-1">傳送訊息給賣家詢問</p>
          </div>
        ) : (
          msgs.map((m) => (
            <div key={m.id} className={`flex ${m.sender_id === user.id ? "justify-end" : "justify-start"}`}>
              <div
                className={`max-w-[80%] px-3 py-2 rounded-2xl text-sm ${
                  m.sender_id === user.id
                    ? "bg-blue-600 text-white rounded-br-md"
                    : "bg-gray-800 text-white rounded-bl-md"
                }`}
              >
                <p>{m.content}</p>
                <p className="text-[10px] mt-1 opacity-60">{new Date(m.created_at).toLocaleTimeString("zh-TW", { hour: "2-digit", minute: "2-digit" })}</p>
              </div>
            </div>
          ))
        )}
        <div ref={bottomRef} />
      </div>

      <div className="sticky bottom-0 border-t border-gray-800 px-4 py-3">
        <div className="flex gap-2">
          <input
            type="text"
            placeholder="輸入訊息..."
            value={input}
            onChange={(e) => setInput(e.target.value)}
            onKeyDown={(e) => e.key === "Enter" && send()}
            className="flex-1 bg-gray-900 text-white rounded-full px-4 py-2 text-sm placeholder-gray-500 outline-none focus:ring-1 focus:ring-gray-600"
          />
          <button
            onClick={send}
            disabled={!input.trim()}
            className="px-4 py-2 rounded-full text-sm font-bold bg-white text-black hover:bg-gray-200 transition disabled:opacity-50"
          >
            傳送
          </button>
        </div>
      </div>
    </div>
  );
}
