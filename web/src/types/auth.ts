export interface AuthResponse {
  token: string;
  user: import("./index").User;
}

export interface RegisterResponse {
  user: import("./index").User;
}
