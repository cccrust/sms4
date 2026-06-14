# SMS4 — 手機社群軟體 (Threads 風格)

Rust + SQLite 後端 + React + TypeScript 前端的社群平台，支援 CLI 和 Web 雙模式操作。

## 功能一覽

| 功能 | CLI | Web API | 前端頁面 |
|------|:---:|:-------:|:--------:|
| 使用者註冊/登入 | ✅ | ✅ | ✅ |
| 貼文發布/回覆/刪除 | ✅ | ✅ | ✅ |
| 追蹤/取消追蹤 | ✅ | ✅ | ✅ |
| 按讚/取消讚 | ✅ | ✅ | ✅ |
| 私訊 (1 對 1) | ✅ | ✅ | ✅ |
| 個人檔案設定 | ✅ | ✅ | ✅ |
| 興趣標籤 | ✅ | ✅ | ✅ |
| 交友配對搜尋 | ✅ | ✅ | ✅ |
| 使用者封鎖 | ✅ | ✅ | ✅ |
| 個人商店 | ✅ | ✅ | ✅ |
| 商品管理 | ✅ | ✅ | ✅ |
| 訂單系統 | ✅ | ✅ | — |
| 商店私訊 | — | ✅ | ✅ |

## 快速開始

```bash
# 初始化資料庫
cargo run -- init

# Web 模式（開發，前端由 Vite proxy）
cd web && npm run dev
# 另開終端機：
SMS4_DB=sms4-dev.db cargo run -- web --dev

# CLI 操作
cargo run -- user add alice 愛麗絲 --password alice123
cargo run -- auth login alice alice123
cargo run -- post add 1 "Hello, SMS4!"
```

### 生產模式

```bash
cd web && npm run build
SMS4_DB=sms4.db cargo run -- web
# 瀏覽器開啟 http://127.0.0.1:8080
```

### 填入測試資料

```bash
bash seed.sh
```

## 目錄結構

```
sms4/
├── src/
│   ├── main.rs              # 程式進入點
│   ├── db.sql               # 資料庫 Schema（include_str!）
│   ├── db.rs                # 資料庫初始化
│   ├── cli/                 # CLI 指令處理
│   │   ├── mod.rs           # Clap 指令定義
│   │   ├── auth.rs          # 認證
│   │   ├── block.rs         # 封鎖
│   │   ├── follow.rs        # 追蹤
│   │   ├── fmt.rs           # 輸出格式化（colored）
│   │   ├── interest.rs      # 興趣標籤
│   │   ├── like.rs          # 按讚
│   │   ├── message.rs       # 私訊
│   │   ├── order.rs         # 訂單
│   │   ├── post.rs          # 貼文
│   │   ├── product.rs       # 商品
│   │   ├── profile.rs       # 交友檔案
│   │   ├── shop.rs          # 商店
│   │   └── user.rs          # 使用者
│   ├── model/               # 資料層
│   │   ├── mod.rs
│   │   ├── auth.rs
│   │   ├── block.rs
│   │   ├── follow.rs
│   │   ├── interest.rs
│   │   ├── like.rs
│   │   ├── message.rs
│   │   ├── order.rs
│   │   ├── post.rs
│   │   ├── product.rs
│   │   ├── profile.rs
│   │   ├── shop.rs
│   │   ├── shop_message.rs
│   │   └── user.rs
│   └── web/                 # Web API (axum)
│       ├── mod.rs           # App 建構、靜態檔案服務
│       ├── error.rs         # 錯誤處理
│       └── routes/          # API 路由處理器
│           ├── mod.rs
│           ├── auth.rs
│           ├── block.rs
│           ├── follow.rs
│           ├── interest.rs
│           ├── like.rs
│           ├── message.rs
│           ├── order.rs
│           ├── post.rs
│           ├── product.rs
│           ├── profile.rs
│           ├── shop.rs
│           ├── shop_message.rs
│           └── user.rs
├── web/                     # React 前端
│   ├── src/
│   │   ├── main.tsx
│   │   ├── App.tsx          # 路由
│   │   ├── api/client.ts    # API 客戶端
│   │   ├── types/           # TypeScript 型別
│   │   ├── contexts/        # React Context
│   │   ├── components/      # 共用元件
│   │   └── pages/           # 頁面元件
│   ├── index.html
│   ├── vite.config.ts
│   └── package.json
├── _doc/                    # 版本文件
├── seed.sh                  # 測試資料腳本
├── AGENTS.md                # AI 輔助開發備忘
└── README.md
```

## CLI 指令

