import { useState, useEffect } from "react";
import { NavLink, Outlet } from "react-router-dom";
import { useAuth } from "../contexts/AuthContext";
import { api } from "../api/client";

const navItems = [
  { label: "首頁", to: "/", icon: "🏠" },
  { label: "配對", to: "/search", icon: "🔍" },
  { label: "商場", to: "/marketplace", icon: "🛒" },
  { label: "私訊", to: "/messages", icon: "✉️" },
  { label: "使用者", to: "/users", icon: "👤" },
];

export default function Layout() {
  const { user, logout } = useAuth();
  const [cartCount, setCartCount] = useState(0);

  useEffect(() => {
    if (!user) return;
    api.cart.count(user.id).then((d) => setCartCount(d.count)).catch(() => {});
    const timer = setInterval(() => {
      api.cart.count(user.id).then((d) => setCartCount(d.count)).catch(() => {});
    }, 5000);
    return () => clearInterval(timer);
  }, [user]);

  return (
    <div className="h-full flex flex-col max-w-lg mx-auto bg-black">
      <header className="sticky top-0 z-10 bg-black/80 backdrop-blur border-b border-gray-800 px-4 py-3">
        <div className="flex items-center justify-between">
          <h1 className="text-xl font-bold text-white">SMS4</h1>
          {user && (
            <div className="flex items-center gap-3">
              <NavLink to="/my-shop" className="text-xs text-gray-500 hover:text-white transition">
                我的商店
              </NavLink>
              <NavLink to="/cart" className="text-xs text-gray-500 hover:text-white transition relative">
                購物車{cartCount > 0 && <span className="ml-1 text-blue-400 font-bold">({cartCount})</span>}
              </NavLink>
              <NavLink to="/orders" className="text-xs text-gray-500 hover:text-white transition">
                訂單
              </NavLink>
              <NavLink to="/shop-messages" className="text-xs text-gray-500 hover:text-white transition">
                商店私訊
              </NavLink>
              <NavLink to={`/users/${user.id}`} className="text-sm text-gray-400 hover:text-white transition">
                @{user.username}
              </NavLink>
              <button
                onClick={logout}
                className="text-xs text-gray-500 hover:text-red-400 transition"
              >
                登出
              </button>
            </div>
          )}
        </div>
      </header>

      <main className="flex-1 overflow-y-auto">
        <Outlet />
      </main>

      <nav className="sticky bottom-0 bg-black border-t border-gray-800">
        <div className="flex justify-around py-2">
          {navItems.map((item) => (
            <NavLink
              key={item.to}
              to={item.to}
              end={item.to === "/"}
              className={({ isActive }) =>
                `flex flex-col items-center px-6 py-1 text-xs transition ${
                  isActive ? "text-white" : "text-gray-500"
                }`
              }
            >
              <span className="text-xl mb-0.5">{item.icon}</span>
              <span>{item.label}</span>
            </NavLink>
          ))}
        </div>
      </nav>
    </div>
  );
}
