import { createContext, useContext, useState, useEffect, type ReactNode } from "react";
import type { User } from "../types";
import { api, setToken as setApiToken } from "../api/client";

interface AuthContextType {
  user: User | null;
  token: string | null;
  isAuthenticated: boolean;
  login: (username: string, password: string) => Promise<void>;
  register: (username: string, password: string, displayName?: string) => Promise<void>;
  logout: () => void;
}

const AuthContext = createContext<AuthContextType | null>(null);

function getStoredToken(): string | null {
  try {
    return localStorage.getItem("sms4_token");
  } catch {
    return null;
  }
}

function getStoredUser(): User | null {
  try {
    const raw = localStorage.getItem("sms4_user");
    return raw ? JSON.parse(raw) : null;
  } catch {
    return null;
  }
}

function storeSession(token: string | null, user: User | null) {
  if (token && user) {
    localStorage.setItem("sms4_token", token);
    localStorage.setItem("sms4_user", JSON.stringify(user));
  } else {
    localStorage.removeItem("sms4_token");
    localStorage.removeItem("sms4_user");
  }
}

export function AuthProvider({ children }: { children: ReactNode }) {
  const [user, setUser] = useState<User | null>(getStoredUser());
  const [token, setTokenState] = useState<string | null>(getStoredToken());

  useEffect(() => {
    setApiToken(token);
  }, [token]);

  const login = async (username: string, password: string) => {
    const res = await api.auth.login(username, password);
    storeSession(res.token, res.user);
    setTokenState(res.token);
    setUser(res.user);
  };

  const register = async (username: string, password: string, displayName?: string) => {
    await api.auth.register(username, password, displayName);
  };

  const logout = () => {
    if (token) {
      api.auth.logout(token).catch(() => {});
    }
    storeSession(null, null);
    setTokenState(null);
    setUser(null);
  };

  return (
    <AuthContext.Provider value={{ user, token, isAuthenticated: !!token && !!user, login, register, logout }}>
      {children}
    </AuthContext.Provider>
  );
}

export function useAuth() {
  const ctx = useContext(AuthContext);
  if (!ctx) throw new Error("useAuth must be used within AuthProvider");
  return ctx;
}
