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
import Marketplace from "./pages/Marketplace";
import MyShop from "./pages/MyShop";
import ShopPage from "./pages/ShopPage";
import Orders from "./pages/Orders";
import ShopMessages from "./pages/ShopMessages";
import ShopMessageList from "./pages/ShopMessageList";
import CartPage from "./pages/CartPage";

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
              <Route path="marketplace" element={<Marketplace />} />
              <Route path="shop/:id" element={<ShopPage />} />
              <Route path="my-shop" element={<MyShop />} />
              <Route path="orders" element={<Orders />} />
              <Route path="shop-messages" element={<ShopMessageList />} />
              <Route path="shop-messages/:shopId" element={<ShopMessages />} />
              <Route path="cart" element={<CartPage />} />
            </Route>
          </Route>
        </Routes>
      </AuthProvider>
    </BrowserRouter>
  );
}
