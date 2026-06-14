import { useState, useEffect } from "react";
import { useNavigate } from "react-router-dom";
import { api } from "../api/client";
import { useAuth } from "../contexts/AuthContext";
import type { OrderWithDetails } from "../types";

export default function Orders() {
  const { user } = useAuth();
  const navigate = useNavigate();
  const [orders, setOrders] = useState<OrderWithDetails[]>([]);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    const fetch = async () => {
      if (!user) return;
      setLoading(true);
      try {
        const data = await api.orders.list(user.id);
        setOrders(data);
      } catch {
        // ignore
      } finally {
        setLoading(false);
      }
    };
    fetch();
  }, [user]);

  if (loading) {
    return <div className="text-center py-20 text-gray-500">載入中...</div>;
  }

  return (
    <div>
      <div className="px-4 py-3 border-b border-gray-800">
        <h2 className="font-bold text-white text-lg">我的訂單</h2>
      </div>

      {orders.length === 0 ? (
        <div className="text-center py-20 text-gray-500">
          <p className="text-lg">尚無訂單</p>
          <p className="text-sm mt-1">前往商場逛逛吧！</p>
        </div>
      ) : (
        <div className="divide-y divide-gray-800">
          {orders.map((o) => (
            <div key={o.id} className="px-4 py-3">
              <div className="flex items-center justify-between">
                <div>
                  <span className="text-white font-bold">{o.product_name}</span>
                  <span className="text-gray-400 ml-2">x {o.quantity}</span>
                </div>
                <div className="flex items-center gap-2">
                  <span className="text-sm text-gray-400">${o.total_price}</span>
                  {user && (
                    <button
                      onClick={async () => {
                        try {
                          const shopData = await api.shops.my(o.shop_user_id);
                          navigate(`/shop-messages/${shopData.id}?other_id=${o.shop_user_id}`);
                        } catch {
                          alert("無法連繫賣家");
                        }
                      }}
                      className="px-3 py-1 rounded-full text-xs border border-gray-700 text-gray-300 hover:border-gray-500 hover:text-white transition"
                    >
                      私訊賣家
                    </button>
                  )}
                </div>
              </div>
              <div className="flex items-center justify-between mt-1">
                <span className="text-sm text-gray-500">{o.shop_name}</span>
                <span className={`text-xs px-2 py-0.5 rounded-full ${
                  o.status === "pending"
                    ? "bg-yellow-900 text-yellow-300"
                    : o.status === "shipped"
                    ? "bg-blue-900 text-blue-300"
                    : "bg-green-900 text-green-300"
                }`}>
                  {o.status === "pending" ? "待處理" : o.status === "shipped" ? "已出貨" : "已完成"}
                </span>
              </div>
              <div className="text-xs text-gray-600 mt-1">{o.created_at}</div>
            </div>
          ))}
        </div>
      )}
    </div>
  );
}