```
sms4 init                              # 初始化資料庫
sms4 user add <username> <name> [--bio] [--password]   # 新增使用者
sms4 user list                        # 列出使用者
sms4 user get <id>                    # 檢視使用者
sms4 user update <id> [--bio]         # 更新使用者
sms4 user delete <id>                 # 刪除使用者
sms4 auth login <username> <password> # 登入
sms4 auth logout <token>              # 登出
sms4 post add <user_id> <content>     # 發布貼文
sms4 post reply <post_id> <user_id> <content>  # 回覆貼文
sms4 post list                        # 列出貼文
sms4 post get <id>                    # 檢視貼文（含回覆）
sms4 post timeline <user_id>          # 時間軸
sms4 post delete <id>                 # 刪除貼文
sms4 follow add <follower> <followee> # 追蹤
sms4 follow remove <follower> <followee>  # 取消追蹤
sms4 follow followers <user_id>       # 粉絲列表
sms4 follow following <user_id>       # 追蹤中列表
sms4 like add <user_id> <post_id>     # 按讚
sms4 like remove <user_id> <post_id>  # 取消讚
sms4 msg send <sender> <receiver> <content>  # 傳送私訊
sms4 msg list <user1> <user2>         # 檢視對話
sms4 msg conversations <user_id>      # 對話列表
sms4 profile set <user_id> [--age] [--city] ...  # 設定交友檔案
sms4 profile get <user_id>            # 檢視交友檔案
sms4 profile search [--city] [--tags] # 搜尋配對
sms4 interest add <user_id> <tag>     # 新增興趣
sms4 interest remove <user_id> <tag>  # 移除興趣
sms4 interest list <user_id>          # 列出興趣
sms4 block add <blocker> <blocked>    # 封鎖使用者
sms4 block remove <blocker> <blocked> # 解除封鎖
sms4 block list <user_id>             # 列出封鎖名單
sms4 shop open <user_id> <name> [--description]  # 開店
sms4 shop show <user_id>              # 檢視商店
sms4 shop update <user_id> [--name] [--description]  # 更新商店
sms4 shop close <user_id>             # 關店
sms4 product add <shop_id> <name> <price> [--stock] [--description]  # 上架商品
sms4 product list <shop_id>           # 列出商品
sms4 product update <id> [--name] [--price] [--stock]  # 更新商品
sms4 product remove <id>              # 刪除商品
sms4 product search [--q] [--min-price] [--max-price]  # 搜尋商品
sms4 order buy <product_id> <buyer_id> [--quantity]  # 下單
sms4 order list <user_id>             # 訂單列表
sms4 web [--port 8080] [--host 127.0.0.1] [--dev]  # 啟動 Web 伺服器
```

## Web API

所有 API 路徑前綴為 `/api`。

### 認證
| 方法 | 路徑 | 說明 |
|------|------|------|
| POST | `/api/auth/register` | 註冊 |
| POST | `/api/auth/login` | 登入 |
| POST | `/api/auth/logout` | 登出 |

### 使用者
| 方法 | 路徑 | 說明 |
|------|------|------|
| GET | `/api/users` | 列出使用者（支援 `search`, `current_user_id`） |
| POST | `/api/users` | 建立使用者 |
| GET | `/api/users/{id}` | 檢視使用者（支援 `current_user_id`） |
| PUT | `/api/users/{id}` | 更新使用者 |
| DELETE | `/api/users/{id}` | 刪除使用者 |
| GET | `/api/users/{id}/timeline` | 時間軸 |
| GET | `/api/users/{id}/followers` | 粉絲 |
| GET | `/api/users/{id}/following` | 追蹤中 |

### 貼文
| 方法 | 路徑 | 說明 |
|------|------|------|
| GET | `/api/posts` | 列出貼文 |
| POST | `/api/posts` | 發布貼文 |
| GET | `/api/posts/{id}` | 檢視貼文（含回覆） |
| DELETE | `/api/posts/{id}` | 刪除貼文 |
| POST | `/api/posts/{id}/reply` | 回覆貼文 |

### 追蹤 / 按讚 / 封鎖
| 方法 | 路徑 | 說明 |
|------|------|------|
| POST | `/api/follow` | 追蹤 |
| DELETE | `/api/follow` | 取消追蹤 |
| POST | `/api/likes` | 按讚 |
| DELETE | `/api/likes` | 取消讚 |
| POST | `/api/block` | 封鎖 |
| DELETE | `/api/block` | 解除封鎖 |
| GET | `/api/block/{user_id}` | 封鎖列表 |
| GET | `/api/block/{user_id}/{other_id}` | 檢查封鎖狀態 |

### 私訊
| 方法 | 路徑 | 說明 |
|------|------|------|
| POST | `/api/messages/send` | 傳送私訊 |
| GET | `/api/messages/{user_id}/conversations` | 對話列表 |
| GET | `/api/messages/{user_id}/unread` | 未讀數量 |
| GET | `/api/messages/{user_id}/{other_id}` | 對話內容 |

