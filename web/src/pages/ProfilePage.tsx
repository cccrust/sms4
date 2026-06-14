import { useState, useEffect } from "react";
import { useParams, Link, useNavigate } from "react-router-dom";
import { api } from "../api/client";
import { useAuth } from "../contexts/AuthContext";
import type { Profile } from "../types";

export default function ProfilePage() {
  const { id } = useParams<{ id: string }>();
  const { user: authUser } = useAuth();
  const navigate = useNavigate();
  const [profile, setProfile] = useState<Profile | null>(null);
  const [tags, setTags] = useState<string[]>([]);
  const [user, setUser] = useState<{ id: number; username: string; display_name: string; bio: string | null } | null>(null);
  const [loading, setLoading] = useState(true);
  const [isFollowing, setIsFollowing] = useState(false);
  const [isBlocked, setIsBlocked] = useState(false);
  const [followersCount, setFollowersCount] = useState(0);
  const [followingCount, setFollowingCount] = useState(0);

  useEffect(() => {
    if (!id) return;
    const fetch = async () => {
      try {
        const uid = parseInt(id);
        const [userData, profileData, followersData] = await Promise.all([
          api.users.get(uid, authUser?.id),
          api.profiles.get(uid).catch(() => null),
          api.users.followers(uid).catch(() => [] as { id: number }[]),
        ]);
        setUser(userData);
        setFollowersCount(userData.followers_count);
        setFollowingCount(userData.following_count);
        if (authUser) {
          setIsFollowing(followersData.some((f: { id: number }) => f.id === authUser.id));
          const blockCheck = await api.block.check(authUser.id, uid).catch(() => ({ blocked: false }));
          setIsBlocked(blockCheck.blocked);
        }
        if (profileData) {
          setProfile(profileData.profile);
          setTags(profileData.tags);
        }
      } catch {
        // ignore
      } finally {
        setLoading(false);
      }
    };
    fetch();
  }, [id, authUser]);

  const handleFollow = async () => {
    if (!authUser || !user) return;
    if (isFollowing) {
      await api.follow.remove(authUser.id, user.id);
      setIsFollowing(false);
      setFollowersCount((c) => c - 1);
    } else {
      await api.follow.add(authUser.id, user.id);
      setIsFollowing(true);
      setFollowersCount((c) => c + 1);
    }
  };

  if (loading) return <div className="text-center py-20 text-gray-500">載入中...</div>;
  if (!user) return <div className="text-center py-20 text-gray-500">使用者不存在</div>;

  const age = profile?.birthday
    ? new Date().getFullYear() - new Date(profile.birthday).getFullYear()
    : null;

  return (
    <div className="px-4 py-6">
      <div className="flex items-center gap-4 mb-4">
        <div className="w-16 h-16 rounded-full bg-gray-700 flex items-center justify-center text-2xl font-bold shrink-0">
          {user.display_name[0]}
        </div>
        <div className="flex-1 min-w-0">
          <h2 className="text-xl font-bold text-white">{user.display_name}</h2>
          <p className="text-sm text-gray-500">@{user.username}</p>
          {user.bio && <p className="text-sm text-gray-400 mt-1">{user.bio}</p>}
        </div>
        {authUser && authUser.id !== user.id && (
          <div className="flex items-center gap-2 shrink-0">
            <button
              onClick={() => navigate(`/messages/${user.id}?uid=${authUser.id}`)}
              className="px-4 py-1.5 rounded-full text-sm font-bold bg-blue-500 text-white hover:bg-blue-600 transition"
            >
              傳送訊息
            </button>
            <button
              onClick={handleFollow}
              className={`px-4 py-1.5 rounded-full text-sm font-bold transition ${
                isFollowing
                  ? "bg-transparent text-white border border-gray-600 hover:border-red-500 hover:text-red-500"
                  : "bg-white text-black hover:bg-gray-200"
              }`}
            >
              {isFollowing ? "追蹤中" : "追蹤"}
            </button>
            <button
              onClick={async () => {
                if (isBlocked) {
                  await api.block.remove(authUser.id, user.id);
                  setIsBlocked(false);
                } else {
                  await api.block.add(authUser.id, user.id);
                  setIsBlocked(true);
                }
              }}
              className={`shrink-0 px-4 py-1.5 rounded-full text-sm font-bold transition ${
                isBlocked
                  ? "bg-red-500 text-white hover:bg-red-600"
                  : "bg-transparent text-gray-500 border border-gray-700 hover:border-red-500 hover:text-red-500"
              }`}
            >
              {isBlocked ? "已封鎖" : "封鎖"}
            </button>
          </div>
        )}
      </div>

      <div className="flex gap-4 text-sm mb-4">
        <span className="text-gray-500">
          <span className="font-bold text-white">{followingCount}</span> 追蹤中
        </span>
        <span className="text-gray-500">
          <span className="font-bold text-white">{followersCount}</span> 粉絲
        </span>
      </div>

      {!profile ? (
        <div className="text-center py-10">
          <p className="text-gray-500 mb-4">該使用者尚未填寫交友資料</p>
          <Link
            to="/profile/edit"
            className="text-blue-500 text-sm font-bold"
          >
            前往填寫
          </Link>
        </div>
      ) : (
        <div className="space-y-4">
          <div className="bg-gray-900 rounded-xl p-4 space-y-3">
            <h3 className="font-bold text-white text-sm">基本資料</h3>
            {profile.gender && (
              <div className="flex justify-between text-sm">
                <span className="text-gray-500">性別</span>
                <span className="text-white">{profile.gender === "male" ? "男" : profile.gender === "female" ? "女" : profile.gender}</span>
              </div>
            )}
            {age && (
              <div className="flex justify-between text-sm">
                <span className="text-gray-500">年齡</span>
                <span className="text-white">{age} 歲</span>
              </div>
            )}
            {profile.city && (
              <div className="flex justify-between text-sm">
                <span className="text-gray-500">城市</span>
                <span className="text-white">{profile.city}</span>
              </div>
            )}
            {profile.occupation && (
              <div className="flex justify-between text-sm">
                <span className="text-gray-500">職業</span>
                <span className="text-white">{profile.occupation}</span>
              </div>
            )}
            {profile.education && (
              <div className="flex justify-between text-sm">
                <span className="text-gray-500">學歷</span>
                <span className="text-white">{profile.education}</span>
              </div>
            )}
            {profile.height && (
              <div className="flex justify-between text-sm">
                <span className="text-gray-500">身高</span>
                <span className="text-white">{profile.height} cm</span>
              </div>
            )}
            {profile.looking_for && (
              <div className="flex justify-between text-sm">
                <span className="text-gray-500">交友目的</span>
                <span className="text-white">
                  {profile.looking_for === "friend" ? "交朋友" : profile.looking_for === "date" ? "約會" : profile.looking_for}
                </span>
              </div>
            )}
          </div>

          {profile.about_me && (
            <div className="bg-gray-900 rounded-xl p-4">
              <h3 className="font-bold text-white text-sm mb-2">關於我</h3>
              <p className="text-sm text-gray-300">{profile.about_me}</p>
            </div>
          )}

          {tags.length > 0 && (
            <div className="bg-gray-900 rounded-xl p-4">
              <h3 className="font-bold text-white text-sm mb-2">興趣</h3>
              <div className="flex flex-wrap gap-2">
                {tags.map((tag) => (
                  <Link
                    key={tag}
                    to={`/search?tags=${tag}`}
                    className="bg-gray-800 text-gray-300 text-xs px-3 py-1 rounded-full hover:bg-gray-700 transition"
                  >
                    {tag}
                  </Link>
                ))}
              </div>
            </div>
          )}
        </div>
      )}
    </div>
  );
}
