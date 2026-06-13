import { BrowserRouter, Routes, Route } from "react-router-dom";
import Layout from "./components/Layout";
import Timeline from "./pages/Timeline";
import UserList from "./pages/UserList";
import UserDetail from "./pages/UserDetail";
import PostDetail from "./pages/PostDetail";

export default function App() {
  return (
    <BrowserRouter>
      <Routes>
        <Route element={<Layout />}>
          <Route index element={<Timeline />} />
          <Route path="users" element={<UserList />} />
          <Route path="users/:id" element={<UserDetail />} />
          <Route path="posts/:id" element={<PostDetail />} />
        </Route>
      </Routes>
    </BrowserRouter>
  );
}
