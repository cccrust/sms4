import { useState, useEffect } from "react";
import { useNavigate } from "react-router-dom";
import { api } from "../api/client";
import type { Conversation } from "../types";

export default function Messages() {
  const navigate = useNavigate();
  const [convs, setConvs] = useState<Conversation[]>([]);
  const [users, setUsers] = useState<{ id: number; username: string; display_name: string }[]>([]);
  const [currentUserId, setCurrentUserId] = useState<number | null>(null);
  const [loading, setLoading] = useState(true);
  const [unread, setUnread] = useState(0);

  useEffect(() => {
    const init = async () => {
      try {
        const userList = await api.users.list();
        setUsers(userList);
        if (userList.length > 0) {
          const uid = userList[0].id;
          setCurrentUserId(uid);
          const [convsData, unreadData] = await Promise.all([
            api.messages.conversations(uid),
            api.messages.unread(uid),
          ]);
          setConvs(convsData);
          setUnread(unreadData.unread);
        }
      } catch {
        // ignore
      } finally {
        setLoading(false);
      }
    };
    init();
  }, []);

  const [selectedUserId, setSelectedUserId] = useState<number | null>(null);
  const [showNew, setShowNew] = useState(false);

  const startConversation = (otherId: number) => {
    navigate(`/messages/${otherId}?uid=${currentUserId}`);
  };

  if (loading) {
    return <div className="text-center py-20 text-gray-500">載入中...</div>;
  }

  return (
    <div>
      <div className="px-4 py-3 border-b border-gray-800 flex items-center justify-between">
        <h2 className="font-bold text-white">私訊</h2>
        <div className="flex items-center gap-3">
          {unread > 0 && (
            <span className="text-xs text-red-500">{unread} 未讀</span>
          )}
          <button
            onClick={() => setShowNew(true)}
            className="text-blue-500 text-sm font-bold"
          >
            新訊息
          </button>
        </div>
      </div>

      {currentUserId != null && (
        <div className="px-4 py-2 border-b border-gray-800 flex gap-2 overflow-x-auto">
          {users.map((u) => (
            <button
              key={u.id}
              onClick={() => setCurrentUserId(u.id)}
              className={`shrink-0 text-xs px-3 py-1 rounded-full border transition ${
                currentUserId === u.id
                  ? "bg-white text-black border-white"
                  : "bg-transparent text-gray-400 border-gray-700"
              }`}
            >
              @{u.username}
            </button>
          ))}
        </div>
      )}

      {convs.length === 0 ? (
        <div className="text-center py-20 text-gray-500">
          <p className="text-lg">尚無對話</p>
          <p className="text-sm mt-1">點擊「新訊息」開始聊天</p>
        </div>
      ) : (
        convs.map((c) => (
          <div
            key={c.other_user_id}
            onClick={() => startConversation(c.other_user_id)}
            className="border-b border-gray-800 px-4 py-3 hover:bg-gray-900/50 transition cursor-pointer"
          >
            <div className="flex items-center gap-3">
              <div className="w-12 h-12 rounded-full bg-gray-700 flex items-center justify-center text-lg font-bold shrink-0">
                {c.other_display_name[0]}
              </div>
              <div className="flex-1 min-w-0">
                <div className="flex items-center gap-2">
                  <span className="font-bold text-white text-sm">{c.other_display_name}</span>
                  {c.unread_count > 0 && (
                    <span className="bg-blue-500 text-white text-xs rounded-full px-1.5 py-0.5">
                      {c.unread_count}
                    </span>
                  )}
                </div>
                <div className="text-sm text-gray-500 truncate">{c.last_message}</div>
              </div>
              <span className="text-xs text-gray-600 shrink-0">
                {new Date(c.last_message_at).toLocaleDateString("zh-TW", { month: "short", day: "numeric" })}
              </span>
            </div>
          </div>
        ))
      )}

      {showNew && (
        <div className="fixed inset-0 z-50 bg-black/80 flex items-end" onClick={() => setShowNew(false)}>
          <div
            className="w-full max-w-lg mx-auto bg-gray-900 rounded-t-2xl max-h-[60vh] flex flex-col"
            onClick={(e) => e.stopPropagation()}
          >
            <div className="flex items-center justify-between px-4 py-3 border-b border-gray-800">
              <h3 className="font-bold text-white">新訊息</h3>
              <button onClick={() => setShowNew(false)} className="text-gray-400">
                <svg className="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M6 18L18 6M6 6l12 12" />
                </svg>
              </button>
            </div>
            <div className="flex-1 overflow-y-auto">
              {users
                .filter((u) => u.id !== currentUserId)
                .map((u) => (
                  <div
                    key={u.id}
                    className="flex items-center gap-3 px-4 py-3 hover:bg-gray-800 cursor-pointer"
                    onClick={() => { setShowNew(false); startConversation(u.id); }}
                  >
                    <div className="w-10 h-10 rounded-full bg-gray-700 flex items-center justify-center text-sm font-bold shrink-0">
                      {u.display_name[0]}
                    </div>
                    <div>
                      <div className="font-bold text-white text-sm">{u.display_name}</div>
                      <div className="text-xs text-gray-500">@{u.username}</div>
                    </div>
                  </div>
                ))}
            </div>
          </div>
        </div>
      )}
    </div>
  );
}
