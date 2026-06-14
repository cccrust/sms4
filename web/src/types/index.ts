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

export interface MessageWithUser {
  id: number;
  sender_id: number;
  receiver_id: number;
  content: string;
  read: number;
  created_at: string;
  sender_username: string;
  sender_display_name: string;
  receiver_username: string;
  receiver_display_name: string;
}

export interface Conversation {
  other_user_id: number;
  other_username: string;
  other_display_name: string;
  last_message: string;
  last_message_at: string;
  unread_count: number;
}

export interface Profile {
  user_id: number;
  birthday: string | null;
  gender: string | null;
  city: string | null;
  occupation: string | null;
  education: string | null;
  height: number | null;
  looking_for: string | null;
  about_me: string | null;
  updated_at: string;
}

export interface ProfileWithUser {
  user_id: number;
  username: string;
  display_name: string;
  bio: string | null;
  birthday: string | null;
  gender: string | null;
  city: string | null;
  occupation: string | null;
  education: string | null;
  height: number | null;
  looking_for: string | null;
  about_me: string | null;
  tags: string[];
  age: number | null;
}

export interface ShopMessageWithUser {
  id: number;
  shop_id: number;
  shop_name: string;
  sender_id: number;
  sender_username: string;
  sender_display_name: string;
  receiver_id: number;
  receiver_username: string;
  receiver_display_name: string;
  content: string;
  created_at: string;
}

export interface CartItemWithDetails {
  id: number;
  user_id: number;
  product_id: number;
  product_name: string;
  price: number;
  stock: number;
  quantity: number;
  total_price: number;
  shop_id: number;
  shop_name: string;
  created_at: string;
}

export interface ShopConversation {
  shop_id: number;
  shop_name: string;
  other_id: number;
  last_message: string | null;
  last_message_at: string | null;
}

export interface Interest {
  id: number;
  user_id: number;
  tag: string;
}

export interface Shop {
  id: number;
  user_id: number;
  name: string;
  description: string | null;
  created_at: string;
  updated_at: string;
}

export interface Product {
  id: number;
  shop_id: number;
  name: string;
  description: string | null;
  price: number;
  stock: number;
  image: string | null;
  created_at: string;
  updated_at: string;
}

export interface ProductWithShop {
  id: number;
  shop_id: number;
  shop_name: string;
  shop_user_id: number;
  name: string;
  description: string | null;
  price: number;
  stock: number;
  image: string | null;
  created_at: string;
}

export interface Order {
  id: number;
  buyer_id: number;
  product_id: number;
  quantity: number;
  total_price: number;
  status: string;
  created_at: string;
  updated_at: string;
}

export interface OrderWithDetails {
  id: number;
  buyer_id: number;
  product_id: number;
  product_name: string;
  shop_name: string;
  shop_user_id: number;
  quantity: number;
  total_price: number;
  status: string;
  created_at: string;
}
