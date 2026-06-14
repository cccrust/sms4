import { useState, useEffect } from "react";
import { api } from "../api/client";
import { useAuth } from "../contexts/AuthContext";
import type { Shop, Product } from "../types";

export default function MyShop() {
  const { user } = useAuth();
  const [shop, setShop] = useState<Shop | null>(null);
  const [products, setProducts] = useState<Product[]>([]);
  const [loading, setLoading] = useState(true);
  const [showForm, setShowForm] = useState(false);
  const [shopName, setShopName] = useState("");
  const [shopDesc, setShopDesc] = useState("");
  const [pName, setPName] = useState("");
  const [pPrice, setPPrice] = useState("");
  const [pStock, setPStock] = useState("1");
  const [pDesc, setPDesc] = useState("");

  useEffect(() => {
    const init = async () => {
      if (!user) return;
      setLoading(true);
      try {
        const s = await api.shops.my(user.id).catch(() => null);
        setShop(s);
        if (s) {
          const ps = await api.products.listByShop(s.id);
          setProducts(ps);
        }
      } catch {
        // ignore
      } finally {
        setLoading(false);
      }
    };
    init();
  }, [user]);

  const openShop = async () => {
    if (!user) return;
    try {
      const res = await api.shops.open(user.id, shopName, shopDesc || undefined);
      setShop(res.shop);
      setShowForm(false);
      setShopName("");
      setShopDesc("");
    } catch (e: unknown) {
      alert(e instanceof Error ? e.message : "開店失敗");
    }
  };

  const closeShop = async () => {
    if (!user || !shop) return;
    if (!confirm("確定要關閉商店？")) return;
    try {
      await api.shops.close(user.id);
      setShop(null);
      setProducts([]);
    } catch (e: unknown) {
      alert(e instanceof Error ? e.message : "關店失敗");
    }
  };

  const addProduct = async () => {
    if (!user || !shop) return;
    try {
      const p = await api.products.add(shop.id, user.id, {
        name: pName,
        price: parseInt(pPrice),
        stock: parseInt(pStock) || 0,
        description: pDesc || undefined,
      });
      setProducts((prev) => [p, ...prev]);
      setPName("");
      setPPrice("");
      setPStock("1");
      setPDesc("");
    } catch (e: unknown) {
      alert(e instanceof Error ? e.message : "上架失敗");
    }
  };

  const removeProduct = async (id: number) => {
    if (!user) return;
    if (!confirm("確定刪除商品？")) return;
    try {
      await api.products.remove(id, user.id);
      setProducts((prev) => prev.filter((p) => p.id !== id));
    } catch (e: unknown) {
      alert(e instanceof Error ? e.message : "刪除失敗");
    }
  };

  if (loading) {
    return <div className="text-center py-20 text-gray-500">載入中...</div>;
  }

  return (
    <div>
      <div className="px-4 py-3 border-b border-gray-800 flex items-center justify-between">
        <h2 className="font-bold text-white text-lg">我的商店</h2>
        {shop && (
          <button
            onClick={closeShop}
            className="text-xs text-red-500 hover:text-red-400 transition"
          >
            關閉商店
          </button>
        )}
      </div>

      {!shop ? (
        <div className="p-4">
          <p className="text-gray-500 mb-3">你還沒有開店</p>
          {showForm ? (
            <div className="space-y-2">
              <input
                type="text"
                placeholder="商店名稱"
                value={shopName}
                onChange={(e) => setShopName(e.target.value)}
                className="w-full bg-gray-900 text-white rounded-lg px-3 py-2 text-sm outline-none focus:ring-1 focus:ring-gray-600"
              />
              <textarea
                placeholder="商店描述（選填）"
                value={shopDesc}
                onChange={(e) => setShopDesc(e.target.value)}
                className="w-full bg-gray-900 text-white rounded-lg px-3 py-2 text-sm outline-none focus:ring-1 focus:ring-gray-600"
              />
              <div className="flex gap-2">
                <button
                  onClick={openShop}
                  className="px-4 py-1.5 rounded-full text-sm font-bold bg-white text-black hover:bg-gray-200 transition"
                >
                  開店
                </button>
                <button
                  onClick={() => setShowForm(false)}
                  className="px-4 py-1.5 rounded-full text-sm text-gray-500 hover:text-white transition"
                >
                  取消
                </button>
              </div>
            </div>
          ) : (
            <button
              onClick={() => setShowForm(true)}
              className="px-4 py-1.5 rounded-full text-sm font-bold bg-white text-black hover:bg-gray-200 transition"
            >
              開店
            </button>
          )}
        </div>
      ) : (
        <div>
          <div className="px-4 py-3 border-b border-gray-800">
            <h3 className="font-bold text-white">{shop.name}</h3>
            {shop.description && (
              <p className="text-sm text-gray-400 mt-1">{shop.description}</p>
            )}
          </div>

          <div className="px-4 py-3 border-b border-gray-800 space-y-2">
            <h4 className="text-sm font-bold text-gray-300">新增商品</h4>
            <input
              type="text"
              placeholder="商品名稱"
              value={pName}
              onChange={(e) => setPName(e.target.value)}
              className="w-full bg-gray-900 text-white rounded-lg px-3 py-2 text-sm outline-none focus:ring-1 focus:ring-gray-600"
            />
            <div className="flex gap-2">
              <input
                type="number"
                placeholder="價格"
                value={pPrice}
                onChange={(e) => setPPrice(e.target.value)}
                className="w-1/2 bg-gray-900 text-white rounded-lg px-3 py-2 text-sm outline-none focus:ring-1 focus:ring-gray-600"
              />
              <input
                type="number"
                placeholder="庫存"
                value={pStock}
                onChange={(e) => setPStock(e.target.value)}
                className="w-1/2 bg-gray-900 text-white rounded-lg px-3 py-2 text-sm outline-none focus:ring-1 focus:ring-gray-600"
              />
            </div>
            <input
              type="text"
              placeholder="描述（選填）"
              value={pDesc}
              onChange={(e) => setPDesc(e.target.value)}
              className="w-full bg-gray-900 text-white rounded-lg px-3 py-2 text-sm outline-none focus:ring-1 focus:ring-gray-600"
            />
            <button
              onClick={addProduct}
              className="px-4 py-1.5 rounded-full text-sm font-bold bg-white text-black hover:bg-gray-200 transition"
            >
              上架
            </button>
          </div>

          {products.length === 0 ? (
            <div className="text-center py-10 text-gray-500 text-sm">尚無商品</div>
          ) : (
            <div className="divide-y divide-gray-800">
              {products.map((p) => (
                <div key={p.id} className="px-4 py-3 flex items-center justify-between">
                  <div>
                    <span className="text-white font-bold">{p.name}</span>
                    <span className="text-gray-400 ml-2">${p.price}</span>
                    <span className="text-gray-600 ml-2">庫存 {p.stock}</span>
                    {p.description && (
                      <p className="text-xs text-gray-500 mt-0.5">{p.description}</p>
                    )}
                  </div>
                  <button
                    onClick={() => removeProduct(p.id)}
                    className="text-xs text-red-500 hover:text-red-400 transition"
                  >
                    刪除
                  </button>
                </div>
              ))}
            </div>
          )}
        </div>
      )}
    </div>
  );
}
