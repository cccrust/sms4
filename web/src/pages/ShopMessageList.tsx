import { useState, useEffect } from "react";
import { useNavigate } from "react-router-dom";
import { api } from "../api/client";
import { useAuth } from "../contexts/AuthContext";
import type { ShopConversation } from "../types";

export default function ShopMessageList() {
  const { user } = useAuth();
  const navigate = useNavigate();
  const [convs, setConvs] = useState<ShopConversation[]>([]);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    const fetch = async () => {
      if (!user) return;
      setLoading(true);
      try {
        const data = await api.shopMessages.conversations(user.id);
        setConvs(data);
      } catch {
        // ignore
      } finally {
        setLoading(false);
      }
    };
    fetch();
    const timer = setInterval(fetch, 3000);
    return () => clearInterval(timer);
  }, [user]);

  if (loading) {
    return <div className="text-center py-20 text-gray-500">載入中...</div>;
  }

  return (
    <div>
      <div className="px-4 py-3 border-b border-gray-800">
        <h2 className="font-bold text-white text-lg">商店私訊</h2>
      </div>

      {convs.length === 0 ? (
        <div className="text-center py-20 text-gray-500">
          <p className="text-lg">尚無商店私訊</p>
          <p className="text-sm mt-1">前往商場逛逛並詢問賣家</p>
        </div>
      ) : (
        convs.map((c) => (
          <div
            key={`${c.shop_id}-${c.other_id}`}
            onClick={() => navigate(`/shop-messages/${c.shop_id}?other_id=${c.other_id}`)}
            className="border-b border-gray-800 px-4 py-3 hover:bg-gray-900/50 transition cursor-pointer"
          >
            <div className="flex items-center gap-3">
              <div className="w-12 h-12 rounded-full bg-gray-700 flex items-center justify-center text-lg font-bold shrink-0">
                {c.shop_name[0]}
              </div>
              <div className="flex-1 min-w-0">
                <div className="font-bold text-white text-sm">{c.shop_name}</div>
                <div className="text-sm text-gray-500 truncate">
                  {c.last_message || "尚無訊息"}
                </div>
              </div>
              {c.last_message_at && (
                <span className="text-xs text-gray-600 shrink-0">
                  {new Date(c.last_message_at).toLocaleDateString("zh-TW", { month: "short", day: "numeric" })}
                </span>
              )}
            </div>
          </div>
        ))
      )}
    </div>
  );
}
