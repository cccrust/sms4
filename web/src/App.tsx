import { BrowserRouter, Routes, Route } from "react-router-dom";
import { AuthProvider } from "./contexts/AuthContext";
import ProtectedRoute from "./components/ProtectedRoute";
import Layout from "./components/Layout";
import LoginPage from "./pages/LoginPage";
import RegisterPage from "./pages/RegisterPage";
import Timeline from "./pages/Timeline";
import Messages from "./pages/Messages";
import Conversation from "./pages/Conversation";
import ProfilePage from "./pages/ProfilePage";
import ProfileEdit from "./pages/ProfileEdit";
import MatchSearch from "./pages/MatchSearch";
import UserList from "./pages/UserList";
import UserDetail from "./pages/UserDetail";
import PostDetail from "./pages/PostDetail";

export default function App() {
  return (
    <BrowserRouter>
      <AuthProvider>
        <Routes>
          <Route path="/login" element={<LoginPage />} />
          <Route path="/register" element={<RegisterPage />} />
          <Route element={<ProtectedRoute />}>
            <Route element={<Layout />}>
              <Route index element={<Timeline />} />
              <Route path="messages" element={<Messages />} />
              <Route path="messages/:otherId" element={<Conversation />} />
              <Route path="profile/:id" element={<ProfilePage />} />
              <Route path="profile/edit" element={<ProfileEdit />} />
              <Route path="search" element={<MatchSearch />} />
              <Route path="users" element={<UserList />} />
              <Route path="users/:id" element={<UserDetail />} />
              <Route path="posts/:id" element={<PostDetail />} />
            </Route>
          </Route>
        </Routes>
      </AuthProvider>
    </BrowserRouter>
  );
}