### 交友 / 興趣
| 方法 | 路徑 | 說明 |
|------|------|------|
| GET | `/api/profiles/{user_id}` | 檢視交友檔案 |
| PUT | `/api/profiles/{user_id}` | 更新交友檔案 |
| GET | `/api/profiles/search` | 搜尋配對 |
| POST | `/api/interests` | 新增興趣 |
| DELETE | `/api/interests` | 移除興趣 |
| GET | `/api/interests/{user_id}` | 興趣列表 |

### 商店
| 方法 | 路徑 | 說明 |
|------|------|------|
| POST | `/api/shops/open` | 開店 |
| GET | `/api/shops` | 查詢自己的商店 |
| GET | `/api/shops/{id}` | 檢視商店 |
| PUT | `/api/shops/{id}` | 更新商店 |
| POST | `/api/shops/close` | 關店 |

### 商品
| 方法 | 路徑 | 說明 |
|------|------|------|
| GET | `/api/products/search` | 搜尋商品（支援 `q`, `min_price`, `max_price`） |
| GET | `/api/products/shop/{shop_id}` | 列出商店商品 |
| POST | `/api/products/shop/{shop_id}` | 新增商品 |
| GET | `/api/products/{id}` | 檢視商品 |
| PUT | `/api/products/{id}/update` | 更新商品 |
| DELETE | `/api/products/{id}` | 刪除商品 |

### 訂單
| 方法 | 路徑 | 說明 |
|------|------|------|
| POST | `/api/orders` | 下單 |
| GET | `/api/orders` | 訂單列表 |
| GET | `/api/orders/{id}` | 檢視訂單 |

### 商店私訊
| 方法 | 路徑 | 說明 |
|------|------|------|
| POST | `/api/shop-messages/send` | 傳送商店私訊 |
| GET | `/api/shop-messages/{shop_id}` | 檢視商店對話 |
| GET | `/api/shop-messages/conversations` | 商店私訊列表 |

## 資料庫表格

| 表格 | 說明 |
|------|------|
| `users` | 使用者帳號（username, display_name, bio, password_hash） |
| `sessions` | 登入 session（token, user_id） |
| `posts` | 貼文（content, parent_id 支援回覆, likes_count, replies_count） |
| `follows` | 追蹤關係（follower_id, followee_id, UNIQUE） |
| `likes` | 按讚記錄（user_id, post_id, UNIQUE） |
| `messages` | 私訊（sender_id, receiver_id, content, read） |
| `profiles` | 交友檔案（birthday, gender, city, occupation, height, etc.） |
| `interests` | 興趣標籤（user_id, tag, UNIQUE） |
| `blocks` | 封鎖記錄（blocker_id, blocked_id, UNIQUE） |
| `shops` | 商店（user_id UNIQUE, name, description） |
| `products` | 商品（shop_id, name, price, stock） |
| `orders` | 訂單（buyer_id, product_id, quantity, total_price, status） |
| `shop_messages` | 商店私訊（shop_id, sender_id, receiver_id, content） |

## 環境變數

| 變數 | 預設值 | 說明 |
|------|--------|------|
| `SMS4_DB` | `sms4.db` | SQLite 資料庫路徑 |
| `NO_COLOR` | — | 設為任意值可關閉彩色輸出（colored crate 原生支援） |

## 開發指令

```bash
cargo build                          # 除錯版
cargo build --release                # 正式版
cargo test                           # 執行所有測試
cargo fmt                            # 格式化
cargo clippy                         # 靜態分析
cargo check                          # 型別檢查
cd web && npm run dev                # 前端開發伺服器
cd web && npm run build              # 前端生產建置
```

## 測試

- **單元測試**：分散於 `src/model/*.rs`，使用 In-memory SQLite，Schema 來自 `src/db.sql`（`include_str!`）
- **整合腳本**：`case1.sh`，需手動執行
- **測試資料**：`seed.sh` 產生 15 位使用者、30 則貼文、5 間商店、21 項商品等

```bash
cargo test          # 33 項測試
bash seed.sh        # 填入測試資料
bash case1.sh       # 執行整合測試（需先 seed）
```

## 前後端架構

### 後端
- **axum 0.8** — Web 框架，所有 handler 接收 `State<AppState>`（內含 `Arc<Mutex<Connection>>`）
- **rusqlite 0.31** — SQLite 操作（bundled 模式）
- **clap 4** — CLI 參數解析（derive 模式）
- **bcrypt** — 密碼雜湊，**uuid** — Session Token
- 所有 handler 回傳 `Result<Json<T>, AppError>`

### 前端
- **React 19** + **TypeScript 5** + **Vite 6**
- **Tailwind CSS 4** — 暗色主題（純黑背景、灰色邊框、白色文字）
- **react-router-dom 7** — 客戶端路由
- Vite proxy `/api` → `http://127.0.0.1:8080`
- 使用 `fetch` + TypeScript 泛型封裝 API 客戶端

## 授權

MIT
