import { useState, useEffect } from "react";
import { useParams, useNavigate } from "react-router-dom";
import { api } from "../api/client";
import { useAuth } from "../contexts/AuthContext";
import type { Group, GroupPostWithUser, GroupMemberBrief } from "../types";
import PostContent from "../components/PostContent";

export default function GroupDetail() {
  const { id } = useParams();
  const { user } = useAuth();
  const navigate = useNavigate();
  const [group, setGroup] = useState<Group | null>(null);
  const [posts, setPosts] = useState<GroupPostWithUser[]>([]);
  const [members, setMembers] = useState<GroupMemberBrief[]>([]);
  const [loading, setLoading] = useState(true);
  const [input, setInput] = useState("");
  const [isMember, setIsMember] = useState(false);
  const [isOwner, setIsOwner] = useState(false);
  const [showMembers, setShowMembers] = useState(false);

  const gid = parseInt(id || "0");

  useEffect(() => {
    const fetch = async () => {
      if (!gid) return;
      setLoading(true);
      try {
        const [g, p, m] = await Promise.all([
          api.groups.get(gid),
          api.groups.listPosts(gid),
          api.groups.members(gid),
        ]);
        setGroup(g);
        setPosts(p);
        setMembers(m);
        if (user) {
          setIsMember(m.some((mb) => mb.user_id === user.id));
          setIsOwner(g.owner_id === user.id);
        }
      } catch {
        // ignore
      } finally {
        setLoading(false);
      }
    };
    fetch();
  }, [gid, user]);

  const handleJoin = async () => {
    if (!user) return navigate("/login");
    try {
      if (isMember) {
        await api.groups.leave(gid, user.id);
        setIsMember(false);
        setGroup((prev) => prev ? { ...prev, member_count: prev.member_count - 1 } : prev);
      } else {
        await api.groups.join(gid, user.id);
        setIsMember(true);
        setGroup((prev) => prev ? { ...prev, member_count: prev.member_count + 1 } : prev);
        const m = await api.groups.members(gid);
        setMembers(m);
      }
    } catch (e: unknown) {
      alert(e instanceof Error ? e.message : "操作失敗");
    }
  };

  const postMsg = async () => {
    if (!user || !input.trim()) return;
    try {
      await api.groups.addPost(gid, user.id, input.trim());
      setInput("");
      const p = await api.groups.listPosts(gid);
      setPosts(p);
    } catch (e: unknown) {
      alert(e instanceof Error ? e.message : "發文失敗");
    }
  };

  const deletePost = async (postId: number) => {
    if (!user) return;
    if (!confirm("確定刪除貼文？")) return;
    try {
      await api.groups.deletePost(gid, postId, user.id);
      setPosts((prev) => prev.filter((p) => p.id !== postId));
    } catch (e: unknown) {
      alert(e instanceof Error ? e.message : "刪除失敗");
    }
  };

  if (loading) {
    return <div className="text-center py-20 text-gray-500">載入中...</div>;
  }
  if (!group) {
    return <div className="text-center py-20 text-gray-500">社團不存在</div>;
  }

  return (
    <div className="h-full flex flex-col">
      <div className="sticky top-0 z-10 bg-black/80 backdrop-blur border-b border-gray-800 px-4 py-3">
        <div className="flex items-center gap-3">
          <button onClick={() => navigate(-1)} className="text-white">
            <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M15 19l-7-7 7-7" />
            </svg>
          </button>
          <div className="flex-1">
            <h2 className="font-bold text-white text-sm">{group.name}</h2>
            <p className="text-xs text-gray-500">{group.member_count} 位成員</p>
          </div>
          {user && (
            <button
              onClick={handleJoin}
              className={`px-4 py-1.5 rounded-full text-sm font-bold transition ${
                isMember
                  ? "bg-transparent text-white border border-gray-600 hover:border-red-500 hover:text-red-500"
                  : "bg-white text-black hover:bg-gray-200"
              }`}
            >
              {isMember ? "退出" : "加入"}
            </button>
          )}
        </div>
      </div>

      {group.description && (
        <div className="px-4 py-3 border-b border-gray-800">
          <p className="text-sm text-gray-400">{group.description}</p>
        </div>
      )}

      <div className="px-4 py-2 border-b border-gray-800 flex gap-4 text-sm">
        <button onClick={() => setShowMembers(true)} className="text-gray-500 hover:text-white transition">
          <span className="font-bold text-white">{group.member_count}</span> 成員
        </button>
      </div>

      <div className="flex-1 overflow-y-auto">
        {posts.length === 0 ? (
          <div className="text-center py-20 text-gray-500">
            <p>尚無貼文</p>
            {isMember && <p className="text-sm mt-1">成為第一個發言的人！</p>}
          </div>
        ) : (
          <div className="divide-y divide-gray-800">
            {posts.map((p) => (
              <div key={p.id} className="px-4 py-3">
                <div className="flex items-center gap-2 mb-1">
                  <span className="font-bold text-white text-sm">{p.display_name}</span>
                  <span className="text-xs text-gray-500">@{p.username}</span>
                  <span className="text-xs text-gray-600">{new Date(p.created_at).toLocaleDateString("zh-TW")}</span>
                </div>
                <div className="text-white text-[15px]"><PostContent content={p.content} /></div>
                <div className="flex items-center gap-3 mt-2">
                  <span className="text-xs text-gray-500">{p.likes_count} 讚</span>
                  {(user?.id === p.user_id || isOwner) && (
                    <button onClick={() => deletePost(p.id)} className="text-xs text-red-500 hover:text-red-400 transition">
                      刪除
                    </button>
                  )}
                </div>
              </div>
            ))}
          </div>
        )}
      </div>

      {isMember && (
        <div className="sticky bottom-0 border-t border-gray-800 px-4 py-3">
          <div className="flex gap-2">
            <input
              type="text"
              placeholder="在社團中發言..."
              value={input}
              onChange={(e) => setInput(e.target.value)}
              onKeyDown={(e) => e.key === "Enter" && postMsg()}
              className="flex-1 bg-gray-900 text-white rounded-full px-4 py-2 text-sm placeholder-gray-500 outline-none focus:ring-1 focus:ring-gray-600"
            />
            <button
              onClick={postMsg}
              disabled={!input.trim()}
              className="px-4 py-2 rounded-full text-sm font-bold bg-white text-black hover:bg-gray-200 transition disabled:opacity-50"
            >
              發送
            </button>
          </div>
        </div>
      )}

      {showMembers && (
        <div className="fixed inset-0 z-50 bg-black/80 flex items-end" onClick={() => setShowMembers(false)}>
          <div className="w-full max-w-lg mx-auto bg-gray-900 rounded-t-2xl max-h-[60vh] flex flex-col" onClick={(e) => e.stopPropagation()}>
            <div className="flex items-center justify-between px-4 py-3 border-b border-gray-800">
              <h3 className="font-bold text-white">成員 ({members.length})</h3>
              <button onClick={() => setShowMembers(false)} className="text-gray-400">
                <svg className="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M6 18L18 6M6 6l12 12" />
                </svg>
              </button>
            </div>
            <div className="flex-1 overflow-y-auto">
              {members.map((m) => (
                <div key={m.user_id} className="flex items-center gap-3 px-4 py-3 hover:bg-gray-800 cursor-pointer"
                  onClick={() => { setShowMembers(false); navigate(`/users/${m.user_id}`); }}>
                  <div className="w-10 h-10 rounded-full bg-gray-700 flex items-center justify-center text-sm font-bold shrink-0">
                    {m.display_name[0]}
                  </div>
                  <div className="flex-1">
                    <div className="font-bold text-white text-sm">{m.display_name}</div>
                    <div className="text-xs text-gray-500">@{m.username}</div>
                  </div>
                  <span className="text-xs text-gray-500">{m.role === "owner" ? "創建者" : m.role === "admin" ? "管理員" : "成員"}</span>
                </div>
              ))}
            </div>
          </div>
        </div>
      )}
    </div>
  );
}
