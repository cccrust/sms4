import { useState, useEffect } from "react";
import { useNavigate } from "react-router-dom";
import { api } from "../api/client";
import { useAuth } from "../contexts/AuthContext";
import type { CartItemWithDetails } from "../types";

export default function CartPage() {
  const { user } = useAuth();
  const navigate = useNavigate();
  const [items, setItems] = useState<CartItemWithDetails[]>([]);
  const [loading, setLoading] = useState(true);
  const [checkingOut, setCheckingOut] = useState(false);

  useEffect(() => {
    if (!user) return;
    setLoading(true);
    api.cart.list(user.id).then(setItems).catch(() => {}).finally(() => setLoading(false));
  }, [user]);

  const updateQty = async (item: CartItemWithDetails, qty: number) => {
    if (!user) return;
    if (qty <= 0) {
      await api.cart.remove(item.id, user.id);
      setItems((prev) => prev.filter((i) => i.id !== item.id));
    } else {
      await api.cart.updateQuantity(item.id, user.id, qty);
      setItems((prev) => prev.map((i) => i.id === item.id ? { ...i, quantity: qty, total_price: i.price * qty } : i));
    }
  };

  const removeItem = async (id: number) => {
    if (!user) return;
    await api.cart.remove(id, user.id);
    setItems((prev) => prev.filter((i) => i.id !== id));
  };

  const doCheckout = async () => {
    if (!user) return;
    setCheckingOut(true);
    try {
      const res = await api.cart.checkout(user.id);
      setItems([]);
      alert(`結帳成功！訂單編號：${res.order_ids.join(", ")}`);
      navigate("/orders");
    } catch (e: unknown) {
      alert(e instanceof Error ? e.message : "結帳失敗");
    } finally {
      setCheckingOut(false);
    }
  };

  const total = items.reduce((sum, i) => sum + i.total_price, 0);

  if (!user) return null;

  return (
    <div>
      <div className="px-4 py-3 border-b border-gray-800 flex items-center justify-between">
        <h2 className="font-bold text-white text-lg">購物車</h2>
        {items.length > 0 && (
          <span className="text-sm text-gray-500">{items.length} 項商品</span>
        )}
      </div>

      {loading ? (
        <div className="text-center py-20 text-gray-500">載入中...</div>
      ) : items.length === 0 ? (
        <div className="text-center py-20 text-gray-500">
          <p className="text-lg">購物車是空的</p>
          <p className="text-sm mt-1">前往商場逛逛吧！</p>
          <button
            onClick={() => navigate("/marketplace")}
            className="mt-4 px-4 py-1.5 rounded-full text-sm font-bold bg-white text-black hover:bg-gray-200 transition"
          >
            去商場
          </button>
        </div>
      ) : (
        <div>
          <div className="divide-y divide-gray-800">
            {items.map((item) => (
              <div key={item.id} className="px-4 py-3">
                <div className="flex items-start justify-between">
                  <div className="flex-1 min-w-0">
                    <div className="flex items-center gap-2">
                      <span className="font-bold text-white">{item.product_name}</span>
                      <span className="text-sm text-gray-400">${item.price}</span>
                    </div>
                    <div className="text-sm text-gray-500 mt-0.5">{item.shop_name}</div>
                  </div>
                  <div className="text-right">
                    <div className="text-white font-bold">${item.total_price}</div>
                  </div>
                </div>
                <div className="flex items-center justify-between mt-2">
                  <div className="flex items-center gap-2">
                    <button
                      onClick={() => updateQty(item, item.quantity - 1)}
                      className="w-7 h-7 rounded-full bg-gray-800 text-white flex items-center justify-center hover:bg-gray-700 transition"
                    >
                      -
                    </button>
                    <span className="text-white text-sm w-6 text-center">{item.quantity}</span>
                    <button
                      onClick={() => updateQty(item, item.quantity + 1)}
                      disabled={item.quantity >= item.stock}
                      className="w-7 h-7 rounded-full bg-gray-800 text-white flex items-center justify-center hover:bg-gray-700 transition disabled:opacity-50"
                    >
                      +
                    </button>
                  </div>
                  <button
                    onClick={() => removeItem(item.id)}
                    className="text-xs text-red-500 hover:text-red-400 transition"
                  >
                    移除
                  </button>
                </div>
              </div>
            ))}
          </div>

          <div className="sticky bottom-0 border-t border-gray-800 bg-black px-4 py-3">
            <div className="flex items-center justify-between mb-3">
              <span className="text-gray-400">總計</span>
              <span className="text-xl font-bold text-white">${total}</span>
            </div>
            <button
              onClick={doCheckout}
              disabled={checkingOut || items.length === 0}
              className="w-full py-2.5 rounded-full text-sm font-bold bg-white text-black hover:bg-gray-200 transition disabled:opacity-50"
            >
              {checkingOut ? "結帳中..." : "結帳"}
            </button>
          </div>
        </div>
      )}
    </div>
  );
}
