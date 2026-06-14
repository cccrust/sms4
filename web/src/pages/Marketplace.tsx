import { useState, useEffect } from "react";
import { useNavigate } from "react-router-dom";
import { api } from "../api/client";
import { useAuth } from "../contexts/AuthContext";
import type { ProductWithShop } from "../types";

export default function Marketplace() {
  const { user } = useAuth();
  const navigate = useNavigate();
  const [products, setProducts] = useState<ProductWithShop[]>([]);
  const [loading, setLoading] = useState(true);
  const [q, setQ] = useState("");
  const [minPrice, setMinPrice] = useState("");
  const [maxPrice, setMaxPrice] = useState("");

  useEffect(() => {
    const fetchProducts = async () => {
      setLoading(true);
      try {
        const params: Record<string, string> = {};
        if (q) params.q = q;
        if (minPrice) params.min_price = minPrice;
        if (maxPrice) params.max_price = maxPrice;
        const data = await api.products.search(params);
        setProducts(data);
      } catch {
        // ignore
      } finally {
        setLoading(false);
      }
    };
    fetchProducts();
  }, [q, minPrice, maxPrice]);

  const addToCart = async (productId: number) => {
    if (!user) return;
    try {
      await api.cart.add(user.id, productId);
      alert("已加入購物車");
    } catch (e: unknown) {
      alert(e instanceof Error ? e.message : "加入購物車失敗");
    }
  };

  return (
    <div>
      <div className="sticky top-0 bg-black/80 backdrop-blur px-4 py-3 border-b border-gray-800 space-y-2">
        <h2 className="font-bold text-white text-lg">商場</h2>
        <input
          type="text"
          placeholder="搜尋商品..."
          value={q}
          onChange={(e) => setQ(e.target.value)}
          className="w-full bg-gray-900 text-white rounded-full px-4 py-2 text-sm placeholder-gray-500 outline-none focus:ring-1 focus:ring-gray-600"
        />
        <div className="flex gap-2">
          <input
            type="number"
            placeholder="最低價"
            value={minPrice}
            onChange={(e) => setMinPrice(e.target.value)}
            className="w-1/2 bg-gray-900 text-white rounded-lg px-3 py-1.5 text-sm placeholder-gray-500 outline-none focus:ring-1 focus:ring-gray-600"
          />
          <input
            type="number"
            placeholder="最高價"
            value={maxPrice}
            onChange={(e) => setMaxPrice(e.target.value)}
            className="w-1/2 bg-gray-900 text-white rounded-lg px-3 py-1.5 text-sm placeholder-gray-500 outline-none focus:ring-1 focus:ring-gray-600"
          />
        </div>
      </div>

      {loading ? (
        <div className="text-center py-20 text-gray-500">載入中...</div>
      ) : products.length === 0 ? (
        <div className="text-center py-20 text-gray-500">
          <p className="text-lg">尚無商品</p>
          <p className="text-sm mt-1">去開店上架商品吧！</p>
        </div>
      ) : (
        <div className="divide-y divide-gray-800">
          {products.map((p) => (
            <div key={p.id} className="px-4 py-3">
              <div className="flex items-start justify-between">
                <div className="flex-1 min-w-0">
                  <div className="flex items-center gap-2">
                    <span className="font-bold text-white">{p.name}</span>
                    <span className="text-sm text-gray-400">${p.price}</span>
                  </div>
                  <div className="text-sm text-gray-500 mt-0.5">
                    <button
                      onClick={() => navigate(`/shop/${p.shop_id}`)}
                      className="hover:text-white transition"
                    >
                      {p.shop_name}
                    </button>
                    {" · "}庫存 {p.stock}
                  </div>
                  {p.description && (
                    <p className="text-sm text-gray-400 mt-1">{p.description}</p>
                  )}
                </div>
                <div className="flex gap-2 ml-3">
                  {user && (
                    <button
                      onClick={async () => {
                        try {
                          const shopData = await api.shops.my(p.shop_user_id);
                          navigate(`/shop-messages/${shopData.id}?other_id=${p.shop_user_id}`);
                        } catch {
                          alert("該賣家沒有商店");
                        }
                      }}
                      className="px-3 py-1.5 rounded-full text-sm border border-gray-700 text-gray-300 hover:border-gray-500 hover:text-white transition"
                    >
                      私訊
                    </button>
                  )}
                  <button
                    onClick={() => addToCart(p.id)}
                    disabled={p.stock <= 0}
                    className="px-4 py-1.5 rounded-full text-sm font-bold bg-white text-black hover:bg-gray-200 transition disabled:opacity-50 disabled:cursor-not-allowed"
                  >
                    加入購物車
                  </button>
                </div>
              </div>
            </div>
          ))}
        </div>
      )}
    </div>
  );
}
