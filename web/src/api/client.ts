const BASE = "/api";

let _token: string | null = null;

export function setToken(t: string | null) {
  _token = t;
}

export function getToken(): string | null {
  return _token;
}

async function request<T>(path: string, options?: RequestInit): Promise<T> {
  const headers: Record<string, string> = {
    "Content-Type": "application/json",
    ...(options?.headers as Record<string, string>),
  };
  if (_token) {
    headers["Authorization"] = `Bearer ${_token}`;
  }
  const res = await fetch(`${BASE}${path}`, { ...options, headers });
  if (!res.ok) {
    const body = await res.json().catch(() => ({ error: res.statusText }));
    throw new Error(body.error || "請求失敗");
  }
  return res.json();
}

export const api = {
  auth: {
    login: (username: string, password: string) =>
      request<import("../types/auth").AuthResponse>("/auth/login", {
        method: "POST",
        body: JSON.stringify({ username, password }),
      }),
    register: (username: string, password: string, display_name?: string) =>
      request<import("../types/auth").RegisterResponse>("/auth/register", {
        method: "POST",
        body: JSON.stringify({ username, password, display_name }),
      }),
    logout: (token: string) =>
      request<{ message: string }>("/auth/logout", {
        method: "POST",
        body: JSON.stringify({ token }),
      }),
  },
  users: {
    list: (params?: Record<string, string>, currentUserId?: number) =>
      request<import("../types").User[]>(`/users?${new URLSearchParams({ ...params, ...(currentUserId ? { current_user_id: String(currentUserId) } : {}) })}`),
    get: (id: number, currentUserId?: number) =>
      request<import("../types").UserDetail>(`/users/${id}?${currentUserId ? `current_user_id=${currentUserId}` : ''}`),
    create: (data: { username: string; display_name: string; bio?: string }) =>
      request<import("../types").User>("/users", {
        method: "POST",
        body: JSON.stringify(data),
      }),
    update: (id: number, data: { display_name?: string; bio?: string }) =>
      request<import("../types").User>(`/users/${id}`, {
        method: "PUT",
        body: JSON.stringify(data),
      }),
    delete: (id: number) => request<{ deleted: boolean }>(`/users/${id}`, { method: "DELETE" }),
    timeline: (id: number) => request<import("../types").PostWithUser[]>(`/users/${id}/timeline`),
    followers: (id: number) => request<import("../types").UserBrief[]>(`/users/${id}/followers`),
    following: (id: number) => request<import("../types").UserBrief[]>(`/users/${id}/following`),
  },
  posts: {
    list: (params?: Record<string, string>) =>
      request<import("../types").PostWithUser[]>(`/posts?${new URLSearchParams(params)}`),
    get: (id: number) => request<import("../types").PostDetail>(`/posts/${id}`),
    create: (data: { user_id: number; content: string }) =>
      request<import("../types").PostWithUser>("/posts", {
        method: "POST",
        body: JSON.stringify(data),
      }),
    reply: (id: number, data: { user_id: number; content: string }) =>
      request<import("../types").PostWithUser>(`/posts/${id}/reply`, {
        method: "POST",
        body: JSON.stringify(data),
      }),
    delete: (id: number) => request<{ deleted: boolean }>(`/posts/${id}`, { method: "DELETE" }),
  },
  follow: {
    add: (follower_id: number, followee_id: number) =>
      request<{ message: string }>("/follow", {
        method: "POST",
        body: JSON.stringify({ follower_id, followee_id }),
      }),
    remove: (follower_id: number, followee_id: number) =>
      request<{ message: string }>("/follow", {
        method: "DELETE",
        body: JSON.stringify({ follower_id, followee_id }),
      }),
  },
  messages: {
    send: (sender_id: number, receiver_id: number, content: string) =>
      request<{ id: number; message: string }>("/messages/send", {
        method: "POST",
        body: JSON.stringify({ sender_id, receiver_id, content }),
      }),
    conversations: (userId: number) =>
      request<import("../types").Conversation[]>(`/messages/${userId}/conversations`),
    messages: (userId: number, otherId: number) =>
      request<import("../types").MessageWithUser[]>(`/messages/${userId}/${otherId}`),
    unread: (userId: number) =>
      request<{ unread: number }>(`/messages/${userId}/unread`),
  },
  profiles: {
    get: (userId: number) =>
      request<{ profile: import("../types").Profile; tags: string[] }>(`/profiles/${userId}`),
    update: (userId: number, data: Record<string, string | number | null>) =>
      request<{ message: string }>(`/profiles/${userId}`, {
        method: "PUT",
        body: JSON.stringify(data),
      }),
    search: (params: Record<string, string>) =>
      request<{ results: import("../types").ProfileWithUser[]; count: number }>(
        `/profiles/search?${new URLSearchParams(params)}`
      ),
  },
  interests: {
    add: (userId: number, tag: string) =>
      request<{ id: number }>("/interests", {
        method: "POST",
        body: JSON.stringify({ user_id: userId, tag }),
      }),
    remove: (userId: number, tag: string) =>
      request<{ message: string }>("/interests", {
        method: "DELETE",
        body: JSON.stringify({ user_id: userId, tag }),
      }),
    list: (userId: number) =>
      request<{ interests: import("../types").Interest[] }>(`/interests/${userId}`),
  },
  block: {
    add: (blocker_id: number, blocked_id: number) =>
      request<{ message: string }>("/block", {
        method: "POST",
        body: JSON.stringify({ blocker_id, blocked_id }),
      }),
    remove: (blocker_id: number, blocked_id: number) =>
      request<{ message: string }>("/block", {
        method: "DELETE",
        body: JSON.stringify({ blocker_id, blocked_id }),
      }),
    list: (userId: number) =>
      request<import("../types").UserBrief[]>(`/block/${userId}`),
    check: (userId: number, otherId: number) =>
      request<{ blocked: boolean }>(`/block/${userId}/${otherId}`),
  },
  likes: {
    add: (user_id: number, post_id: number) =>
      request<{ message: string }>("/likes", {
        method: "POST",
        body: JSON.stringify({ user_id, post_id }),
      }),
    remove: (user_id: number, post_id: number) =>
      request<{ message: string }>("/likes", {
        method: "DELETE",
        body: JSON.stringify({ user_id, post_id }),
      }),
  },
  shops: {
    open: (userId: number, name: string, description?: string) =>
      request<{ shop: import("../types").Shop }>("/shops/open", {
        method: "POST",
        body: JSON.stringify({ user_id: userId, name, description }),
      }),
    my: (userId: number) =>
      request<import("../types").Shop>(`/shops?user_id=${userId}`),
    get: (id: number) =>
      request<import("../types").Shop>(`/shops/${id}`),
    update: (id: number, userId: number, data: { name?: string; description?: string }) =>
      request<import("../types").Shop>(`/shops/${id}`, {
        method: "PUT",
        body: JSON.stringify({ user_id: userId, ...data }),
      }),
    close: (userId: number) =>
      request<{ closed: boolean }>("/shops/close", {
        method: "POST",
        body: JSON.stringify({ user_id: userId }),
      }),
  },
  products: {
    add: (shopId: number, userId: number, data: { name: string; price: number; stock?: number; description?: string }) =>
      request<import("../types").Product>(`/products/shop/${shopId}`, {
        method: "POST",
        body: JSON.stringify({ user_id: userId, ...data }),
      }),
    listByShop: (shopId: number) =>
      request<import("../types").Product[]>(`/products/shop/${shopId}`),
    get: (id: number) =>
      request<import("../types").Product>(`/products/${id}`),
    update: (id: number, userId: number, data: { name?: string; price?: number; stock?: number; description?: string }) =>
      request<import("../types").Product>(`/products/${id}/update`, {
        method: "PUT",
        body: JSON.stringify({ user_id: userId, ...data }),
      }),
    remove: (id: number, userId: number) =>
      request<{ removed: boolean }>(`/products/${id}`, {
        method: "DELETE",
        body: JSON.stringify({ user_id: userId }),
      }),
    search: (params?: Record<string, string>) =>
      request<import("../types").ProductWithShop[]>(`/products/search?${new URLSearchParams(params)}`),
  },
  shopMessages: {
    send: (senderId: number, receiverId: number, shopId: number, content: string) =>
      request<import("../types").ShopMessageWithUser>("/shop-messages/send", {
        method: "POST",
        body: JSON.stringify({ sender_id: senderId, receiver_id: receiverId, shop_id: shopId, content }),
      }),
    list: (shopId: number, userId: number, otherId: number) =>
      request<import("../types").ShopMessageWithUser[]>(`/shop-messages/${shopId}?user_id=${userId}&other_id=${otherId}`),
    conversations: (userId: number) =>
      request<import("../types").ShopConversation[]>(`/shop-messages/conversations?user_id=${userId}`),
  },
  orders: {
    create: (buyerId: number, productId: number, quantity?: number) =>
      request<import("../types").Order>("/orders", {
        method: "POST",
        body: JSON.stringify({ buyer_id: buyerId, product_id: productId, quantity: quantity ?? 1 }),
      }),
    list: (userId: number) =>
      request<import("../types").OrderWithDetails[]>(`/orders?user_id=${userId}`),
    get: (id: number, userId: number) =>
      request<import("../types").Order>(`/orders/${id}?user_id=${userId}`),
  },
};
