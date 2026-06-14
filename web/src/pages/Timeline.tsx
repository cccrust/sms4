import { useState, useEffect, useCallback } from "react";
import { api } from "../api/client";
import type { PostWithUser } from "../types";
import { useAuth } from "../contexts/AuthContext";
import PostCard from "../components/PostCard";
import PostForm from "../components/PostForm";

export default function Timeline() {
  const { user } = useAuth();
  const [posts, setPosts] = useState<PostWithUser[]>([]);
  const [loading, setLoading] = useState(true);
  const [likedPosts, setLikedPosts] = useState<Set<number>>(new Set());

  const fetchPosts = useCallback(async () => {
    try {
      const data = await api.posts.list();
      setPosts(data);
    } catch {
      // ignore
    } finally {
      setLoading(false);
    }
  }, []);

  useEffect(() => {
    fetchPosts();
  }, [fetchPosts]);

  const handlePost = async (content: string) => {
    if (!user) return;
    const newPost = await api.posts.create({ user_id: user.id, content });
    setPosts((prev) => [newPost, ...prev]);
  };

  const handleLike = async (postId: number) => {
    if (!user) return;
    if (likedPosts.has(postId)) {
      await api.likes.remove(user.id, postId);
      setLikedPosts((prev) => { const n = new Set(prev); n.delete(postId); return n; });
      setPosts((prev) => prev.map((p) => p.id === postId ? { ...p, likes_count: p.likes_count - 1 } : p));
    } else {
      await api.likes.add(user.id, postId);
      setLikedPosts((prev) => { const n = new Set(prev); n.add(postId); return n; });
      setPosts((prev) => prev.map((p) => p.id === postId ? { ...p, likes_count: p.likes_count + 1 } : p));
    }
  };

  const handleReply = (postId: number) => {
    window.location.href = `/posts/${postId}`;
  };

  if (loading) {
    return (
      <div className="flex items-center justify-center py-20 text-gray-500">
        載入中...
      </div>
    );
  }

  return (
    <div>
      <PostForm onSubmit={handlePost} />
      {user && (
        <div className="px-4 py-2 text-xs text-gray-500 border-b border-gray-800">
          以 @{user.username} 發布
        </div>
      )}
      {posts.length === 0 ? (
        <div className="text-center py-20 text-gray-500">
          <p className="text-lg">尚無貼文</p>
          <p className="text-sm mt-1">成為第一個發布貼文的人！</p>
        </div>
      ) : (
        posts.map((post) => (
          <PostCard
            key={post.id}
            post={post}
            showThread
            onLike={() => handleLike(post.id)}
            onReply={() => handleReply(post.id)}
            liked={likedPosts.has(post.id)}
          />
        ))
      )}
    </div>
  );
}
