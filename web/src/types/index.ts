export interface User {
  id: number;
  username: string;
  display_name: string;
  bio: string | null;
  avatar: string | null;
  created_at: string;
  updated_at: string;
}

export interface UserDetail {
  id: number;
  username: string;
  display_name: string;
  bio: string | null;
  avatar: string | null;
  followers_count: number;
  following_count: number;
  created_at: string;
  updated_at: string;
}

export interface PostWithUser {
  id: number;
  content: string;
  parent_id: number | null;
  likes_count: number;
  replies_count: number;
  created_at: string;
  user_id: number;
  username: string;
  display_name: string;
}

export interface PostDetail {
  post: PostWithUser;
  replies: PostWithUser[];
}

export interface UserBrief {
  id: number;
  username: string;
  display_name: string;
}
