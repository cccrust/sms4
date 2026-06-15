import { useState, useEffect } from "react";
import { useNavigate } from "react-router-dom";
import { api } from "../api/client";
import { useAuth } from "../contexts/AuthContext";
import type { GroupWithOwner } from "../types";

export default function GroupList() {
  const { user } = useAuth();
  const navigate = useNavigate();
  const [groups, setGroups] = useState<GroupWithOwner[]>([]);
  const [myGroups, setMyGroups] = useState<GroupWithOwner[]>([]);
  const [loading, setLoading] = useState(true);
  const [tab, setTab] = useState<"all" | "mine">("all");
  const [search, setSearch] = useState("");
  const [showCreate, setShowCreate] = useState(false);
  const [newName, setNewName] = useState("");
  const [newDesc, setNewDesc] = useState("");

  useEffect(() => {
    const fetch = async () => {
      setLoading(true);
      try {
        const [all, mine] = await Promise.all([
          api.groups.list(search || undefined),
          user ? api.groups.mine(user.id) : Promise.resolve([]),
        ]);
        setGroups(all);
        setMyGroups(mine);
      } catch {
        // ignore
      } finally {
        setLoading(false);
      }
    };
    fetch();
  }, [search, user]);

  const createGroup = async () => {
    if (!user || !newName.trim()) return;
    try {
      const g = await api.groups.create(user.id, newName.trim(), newDesc.trim() || undefined);
      setShowCreate(false);
      setNewName("");
      setNewDesc("");
      navigate(`/groups/${g.id}`);
    } catch (e: unknown) {
      alert(e instanceof Error ? e.message : "建立失敗");
    }
  };

  const displayed = tab === "all" ? groups : myGroups;

  return (
    <div>
      <div className="sticky top-0 bg-black/80 backdrop-blur px-4 py-3 border-b border-gray-800 space-y-2">
        <div className="flex items-center justify-between">
          <h2 className="font-bold text-white text-lg">社團</h2>
          {user && (
            <button
              onClick={() => setShowCreate(true)}
              className="px-4 py-1 rounded-full text-sm font-bold bg-white text-black hover:bg-gray-200 transition"
            >
              建立社團
            </button>
          )}
        </div>
        <div className="flex gap-2">
          <button
            onClick={() => setTab("all")}
            className={`px-3 py-1 rounded-full text-sm transition ${tab === "all" ? "bg-white text-black" : "text-gray-500 hover:text-white"}`}
          >
            探索
          </button>
          {user && (
            <button
              onClick={() => setTab("mine")}
              className={`px-3 py-1 rounded-full text-sm transition ${tab === "mine" ? "bg-white text-black" : "text-gray-500 hover:text-white"}`}
            >
              我的社團
            </button>
          )}
        </div>
        <input
          type="text"
          placeholder="搜尋社團..."
          value={search}
          onChange={(e) => setSearch(e.target.value)}
          className="w-full bg-gray-900 text-white rounded-full px-4 py-2 text-sm placeholder-gray-500 outline-none focus:ring-1 focus:ring-gray-600"
        />
      </div>

      {showCreate && (
        <div className="fixed inset-0 z-50 bg-black/80 flex items-end" onClick={() => setShowCreate(false)}>
          <div className="w-full max-w-lg mx-auto bg-gray-900 rounded-t-2xl p-4 space-y-3" onClick={(e) => e.stopPropagation()}>
            <h3 className="font-bold text-white text-lg">建立社團</h3>
            <input
              type="text"
              placeholder="社團名稱"
              value={newName}
              onChange={(e) => setNewName(e.target.value)}
              className="w-full bg-gray-800 text-white rounded-lg px-3 py-2 text-sm outline-none focus:ring-1 focus:ring-gray-600"
            />
            <textarea
              placeholder="描述（選填）"
              value={newDesc}
              onChange={(e) => setNewDesc(e.target.value)}
              className="w-full bg-gray-800 text-white rounded-lg px-3 py-2 text-sm outline-none focus:ring-1 focus:ring-gray-600"
            />
            <div className="flex gap-2">
              <button onClick={createGroup} className="px-4 py-1.5 rounded-full text-sm font-bold bg-white text-black hover:bg-gray-200 transition">
                建立
              </button>
              <button onClick={() => setShowCreate(false)} className="px-4 py-1.5 rounded-full text-sm text-gray-500 hover:text-white transition">
                取消
              </button>
            </div>
          </div>
        </div>
      )}

      {loading ? (
        <div className="text-center py-20 text-gray-500">載入中...</div>
      ) : displayed.length === 0 ? (
        <div className="text-center py-20 text-gray-500">
          <p className="text-lg">{tab === "mine" ? "尚未加入社團" : "尚無社團"}</p>
        </div>
      ) : (
        <div className="divide-y divide-gray-800">
          {displayed.map((g) => (
            <div
              key={g.id}
              onClick={() => navigate(`/groups/${g.id}`)}
              className="px-4 py-3 hover:bg-gray-900/50 transition cursor-pointer"
            >
              <div className="flex items-center justify-between">
                <div className="flex-1 min-w-0">
                  <span className="font-bold text-white">{g.name}</span>
                  <span className="text-gray-500 ml-2 text-sm">{g.member_count} 位成員</span>
                </div>
                <span className="text-xs text-gray-600">@{g.owner_username}</span>
              </div>
              {g.description && (
                <p className="text-sm text-gray-400 mt-1 truncate">{g.description}</p>
              )}
            </div>
          ))}
        </div>
      )}
    </div>
  );
}
