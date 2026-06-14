import { useState, useEffect } from "react";
import { useParams, useNavigate } from "react-router-dom";
import { api } from "../api/client";
import { useAuth } from "../contexts/AuthContext";
import type { Shop, Product } from "../types";

export default function ShopPage() {
  const { id } = useParams();
  const { user } = useAuth();
  const navigate = useNavigate();
  const [shop, setShop] = useState<Shop | null>(null);
  const [products, setProducts] = useState<Product[]>([]);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    const fetch = async () => {
      if (!id) return;
      setLoading(true);
      try {
        const s = await api.shops.get(parseInt(id));
        setShop(s);
        const ps = await api.products.listByShop(s.id);
        setProducts(ps);
      } catch {
        setShop(null);
      } finally {
        setLoading(false);
      }
    };
    fetch();
  }, [id]);

  const buy = async (productId: number) => {
    if (!user) return navigate("/login");
    try {
      await api.orders.create(user.id, productId);
      alert("訂單已建立！");
    } catch (e: unknown) {
      alert(e instanceof Error ? e.message : "下單失敗");
    }
  };

  if (loading) {
    return <div className="text-center py-20 text-gray-500">載入中...</div>;
  }

  if (!shop) {
    return (
      <div className="text-center py-20 text-gray-500">
        <p className="text-lg">商店不存在</p>
      </div>
    );
  }

  return (
    <div>
      <div className="px-4 py-4 border-b border-gray-800">
        <div className="flex items-center justify-between">
          <div>
            <h2 className="text-xl font-bold text-white">{shop.name}</h2>
            {shop.description && (
              <p className="text-sm text-gray-400 mt-1">{shop.description}</p>
            )}
            <p className="text-xs text-gray-600 mt-2">開店於 {shop.created_at}</p>
          </div>
          {user && user.id !== shop.user_id && (
            <button
              onClick={() => navigate(`/shop-messages/${shop.id}?other_id=${shop.user_id}`)}
              className="px-4 py-1.5 rounded-full text-sm border border-gray-700 text-gray-300 hover:border-gray-500 hover:text-white transition"
            >
              私訊
            </button>
          )}
        </div>
      </div>

      {products.length === 0 ? (
        <div className="text-center py-20 text-gray-500">
          <p className="text-lg">此商店尚無商品</p>
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
                  <div className="text-sm text-gray-500 mt-0.5">庫存 {p.stock}</div>
                  {p.description && (
                    <p className="text-sm text-gray-400 mt-1">{p.description}</p>
                  )}
                </div>
                <div className="flex gap-2 ml-3">
                  {user && (
                    <button
                      onClick={() => navigate(`/shop-messages/${shop.id}?other_id=${shop.user_id}`)}
                      className="px-3 py-1.5 rounded-full text-sm border border-gray-700 text-gray-300 hover:border-gray-500 hover:text-white transition"
                    >
                      私訊
                    </button>
                  )}
                  <button
                    onClick={() => buy(p.id)}
                    disabled={p.stock <= 0}
                    className="px-4 py-1.5 rounded-full text-sm font-bold bg-white text-black hover:bg-gray-200 transition disabled:opacity-50 disabled:cursor-not-allowed"
                  >
                    購買
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
