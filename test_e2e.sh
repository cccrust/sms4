#!/usr/bin/env bash
# test_e2e.sh — 端到端整合測試（production 模式，含前端 SPA）
set -uo pipefail

SMS4=${SMS4:-cargo run --}
SMS4_DB=${SMS4_DB:-sms4-e2e.db}
API_PORT=${API_PORT:-9880}
BASE="http://127.0.0.1:$API_PORT"

PASS=0
FAIL=0

assert() {
    local desc="$1" expected="$2" actual="$3"
    if [ "$actual" = "$expected" ]; then
        echo "  ✅ $desc"
        PASS=$((PASS + 1))
    else
        echo "  ❌ $desc (預期: $expected, 實際: $actual)"
        FAIL=$((FAIL + 1))
    fi
}

assert_contains() {
    local desc="$1" expected="$2"
    local input
    input=$(cat)
    if echo "$input" | grep -q "$expected"; then
        echo "  ✅ $desc"
        PASS=$((PASS + 1))
    else
        echo "  ❌ $desc (應包含: $expected)"
        FAIL=$((FAIL + 1))
    fi
}

req() {
    local method="$1" url="$2"
    shift 2
    local output
    if [ $# -gt 0 ]; then
        output=$(rurl -v -m "$method" "$@" "$url" 2>&1) || true
    else
        output=$(rurl -v -m "$method" "$url" 2>&1) || true
    fi
    HTTP_CODE=$(echo "$output" | sed -n 's/^Status: HTTP\/1.1 \([0-9][0-9]*\).*/\1/p')
    [ -z "$HTTP_CODE" ] && HTTP_CODE="000"
    if echo "$output" | grep -q "^Response body:"; then
        RESP=$(echo "$output" | sed -n 's/^Response body: //p')
    else
        RESP=$(echo "$output" | awk 'BEGIN{f=0} /^$/{if(f==0){f=1;next}} f==1{print}')
    fi
}

json_val() {
    local expr="$1"
    python3 -c "
import sys,json
v = json.load(sys.stdin)$expr
if isinstance(v, (list, dict)):
    print(json.dumps(v, ensure_ascii=False))
elif isinstance(v, bool):
    print(str(v).lower())
else:
    print(v)
" 2>/dev/null || echo "__ERROR__"
}

cleanup() {
    echo ""
    echo "正在停止伺服器..."
    kill "$API_PID" 2>/dev/null || true
    wait "$API_PID" 2>/dev/null || true
    rm -f "$SMS4_DB"
    echo "測試完成：通過 $PASS / 失敗 $FAIL / 總計 $((PASS+FAIL))"
    [ "$FAIL" -eq 0 ]
}

trap cleanup EXIT INT TERM

echo "=== SMS4 端到端整合測試 (production 模式) ==="
echo ""

# ── 確保前端已建置 ──
if [ ! -f "web/dist/index.html" ]; then
    echo "建置前端..."
    (cd web && npm run build) || {
        echo "❌ 前端建置失敗"
        exit 1
    }
fi

# ── 啟動 production 伺服器 ──
export SMS4_DB
SMS4_DB="$SMS4_DB" $SMS4 init
SMS4_DB="$SMS4_DB" $SMS4 web --port "$API_PORT" &
API_PID=$!
sleep 2

# ═══════════════════════════════════════════════════════
# 1. 靜態檔案服務 + SPA 路由
# ═══════════════════════════════════════════════════════

echo "=== 1. 靜態檔案服務 + SPA 路由 ==="

# 根路徑
req GET "$BASE/"
assert "GET / 回傳 200" "200" "$HTTP_CODE"
echo "$RESP" | assert_contains "含 <title>SMS4</title>" "<title>SMS4</title>"
echo "$RESP" | assert_contains "含 root div" 'id="root"'

# SPA 路由—全部應回傳 index.html（而非 404）
SPA_ROUTES=(
    "/login" "/register" "/messages" "/messages/1"
    "/messages/2?uid=1"
    "/profile/1" "/profile/edit" "/search"
    "/users" "/users/1" "/posts/1"
    "/marketplace" "/shop/1" "/my-shop" "/orders"
    "/cart" "/groups" "/groups/1"
    "/shop-messages" "/shop-messages/1"
)
for route in "${SPA_ROUTES[@]}"; do
    req GET "$BASE$route"
    assert "SPA $route 回傳 200" "200" "$HTTP_CODE"
    echo "$RESP" | assert_contains "SPA $route 含 root" 'id="root"'
done

# JS bundle 可存取
JS_PATH=$(grep -o 'src="/[^"]*"' web/dist/index.html | sed 's/src="//;s/"//' | head -1)
req GET "$BASE$JS_PATH"
assert "JS bundle $JS_PATH 回傳 200" "200" "$HTTP_CODE"

# CSS bundle 可存取
CSS_PATH=$(grep -o 'href="/[^"]*\.css"' web/dist/index.html | sed 's/href="//;s/"//' | head -1)
if [ -n "$CSS_PATH" ]; then
    req GET "$BASE$CSS_PATH"
    assert "CSS bundle $CSS_PATH 回傳 200" "200" "$HTTP_CODE"
fi

# ═══════════════════════════════════════════════════════
# 2. 完整使用者旅程
# ═══════════════════════════════════════════════════════

echo ""
echo "=== 2. 完整使用者旅程 ==="

# 註冊
req POST "$BASE/api/auth/register" -H "Content-Type: application/json" \
    -d '{"username":"alice","password":"pass1234","display_name":"愛麗絲"}'
assert "註冊回傳 200" "200" "$HTTP_CODE"
echo "$RESP" | assert_contains "註冊含 user" "username"
ALICE_ID=$(echo "$RESP" | json_val "['user']['id']")

# 登入
req POST "$BASE/api/auth/login" -H "Content-Type: application/json" \
    -d '{"username":"alice","password":"pass1234"}'
assert "登入回傳 200" "200" "$HTTP_CODE"
TOKEN=$(echo "$RESP" | json_val "['token']")
[ -n "$TOKEN" ] && assert "取得 token" "1" "1" || assert "取得 token" "1" "0"

# 註冊第二位使用者
req POST "$BASE/api/auth/register" -H "Content-Type: application/json" \
    -d '{"username":"bob","password":"pass1234","display_name":"鮑勃"}'
assert "註冊 bob 回傳 200" "200" "$HTTP_CODE"
BOB_ID=$(echo "$RESP" | json_val "['user']['id']")

# 登入 bob
req POST "$BASE/api/auth/login" -H "Content-Type: application/json" \
    -d '{"username":"bob","password":"pass1234"}'
assert "bob 登入回傳 200" "200" "$HTTP_CODE"
BOB_TOKEN=$(echo "$RESP" | json_val "['token']")

# 登出 alice（測試登出功能）
req POST "$BASE/api/auth/logout" -H "Content-Type: application/json" \
    -d "{\"token\":\"$TOKEN\"}"
assert "登出回傳 200" "200" "$HTTP_CODE"

# alice 重新登入
req POST "$BASE/api/auth/login" -H "Content-Type: application/json" \
    -d '{"username":"alice","password":"pass1234"}'
assert "重新登入回傳 200" "200" "$HTTP_CODE"
TOKEN=$(echo "$RESP" | json_val "['token']")

# bob 發佈貼文
req POST "$BASE/api/posts" -H "Content-Type: application/json" \
    -d "{\"user_id\":$BOB_ID,\"content\":\"Rust 好難但是好好玩！\"}"
assert "bob 發文回傳 200" "200" "$HTTP_CODE"
BOB_POST_ID=$(echo "$RESP" | json_val "['id']")

# alice 發佈貼文
req POST "$BASE/api/posts" -H "Content-Type: application/json" \
    -d "{\"user_id\":$ALICE_ID,\"content\":\"今天天氣真好 🌞\"}"
assert "alice 發文回傳 200" "200" "$HTTP_CODE"
ALICE_POST_ID=$(echo "$RESP" | json_val "['id']")

# alice 追蹤 bob
req POST "$BASE/api/follow" -H "Content-Type: application/json" \
    -d "{\"follower_id\":$ALICE_ID,\"followee_id\":$BOB_ID}"
assert "追蹤 bob 回傳 200" "200" "$HTTP_CODE"

# alice 的 timeline 應有 bob 的貼文
req GET "$BASE/api/users/$ALICE_ID/timeline"
assert "timeline 回傳 200" "200" "$HTTP_CODE"
echo "$RESP" | assert_contains "timeline 含 bob 貼文" "Rust 好難"
echo "$RESP" | assert_contains "timeline 含自己的貼文" "天氣真好"

# bob 對 alice 的貼文按讚
req POST "$BASE/api/likes" -H "Content-Type: application/json" \
    -d "{\"user_id\":$BOB_ID,\"post_id\":$ALICE_POST_ID}"
assert "bob 按讚回傳 200" "200" "$HTTP_CODE"
echo "$RESP" | assert_contains "按讚成功" "已按讚"

# 驗證讚數
req GET "$BASE/api/posts/$ALICE_POST_ID"
assert "讚數=1 (關係驗證)" "1" "$(echo "$RESP" | json_val "['post']['likes_count']")"

# bob 回覆 alice 的貼文
req POST "$BASE/api/posts/$ALICE_POST_ID/reply" -H "Content-Type: application/json" \
    -d "{\"user_id\":$BOB_ID,\"content\":\"對啊，適合出門走走！\"}"
assert "回覆貼文回傳 200" "200" "$HTTP_CODE"

# 驗證回覆數
req GET "$BASE/api/posts/$ALICE_POST_ID"
assert "replies_count=1 (關係驗證)" "1" "$(echo "$RESP" | json_val "['post']['replies_count']")"

# ═══════════════════════════════════════════════════════
# 3. CLI × Web API 跨端一致性
# ═══════════════════════════════════════════════════════

echo ""
echo "=== 3. CLI × Web API 跨端一致性 ==="

# CLI 看到 Web API 建立的資料
R=$($SMS4 user list 2>&1 | grep -c "alice")
assert "CLI 看到 alice (跨端驗證)" "1" "$(echo "$R")"

R=$($SMS4 post list 2>&1 | grep -c "天氣真好")
assert "CLI 看到 alice 貼文 (跨端驗證)" "1" "$(echo "$R")"

R=$($SMS4 post list 2>&1 | grep -c "Rust 好難")
assert "CLI 看到 bob 貼文 (跨端驗證)" "1" "$(echo "$R")"

# CLI 查詢追蹤
R=$($SMS4 follow following "$ALICE_ID" 2>&1 | grep -c "bob")
assert "CLI 看到 alice 追蹤 bob (跨端驗證)" "1" "$(echo "$R")"

# CLI 新增資料後 Web API 可看到
$SMS4 user add dave 大衛 --bio "新使用者" 2>&1 > /dev/null
req GET "$BASE/api/users"
echo "$RESP" | assert_contains "Web API 看到 CLI 新增的 dave" "dave"

# CLI 新增貼文後 Web API 可看到
$SMS4 post add "$ALICE_ID" "CLI 新增的貼文" 2>&1 > /dev/null
req GET "$BASE/api/users/$ALICE_ID/timeline"
echo "$RESP" | assert_contains "Web API 看到 CLI 新增貼文" "CLI 新增的貼文"

# ═══════════════════════════════════════════════════════
# 4. CLI 完整指令測試
# ═══════════════════════════════════════════════════════

echo ""
echo "=== 4. CLI 完整指令測試 ==="

# ── user 完整 CRUD ──

# user get
R=$($SMS4 user get "$ALICE_ID" 2>&1)
echo "$R" | assert_contains "CLI user get 顯示 alice" "alice"
echo "$R" | assert_contains "CLI user get 愛麗絲" "愛麗絲"

# user update
$SMS4 user update "$ALICE_ID" --bio "熱愛 Rust 的工程師" 2>&1 > /dev/null
R=$($SMS4 user get "$ALICE_ID" 2>&1)
echo "$R" | assert_contains "CLI user update bio 成功" "Rust"

# user delete (測試用，不刪 alice)—透過 API 取得 ID 再刪除
$SMS4 user add tempuser 臨時使用者 2>&1 > /dev/null
req GET "$BASE/api/users"
TEMP_ID=$(echo "$RESP" | python3 -c "import sys,json; users=json.load(sys.stdin); print([u['id'] for u in users if u['username']=='tempuser'][0])" 2>/dev/null || echo "")
if [ -n "$TEMP_ID" ]; then
    $SMS4 user delete "$TEMP_ID" 2>&1 > /dev/null
    R=$($SMS4 user list 2>&1 | grep -c "tempuser")
    assert "CLI user delete 成功移除" "0" "$R"
fi

# ── post 完整 CRUD ──

# post reply（CLI）
$SMS4 post reply "$ALICE_POST_ID" "$BOB_ID" "CLI 回覆測試" 2>&1 > /dev/null
R=$($SMS4 post get "$ALICE_POST_ID" 2>&1)
echo "$R" | assert_contains "CLI post get 顯示回覆" "CLI 回覆測試"
echo "$R" | assert_contains "CLI post get 顯示原貼文" "天氣真好"

# post timeline（CLI）
R=$($SMS4 post timeline "$ALICE_ID" 2>&1)
echo "$R" | assert_contains "CLI timeline 顯示 bob 貼文" "Rust 好難"
echo "$R" | assert_contains "CLI timeline 顯示 alice 貼文" "天氣真好"

# post delete（CLI—透過 API 取得貼文 ID 再刪除）
req GET "$BASE/api/users/$ALICE_ID/timeline"
DELETE_PID=$(echo "$RESP" | python3 -c "import sys,json; posts=json.load(sys.stdin); print([p['id'] for p in posts if 'CLI 新增的貼文' in p['content']][0])" 2>/dev/null || echo "")
if [ -n "$DELETE_PID" ]; then
    $SMS4 post delete "$DELETE_PID" 2>&1 > /dev/null
    R=$($SMS4 post list 2>&1 | grep -c "CLI 新增的貼文")
    assert "CLI post delete 成功移除" "0" "$R"
fi

# ── follow 指令 ──

# follow add（CLI 讓 bob 追蹤 alice）
$SMS4 follow add "$BOB_ID" "$ALICE_ID" 2>&1 > /dev/null

# follow followers
R=$($SMS4 follow followers "$ALICE_ID" 2>&1)
echo "$R" | assert_contains "CLI followers 顯示 bob" "bob"

# follow following
R=$($SMS4 follow following "$BOB_ID" 2>&1)
echo "$R" | assert_contains "CLI following 顯示 alice" "alice"

# follow remove
$SMS4 follow remove "$BOB_ID" "$ALICE_ID" 2>&1 > /dev/null
R=$($SMS4 follow following "$BOB_ID" 2>&1 | grep -c "alice")
assert "CLI follow remove 成功" "0" "$R"

# ── like 指令 ──

# 先取消讚（原本 user journey 中 bob 已按讚，讚數=1）
$SMS4 like remove "$BOB_ID" "$ALICE_POST_ID" 2>&1 > /dev/null
req GET "$BASE/api/posts/$ALICE_POST_ID"
assert "CLI like remove 後讚數=0" "0" "$(echo "$RESP" | json_val "['post']['likes_count']")"

# like add（CLI）
$SMS4 like add "$BOB_ID" "$ALICE_POST_ID" 2>&1 > /dev/null
req GET "$BASE/api/posts/$ALICE_POST_ID"
assert "CLI like add 後讚數=1 (關係驗證)" "1" "$(echo "$RESP" | json_val "['post']['likes_count']")"

# ── msg 指令 ──

# msg send（CLI）
$SMS4 msg send "$ALICE_ID" "$BOB_ID" "這是 CLI 發送的私訊" 2>&1 > /dev/null

# msg inbox
R=$($SMS4 msg inbox "$BOB_ID" 2>&1)
echo "$R" | assert_contains "CLI inbox 顯示對話" "愛麗絲"

# msg conversation
R=$($SMS4 msg conversation "$ALICE_ID" "$BOB_ID" 2>&1)
echo "$R" | assert_contains "CLI conversation 顯示私訊" "這是 CLI 發送的私訊"

# 跨端驗證：Web API 可看到 CLI 發送的私訊
req GET "$BASE/api/messages/$ALICE_ID/conversations"
echo "$RESP" | assert_contains "Web API 顯示 CLI 私訊對話" "鮑勃"

# ── profile + interest 指令 ──

# user add 不帶 --bio（測試無 bio 情境）
$SMS4 user add nancy 小南 2>&1 > /dev/null

# profile set（CLI）
$SMS4 profile set "$ALICE_ID" --birthday "1995-03-15" --gender male --city "台北" --about-me "我是測試用帳號" 2>&1 > /dev/null

# profile show
R=$($SMS4 profile show "$ALICE_ID" 2>&1)
echo "$R" | assert_contains "CLI profile show 顯示 city" "台北"

# interest add（CLI）
$SMS4 interest add "$ALICE_ID" "跑步" 2>&1 > /dev/null
$SMS4 interest add "$ALICE_ID" "游泳" 2>&1 > /dev/null

# interest list
R=$($SMS4 interest list "$ALICE_ID" 2>&1)
echo "$R" | assert_contains "CLI interest list 含跑步" "跑步"
echo "$R" | assert_contains "CLI interest list 含游泳" "游泳"

# interest remove
$SMS4 interest remove "$ALICE_ID" "游泳" 2>&1 > /dev/null
R=$($SMS4 interest list "$ALICE_ID" 2>&1)
echo "$R" | assert_contains "CLI interest 移除後仍含跑步" "跑步"
assert "CLI interest 移除後無游泳" "0" "$(echo "$R" | grep -c "游泳")"

# profile search（CLI）
R=$($SMS4 profile search --gender male 2>&1)
echo "$R" | assert_contains "CLI 搜尋 male 含 alice" "alice"

R=$($SMS4 profile search --city 台 2>&1)
echo "$R" | assert_contains "CLI 搜尋 city 有結果" "alice"

R=$($SMS4 profile search --tags 跑步 2>&1)
echo "$R" | assert_contains "CLI 搜尋 tags 含 alice" "alice"

R=$($SMS4 profile search --age-min 20 --age-max 40 2>&1)
echo "$R" | assert_contains "CLI 年齡搜尋有結果" "alice"

R=$($SMS4 profile search -q 測試 2>&1)
echo "$R" | assert_contains "CLI 關鍵字搜尋含 alice" "alice"

# ═══════════════════════════════════════════════════════
# 5. 錯誤處理
# ═══════════════════════════════════════════════════════

echo ""
echo "=== 5. 錯誤處理 ==="

# API 層 404
req GET "$BASE/api/users/999"
assert "GET /api/users/999 回傳 404" "404" "$HTTP_CODE"

req GET "$BASE/api/posts/999"
assert "GET /api/posts/999 回傳 404" "404" "$HTTP_CODE"

# 錯誤密碼
req POST "$BASE/api/auth/login" -H "Content-Type: application/json" \
    -d '{"username":"alice","password":"wrongpass"}'
assert "錯誤密碼回傳 400" "400" "$HTTP_CODE"

# 缺少欄位（密碼太短）
req POST "$BASE/api/auth/register" -H "Content-Type: application/json" \
    -d '{"username":"test","password":"ab","display_name":"T"}'
assert "密碼太短回傳 400" "400" "$HTTP_CODE"

# SPA fallback（不存在的 SPA 路徑仍回 200 + index.html）
req GET "$BASE/some/random/path"
assert "SPA fallback 回傳 200" "200" "$HTTP_CODE"
echo "$RESP" | assert_contains "SPA fallback 含 root" 'id="root"'

# 靜態檔案 404（不存在的 asset）
req GET "$BASE/assets/nonexistent.js"
assert "不存在 asset 回傳 404" "404" "$HTTP_CODE"

echo ""
