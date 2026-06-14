import { createContext, useContext, useState, useEffect, type ReactNode } from "react";
import type { User } from "../types";
import { api } from "../api/client";

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

function storeToken(token: string | null) {
  if (token) {
    localStorage.setItem("sms4_token", token);
  } else {
    localStorage.removeItem("sms4_token");
  }
}

export function AuthProvider({ children }: { children: ReactNode }) {
  const [user, setUser] = useState<User | null>(null);
  const [token, setToken] = useState<string | null>(getStoredToken());

  useEffect(() => {
    api.setToken(token);
  }, [token]);

  const login = async (username: string, password: string) => {
    const res = await api.auth.login(username, password);
    storeToken(res.token);
    setToken(res.token);
    setUser(res.user);
  };

  const register = async (username: string, password: string, displayName?: string) => {
    await api.auth.register(username, password, displayName);
  };

  const logout = () => {
    if (token) {
      api.auth.logout(token).catch(() => {});
    }
    storeToken(null);
    setToken(null);
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
