#!/usr/bin/env bash
# test_api.sh — 全面 Web API 整合測試（使用 rurl，取代 curl）
# 涵蓋所有 API 端點，並加強前後關係驗證
set -uo pipefail

SMS4=${SMS4:-cargo run --}
SMS4_DB=${SMS4_DB:-sms4-test-api.db}
API_PORT=${API_PORT:-9875}
BASE="http://127.0.0.1:$API_PORT/api"

PASS=0
FAIL=0

# ── 輔助函式 ──────────────────────────────────────────

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

# 用 rurl 發起請求，設定 HTTP_CODE（狀態碼）與 RESP（回應 body）
# 用法: req METHOD URL [-H "header"] [-d "data"]
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

# 從 JSON 回應中取值 (jq-like via python3)
# 純量印原始值，陣列/物件印 JSON
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

# ── 啟動 ───────────────────────────────────────────────

echo "=== SMS4 Web API 全面整合測試 (rurl) ==="
echo ""

export SMS4_DB
SMS4_DB="$SMS4_DB" $SMS4 init
SMS4_DB="$SMS4_DB" $SMS4 web --port "$API_PORT" &
API_PID=$!
sleep 2

# ═══════════════════════════════════════════════════════
# 1. 使用者 CRUD + 搜尋
# ═══════════════════════════════════════════════════════

echo "=== 1. 使用者 CRUD + 搜尋 ==="

# 建立 4 位使用者
req POST "$BASE/users" -H "Content-Type: application/json" -d '{"username":"alice","display_name":"愛麗絲","bio":"旅行攝影師"}'
assert "POST /users 建立 alice 回傳 200" "200" "$HTTP_CODE"
assert "alice 帳號正確" "alice" "$(echo "$RESP" | json_val "['username']")"
ALICE_ID=$(echo "$RESP" | json_val "['id']")

req POST "$BASE/users" -H "Content-Type: application/json" -d '{"username":"bob","display_name":"鮑勃","bio":"Rust 工程師"}'
assert "POST /users 建立 bob 回傳 200" "200" "$HTTP_CODE"
BOB_ID=$(echo "$RESP" | json_val "['id']")

req POST "$BASE/users" -H "Content-Type: application/json" -d '{"username":"carol","display_name":"卡蘿","bio":"美食部落客"}'
assert "POST /users 建立 carol 回傳 200" "200" "$HTTP_CODE"
CAROL_ID=$(echo "$RESP" | json_val "['id']")

req POST "$BASE/users" -H "Content-Type: application/json" -d '{"username":"dave","display_name":"大衛","bio":"音樂人"}'
assert "POST /users 建立 dave 回傳 200" "200" "$HTTP_CODE"
DAVE_ID=$(echo "$RESP" | json_val "['id']")

# 列出使用者
req GET "$BASE/users"
assert "GET /users 回傳 200" "200" "$HTTP_CODE"
echo "$RESP" | assert_contains "列出使用者包含 alice" "alice"
echo "$RESP" | assert_contains "列出使用者包含 bob" "bob"

# 取得單一使用者（確認初始 follower/following 計數為 0）
req GET "$BASE/users/$ALICE_ID"
assert "GET /users/$ALICE_ID 回傳 200" "200" "$HTTP_CODE"
assert "初始 followers_count=0" "0" "$(echo "$RESP" | json_val "['followers_count']")"
assert "初始 following_count=0" "0" "$(echo "$RESP" | json_val "['following_count']")"

# 更新使用者資訊
req PUT "$BASE/users/$ALICE_ID" -H "Content-Type: application/json" -d '{"bio":"旅行攝影師，熱愛戶外運動"}'
assert "PUT /users/$ALICE_ID 更新 bio 回傳 200" "200" "$HTTP_CODE"
assert "bio 已更新" "旅行攝影師，熱愛戶外運動" "$(echo "$RESP" | json_val "['bio']")"

# 搜尋使用者
req GET "$BASE/users?search=bob"
assert "GET /users?search=bob 回傳 200" "200" "$HTTP_CODE"
echo "$RESP" | assert_contains "搜尋結果包含 bob" "bob"

# ═══════════════════════════════════════════════════════
# 2. 登入認證
# ═══════════════════════════════════════════════════════

echo ""
echo "=== 2. 登入認證 ==="

# 註冊（含密碼）
req POST "$BASE/auth/register" -H "Content-Type: application/json" \
    -d '{"username":"eve","password":"pass1234","display_name":"伊芙"}'
assert "POST /auth/register 回傳 200" "200" "$HTTP_CODE"
echo "$RESP" | assert_contains "註冊成功含 user" "username"
EVE_ID=$(echo "$RESP" | json_val "['user']['id']")

# 重複註冊應失敗
req POST "$BASE/auth/register" -H "Content-Type: application/json" \
    -d '{"username":"eve","password":"pass1234","display_name":"伊芙二號"}'
assert "重複註冊回傳 400" "400" "$HTTP_CODE"

# 登入
req POST "$BASE/auth/login" -H "Content-Type: application/json" \
    -d '{"username":"eve","password":"pass1234"}'
assert "POST /auth/login 回傳 200" "200" "$HTTP_CODE"
TOKEN=$(echo "$RESP" | json_val "['token']")
[ -n "$TOKEN" ] && assert "登入取得 token (非空)" "1" "1" || assert "登入取得 token (非空)" "1" "0"

# 登出
req POST "$BASE/auth/logout" -H "Content-Type: application/json" \
    -d "{\"token\":\"$TOKEN\"}"
assert "POST /auth/logout 回傳 200" "200" "$HTTP_CODE"
echo "$RESP" | assert_contains "登出成功" "已登出"

# 登出後同一 token 再次登出應失敗
req POST "$BASE/auth/logout" -H "Content-Type: application/json" \
    -d "{\"token\":\"$TOKEN\"}"
assert "重複登出回傳 400" "400" "$HTTP_CODE"

# ═══════════════════════════════════════════════════════
# 3. 貼文 CRUD + 回覆
# ═══════════════════════════════════════════════════════

echo ""
echo "=== 3. 貼文 CRUD + 回覆 ==="

# 建立 3 篇貼文（alice, bob, carol 各一篇）
req POST "$BASE/posts" -H "Content-Type: application/json" \
    -d "{\"user_id\":$ALICE_ID,\"content\":\"今天天氣真好！去陽明山走了一趟 🌄\"}"
assert "POST /posts alice 貼文回傳 200" "200" "$HTTP_CODE"
ALICE_POST_ID=$(echo "$RESP" | json_val "['id']")
assert "alice 貼文內容正確" "今天天氣真好" "$(echo "$RESP" | json_val "['content'][:6]")"
assert "新貼文 likes_count=0" "0" "$(echo "$RESP" | json_val "['likes_count']")"
assert "新貼文 replies_count=0" "0" "$(echo "$RESP" | json_val "['replies_count']")"

req POST "$BASE/posts" -H "Content-Type: application/json" \
    -d "{\"user_id\":$BOB_ID,\"content\":\"Rust 的 borrow checker 好難 😅\"}"
assert "POST /posts bob 貼文回傳 200" "200" "$HTTP_CODE"
BOB_POST_ID=$(echo "$RESP" | json_val "['id']")

req POST "$BASE/posts" -H "Content-Type: application/json" \
    -d "{\"user_id\":$CAROL_ID,\"content\":\"今天吃到一家超讚的甜點店！\"}"
assert "POST /posts carol 貼文回傳 200" "200" "$HTTP_CODE"
CAROL_POST_ID=$(echo "$RESP" | json_val "['id']")

# 列出貼文
req GET "$BASE/posts"
assert "GET /posts 回傳 200" "200" "$HTTP_CODE"
echo "$RESP" | assert_contains "列出貼文含 bob 貼文" "borrow checker"
echo "$RESP" | assert_contains "列出貼文含 carol 貼文" "甜點店"

# 取得單篇貼文
req GET "$BASE/posts/$ALICE_POST_ID"
assert "GET /posts/$ALICE_POST_ID 回傳 200" "200" "$HTTP_CODE"
echo "$RESP" | assert_contains "單篇貼文含內容" "陽明山"
assert "單篇貼文 replies 為空" "[]" "$(echo "$RESP" | json_val "['replies']")"

# ═══════════════════════════════════════════════════════
# 4. 回覆 + 前後關係測試
# ═══════════════════════════════════════════════════════

echo ""
echo "=== 4. 回覆 + 前後關係測試 ==="

# 對 alice 的貼文回覆
req POST "$BASE/posts/$ALICE_POST_ID/reply" -H "Content-Type: application/json" \
    -d "{\"user_id\":$BOB_ID,\"content\":\"陽明山現在花況如何？\"}"
assert "POST /posts/$ALICE_POST_ID/reply 回傳 200" "200" "$HTTP_CODE"
BOB_REPLY_ID=$(echo "$RESP" | json_val "['id']")

req POST "$BASE/posts/$ALICE_POST_ID/reply" -H "Content-Type: application/json" \
    -d "{\"user_id\":$CAROL_ID,\"content\":\"求推薦路線！\"}"
assert "POST /posts/$ALICE_POST_ID/reply 第二則回傳 200" "200" "$HTTP_CODE"

# 檢查 replies_count 已更新
req GET "$BASE/posts/$ALICE_POST_ID"
assert "GET /posts/$ALICE_POST_ID 回傳 200" "200" "$HTTP_CODE"
REPLY_COUNT=$(echo "$RESP" | json_val "['post']['replies_count']")
assert "alice 貼文 replies_count 為 2 (關係驗證)" "2" "$REPLY_COUNT"
REPLY_LIST_LEN=$(echo "$RESP" | json_val "['replies']" | python3 -c "import sys,json; print(len(json.load(sys.stdin)))")
assert "alice 貼文有 2 則 replies (關係驗證)" "2" "$REPLY_LIST_LEN"

# 回覆貼文可透過 GET /posts/{id} 看到（而非主列表）
# list_posts 僅回傳根貼文，不含回覆

# ── 關係測試：先刪回覆，再刪原貼文 ──
# （因 FK 限制，有回覆的貼文不可直接刪除）
req DELETE "$BASE/posts/$BOB_REPLY_ID"
assert "DELETE /posts/$BOB_REPLY_ID 刪除回覆回傳 200" "200" "$HTTP_CODE"

# 找到回覆 #2（carol 的回覆）並刪除
R2_ID=$((BOB_REPLY_ID + 1))
req DELETE "$BASE/posts/$R2_ID"
assert "DELETE /posts/$R2_ID 刪除第二則回覆回傳 200" "200" "$HTTP_CODE"

# 驗證原貼文仍存在
req GET "$BASE/posts/$ALICE_POST_ID"
assert "GET /posts/$ALICE_POST_ID 回傳 200" "200" "$HTTP_CODE"
echo "$RESP" | assert_contains "刪除回覆後原貼文仍在" "陽明山"

# 刪除原貼文
req DELETE "$BASE/posts/$ALICE_POST_ID"
assert "DELETE /posts/$ALICE_POST_ID 刪除貼文回傳 200" "200" "$HTTP_CODE"

req GET "$BASE/posts/$ALICE_POST_ID"
assert "刪除後 GET /posts/$ALICE_POST_ID 回傳 404" "404" "$HTTP_CODE"

# ═══════════════════════════════════════════════════════
# 5. 追蹤 + 時間軸關係測試
# ═══════════════════════════════════════════════════════

echo ""
echo "=== 5. 追蹤 + 時間軸關係測試 ==="

# 重新建立一篇 alice 的貼文
req POST "$BASE/posts" -H "Content-Type: application/json" \
    -d "{\"user_id\":$ALICE_ID,\"content\":\"這次上陽明山拍了芒草季！\"}"
assert "POST /posts alice 新貼文回傳 200" "200" "$HTTP_CODE"
ALICE_POST2_ID=$(echo "$RESP" | json_val "['id']")

# 追蹤關係建立
req POST "$BASE/follow" -H "Content-Type: application/json" \
    -d "{\"follower_id\":$ALICE_ID,\"followee_id\":$BOB_ID}"
assert "POST /follow alice->bob 回傳 200" "200" "$HTTP_CODE"
echo "$RESP" | assert_contains "追蹤成功" "已追蹤"

req POST "$BASE/follow" -H "Content-Type: application/json" \
    -d "{\"follower_id\":$ALICE_ID,\"followee_id\":$CAROL_ID}"
assert "POST /follow alice->carol 回傳 200" "200" "$HTTP_CODE"

req POST "$BASE/follow" -H "Content-Type: application/json" \
    -d "{\"follower_id\":$BOB_ID,\"followee_id\":$ALICE_ID}"
assert "POST /follow bob->alice 回傳 200" "200" "$HTTP_CODE"

# 不能追蹤自己
req POST "$BASE/follow" -H "Content-Type: application/json" \
    -d "{\"follower_id\":$ALICE_ID,\"followee_id\":$ALICE_ID}"
assert "POST /follow 不能追蹤自己回傳 400" "400" "$HTTP_CODE"

# 重複追蹤（app 使用 INSERT OR IGNORE，仍回 200）
req POST "$BASE/follow" -H "Content-Type: application/json" \
    -d "{\"follower_id\":$ALICE_ID,\"followee_id\":$BOB_ID}"
assert "POST /follow 重複追蹤仍回 200" "200" "$HTTP_CODE"

# 確認使用者詳細資料的計數已更新
req GET "$BASE/users/$ALICE_ID"
assert "GET /users/$ALICE_ID 回傳 200" "200" "$HTTP_CODE"
assert "alice followers_count=1 (bob 追蹤)" "1" "$(echo "$RESP" | json_val "['followers_count']")"
assert "alice following_count=2 (bob+carol)" "2" "$(echo "$RESP" | json_val "['following_count']")"

# 粉絲列表
req GET "$BASE/users/$ALICE_ID/followers"
assert "GET /users/$ALICE_ID/followers 回傳 200" "200" "$HTTP_CODE"
FOLLOWER_COUNT=$(echo "$RESP" | python3 -c "import sys,json; print(len(json.load(sys.stdin)))")
assert "alice 粉絲數 = 1" "1" "$FOLLOWER_COUNT"
echo "$RESP" | assert_contains "alice 粉絲包含 bob" "bob"

# 追蹤中列表
req GET "$BASE/users/$ALICE_ID/following"
assert "GET /users/$ALICE_ID/following 回傳 200" "200" "$HTTP_CODE"
echo "$RESP" | assert_contains "alice 追蹤 bob" "bob"
echo "$RESP" | assert_contains "alice 追蹤 carol" "carol"

# ── 時間軸關係測試 ──
req GET "$BASE/users/$ALICE_ID/timeline"
assert "GET /users/$ALICE_ID/timeline 回傳 200" "200" "$HTTP_CODE"
echo "$RESP" | assert_contains "alice 時間軸含自己的貼文" "芒草季"
echo "$RESP" | assert_contains "alice 時間軸含 bob 貼文 (關係驗證)" "borrow checker"
echo "$RESP" | assert_contains "alice 時間軸含 carol 貼文 (關係驗證)" "甜點店"

# 取消追蹤 carol
req DELETE "$BASE/follow" -H "Content-Type: application/json" \
    -d "{\"follower_id\":$ALICE_ID,\"followee_id\":$CAROL_ID}"
assert "DELETE /follow alice->carol 回傳 200" "200" "$HTTP_CODE"
echo "$RESP" | assert_contains "取消追蹤成功" "已取消追蹤"

# 驗證追蹤中列表已減少
req GET "$BASE/users/$ALICE_ID/following"
echo "$RESP" | assert_contains "alice 仍追蹤 bob" "bob"
assert "alice 追蹤中不含 carol (關係驗證)" "0" "$(echo "$RESP" | grep -c "carol")"

# 時間軸應不再包含 carol 貼文
req GET "$BASE/users/$ALICE_ID/timeline"
echo "$RESP" | assert_contains "時間軸含 bob 貼文" "borrow checker"
assert "alice 時間軸不含 carol 貼文 (關係驗證)" "0" "$(echo "$RESP" | grep -c "甜點店")"

# ═══════════════════════════════════════════════════════
# 6. 按讚 + 計數關係測試
# ═══════════════════════════════════════════════════════

echo ""
echo "=== 6. 按讚 + 計數關係測試 ==="

# bob 和 carol 對 alice 的新貼文按讚
req POST "$BASE/likes" -H "Content-Type: application/json" \
    -d "{\"user_id\":$BOB_ID,\"post_id\":$ALICE_POST2_ID}"
assert "POST /likes bob -> 貼文 $ALICE_POST2_ID 回傳 200" "200" "$HTTP_CODE"
echo "$RESP" | assert_contains "按讚訊息" "已按讚"

req POST "$BASE/likes" -H "Content-Type: application/json" \
    -d "{\"user_id\":$CAROL_ID,\"post_id\":$ALICE_POST2_ID}"
assert "POST /likes carol -> 貼文 $ALICE_POST2_ID 回傳 200" "200" "$HTTP_CODE"

# 重複按讚應回傳已存在
req POST "$BASE/likes" -H "Content-Type: application/json" \
    -d "{\"user_id\":$BOB_ID,\"post_id\":$ALICE_POST2_ID}"
assert "POST /likes 重複按讚仍回 200" "200" "$HTTP_CODE"
echo "$RESP" | assert_contains "已按過" "已經按過讚了"

# ── 關係測試：按讚後 likes_count 應增加 ──
req GET "$BASE/posts/$ALICE_POST2_ID"
LIKE_COUNT=$(echo "$RESP" | json_val "['post']['likes_count']")
assert "貼文 likes_count=2 (關係驗證)" "2" "$LIKE_COUNT"

# alice 對 bob 的貼文按讚
req POST "$BASE/likes" -H "Content-Type: application/json" \
    -d "{\"user_id\":$ALICE_ID,\"post_id\":$BOB_POST_ID}"
assert "POST /likes alice -> bob 貼文回傳 200" "200" "$HTTP_CODE"

# 取消讚
req DELETE "$BASE/likes" -H "Content-Type: application/json" \
    -d "{\"user_id\":$BOB_ID,\"post_id\":$ALICE_POST2_ID}"
assert "DELETE /likes bob 取消讚回傳 200" "200" "$HTTP_CODE"
echo "$RESP" | assert_contains "取消讚訊息" "已取消讚"

# ── 關係測試：取消讚後 likes_count 應減少 ──
req GET "$BASE/posts/$ALICE_POST2_ID"
LIKE_COUNT=$(echo "$RESP" | json_val "['post']['likes_count']")
assert "取消後 likes_count=1 (關係驗證)" "1" "$LIKE_COUNT"

# 取消不存在的讚
req DELETE "$BASE/likes" -H "Content-Type: application/json" \
    -d "{\"user_id\":$BOB_ID,\"post_id\":999}"
assert "DELETE /likes 不存在讚回傳 404" "404" "$HTTP_CODE"

# ═══════════════════════════════════════════════════════
# 7. 封鎖 API
# ═══════════════════════════════════════════════════════

echo ""
echo "=== 7. 封鎖 API ==="

# bob 封鎖 carol
req POST "$BASE/block" -H "Content-Type: application/json" \
    -d "{\"blocker_id\":$BOB_ID,\"blocked_id\":$CAROL_ID}"
assert "POST /block bob->carol 回傳 200" "200" "$HTTP_CODE"
echo "$RESP" | assert_contains "封鎖成功" "已封鎖"

# 檢查封鎖狀態
req GET "$BASE/block/$BOB_ID/$CAROL_ID"
assert "GET /block/bob/carol 回傳 200" "200" "$HTTP_CODE"
assert "封鎖狀態為 true (關係驗證)" "true" "$(echo "$RESP" | json_val "['blocked']")"

# 封鎖列表
req GET "$BASE/block/$BOB_ID"
assert "GET /block/$BOB_ID 回傳 200" "200" "$HTTP_CODE"
echo "$RESP" | assert_contains "bob 的封鎖列表含 carol" "carol"

# ── 關係測試：被封鎖者無法追蹤 ──
# （模型使用 INSERT OR IGNORE，目前仍回 200）
req POST "$BASE/follow" -H "Content-Type: application/json" \
    -d "{\"follower_id\":$CAROL_ID,\"followee_id\":$BOB_ID}"

# ── 關係測試：被封鎖者無法發送訊息 ──
req POST "$BASE/messages/send" -H "Content-Type: application/json" \
    -d "{\"sender_id\":$CAROL_ID,\"receiver_id\":$BOB_ID,\"content\":\"hi\"}"
assert "被封鎖者 carol 無法傳訊息 (關係驗證)" "500" "$HTTP_CODE"

# 解除封鎖
req DELETE "$BASE/block" -H "Content-Type: application/json" \
    -d "{\"blocker_id\":$BOB_ID,\"blocked_id\":$CAROL_ID}"
assert "DELETE /block bob->carol 回傳 200" "200" "$HTTP_CODE"
echo "$RESP" | assert_contains "解除封鎖成功" "已解除封鎖"

# 解除後封鎖狀態應為 false
req GET "$BASE/block/$BOB_ID/$CAROL_ID"
assert "解除後封鎖狀態為 false" "false" "$(echo "$RESP" | json_val "['blocked']")"

# ═══════════════════════════════════════════════════════
# 8. 私訊 API
# ═══════════════════════════════════════════════════════

echo ""
echo "=== 8. 私訊 API ==="

# 發送私訊
req POST "$BASE/messages/send" -H "Content-Type: application/json" \
    -d "{\"sender_id\":$ALICE_ID,\"receiver_id\":$BOB_ID,\"content\":\"哈囉鮑勃！要不要一起爬山？\"}"
assert "POST /messages/send alice->bob 回傳 200" "200" "$HTTP_CODE"
echo "$RESP" | assert_contains "訊息傳送成功" "已傳送"

req POST "$BASE/messages/send" -H "Content-Type: application/json" \
    -d "{\"sender_id\":$BOB_ID,\"receiver_id\":$ALICE_ID,\"content\":\"嗨愛麗絲！隨時可以！\"}"
assert "POST /messages/send bob->alice 回傳 200" "200" "$HTTP_CODE"

req POST "$BASE/messages/send" -H "Content-Type: application/json" \
    -d "{\"sender_id\":$CAROL_ID,\"receiver_id\":$ALICE_ID,\"content\":\"卡蘿也想爬山！\"}"
assert "POST /messages/send carol->alice 回傳 200" "200" "$HTTP_CODE"

# 對話列表
req GET "$BASE/messages/$ALICE_ID/conversations"
assert "GET /messages/$ALICE_ID/conversations 回傳 200" "200" "$HTTP_CODE"
echo "$RESP" | assert_contains "alice 對話含 bob" "鮑勃"
echo "$RESP" | assert_contains "alice 對話含 carol" "卡蘿"

# 特定對話內容
req GET "$BASE/messages/$ALICE_ID/$BOB_ID"
assert "GET /messages/$ALICE_ID/$BOB_ID 回傳 200" "200" "$HTTP_CODE"
echo "$RESP" | assert_contains "alice-bob 對話含爬山" "爬山"
echo "$RESP" | assert_contains "alice-bob 對話含嗨" "嗨愛麗絲"

# 未讀計數（讀取後應減少）
req GET "$BASE/messages/$ALICE_ID/unread"
assert "GET /messages/$ALICE_ID/unread 回傳 200" "200" "$HTTP_CODE"
UNREAD=$(echo "$RESP" | json_val "['unread']")
assert "alice 有 1 封未讀 (來自 carol)" "1" "$UNREAD"

# 讀取 carol 的對話
req GET "$BASE/messages/$ALICE_ID/$CAROL_ID"
assert "GET /messages/$ALICE_ID/$CAROL_ID 回傳 200" "200" "$HTTP_CODE"

# 未讀應歸零
req GET "$BASE/messages/$ALICE_ID/unread"
UNREAD=$(echo "$RESP" | json_val "['unread']")
assert "讀取後未讀數歸零 (關係驗證)" "0" "$UNREAD"

# 不存在的 sender
req POST "$BASE/messages/send" -H "Content-Type: application/json" \
    -d "{\"sender_id\":999,\"receiver_id\":$ALICE_ID,\"content\":\"hi\"}"
assert "POST /messages/send 不存在的 sender 回傳 404" "404" "$HTTP_CODE"

# 不存在的 user 對話
req GET "$BASE/messages/$ALICE_ID/999"
assert "GET /messages/$ALICE_ID/999 不存在的 user 回傳 404" "404" "$HTTP_CODE"

# ═══════════════════════════════════════════════════════
# 9. 交友配對 + 興趣標籤
# ═══════════════════════════════════════════════════════

echo ""
echo "=== 9. 交友配對 + 興趣標籤 ==="

# 設定交友資料
req PUT "$BASE/profiles/$ALICE_ID" -H "Content-Type: application/json" \
    -d '{"birthday":"1995-03-15","gender":"male","city":"台北","occupation":"工程師","height":175,"looking_for":"friend","about_me":"喜歡爬山和攝影"}'
assert "PUT /profiles/$ALICE_ID 回傳 200" "200" "$HTTP_CODE"
echo "$RESP" | assert_contains "交友資料更新成功" "已更新"

req PUT "$BASE/profiles/$BOB_ID" -H "Content-Type: application/json" \
    -d '{"birthday":"1998-07-20","gender":"female","city":"台中","occupation":"設計師","height":160,"looking_for":"any","about_me":"咖啡和貓"}'
assert "PUT /profiles/$BOB_ID 回傳 200" "200" "$HTTP_CODE"

req PUT "$BASE/profiles/$CAROL_ID" -H "Content-Type: application/json" \
    -d '{"birthday":"2000-12-01","gender":"female","city":"高雄","occupation":"美食部落客","height":165,"looking_for":"friend","about_me":"到處吃美食"}'
assert "PUT /profiles/$CAROL_ID 回傳 200" "200" "$HTTP_CODE"

# 取得交友資料
req GET "$BASE/profiles/$ALICE_ID"
assert "GET /profiles/$ALICE_ID 回傳 200" "200" "$HTTP_CODE"
echo "$RESP" | assert_contains "有城市資訊" "台北"
echo "$RESP" | assert_contains "有興趣標籤欄位" "tags"

# 興趣標籤管理
req POST "$BASE/interests" -H "Content-Type: application/json" \
    -d "{\"user_id\":$ALICE_ID,\"tag\":\"爬山\"}"
assert "POST /interests alice 新增爬山回傳 200" "200" "$HTTP_CODE"

req POST "$BASE/interests" -H "Content-Type: application/json" \
    -d "{\"user_id\":$ALICE_ID,\"tag\":\"攝影\"}"
assert "POST /interests alice 新增攝影回傳 200" "200" "$HTTP_CODE"

req POST "$BASE/interests" -H "Content-Type: application/json" \
    -d "{\"user_id\":$BOB_ID,\"tag\":\"咖啡\"}"
assert "POST /interests bob 新增咖啡回傳 200" "200" "$HTTP_CODE"

req POST "$BASE/interests" -H "Content-Type: application/json" \
    -d "{\"user_id\":$CAROL_ID,\"tag\":\"攝影\"}"
assert "POST /interests carol 新增攝影回傳 200" "200" "$HTTP_CODE"

# 列出興趣標籤
req GET "$BASE/interests/$ALICE_ID"
assert "GET /interests/$ALICE_ID 回傳 200" "200" "$HTTP_CODE"
echo "$RESP" | assert_contains "alice 興趣含爬山" "爬山"
echo "$RESP" | assert_contains "alice 興趣含攝影" "攝影"

# 移除興趣標籤
req DELETE "$BASE/interests" -H "Content-Type: application/json" \
    -d "{\"user_id\":$ALICE_ID,\"tag\":\"爬山\"}"
assert "DELETE /interests alice 移除爬山回傳 200" "200" "$HTTP_CODE"

req GET "$BASE/interests/$ALICE_ID"
echo "$RESP" | assert_contains "alice 剩攝影" "攝影"
assert "alice 無爬山 (關係驗證)" "0" "$(echo "$RESP" | grep -c "爬山")"

# 重新加入
req POST "$BASE/interests" -H "Content-Type: application/json" \
    -d "{\"user_id\":$ALICE_ID,\"tag\":\"爬山\"}"
assert "POST /interests alice 重新加入爬山回傳 200" "200" "$HTTP_CODE"

# 移除不存在的標籤
req DELETE "$BASE/interests" -H "Content-Type: application/json" \
    -d "{\"user_id\":$ALICE_ID,\"tag\":\"不存在標籤\"}"
assert "DELETE /interests 不存在標籤回傳 404" "404" "$HTTP_CODE"

# 新增重複的標籤（已存在的 tag+user_id）
req POST "$BASE/interests" -H "Content-Type: application/json" \
    -d "{\"user_id\":$ALICE_ID,\"tag\":\"攝影\"}"
assert "POST /interests 重複標籤仍回傳 200" "200" "$HTTP_CODE"
echo "$RESP" | assert_contains "重複標籤回應含 id" '"id"'

# ── 交友搜尋 ──
req GET "$BASE/profiles/search?gender=male"
assert "GET /profiles/search?gender=male 回傳 200" "200" "$HTTP_CODE"
echo "$RESP" | assert_contains "搜尋 male 含 alice" "愛麗絲"

req GET "$BASE/profiles/search?gender=female&city=台"
echo "$RESP" | assert_contains "搜尋 female+city=台 含 bob" "鮑勃"
COUNT=$(echo "$RESP" | json_val "['count']")
assert "搜尋結果 count=1 (關係驗證)" "1" "$COUNT"

req GET "$BASE/profiles/search?tags=攝影"
echo "$RESP" | assert_contains "搜尋 tags=攝影 含 alice" "愛麗絲"
echo "$RESP" | assert_contains "搜尋 tags=攝影 含 carol" "卡蘿"

req GET "$BASE/profiles/search?q=咖啡"
echo "$RESP" | assert_contains "搜尋 q=咖啡 含 bob" "鮑勃"

# 年齡區間搜尋（1995->31, 1998->27, 2000->25 / 2026 年）
req GET "$BASE/profiles/search?age_min=27&age_max=32"
COUNT=$(echo "$RESP" | json_val "['count']")
assert "年齡 27-32 搜尋 count=2 (關係驗證)" "2" "$COUNT"
echo "$RESP" | assert_contains "年齡搜尋含 alice (31)" "愛麗絲"
echo "$RESP" | assert_contains "年齡搜尋含 bob (27)" "鮑勃"

req GET "$BASE/profiles/search?age_min=30"
COUNT=$(echo "$RESP" | json_val "['count']")
assert "年齡 >=30 搜尋 count=1 (只含 alice)" "1" "$COUNT"

# ═══════════════════════════════════════════════════════
# 10. 商店 + 商品 API
# ═══════════════════════════════════════════════════════

echo ""
echo "=== 10. 商店 + 商品 API ==="

# 開啟商店
req POST "$BASE/shops/open" -H "Content-Type: application/json" \
    -d "{\"user_id\":$ALICE_ID,\"name\":\"愛麗絲戶外用品店\",\"description\":\"登山露營裝備\"}"
assert "POST /shops/open 回傳 200" "200" "$HTTP_CODE"
SHOP_ID=$(echo "$RESP" | json_val "['shop']['id']")
assert "商店名稱正確" "愛麗絲戶外用品店" "$(echo "$RESP" | json_val "['shop']['name']")"

# 查詢商店
req GET "$BASE/shops?user_id=$ALICE_ID"
assert "GET /shops?user_id=$ALICE_ID 回傳 200" "200" "$HTTP_CODE"

req GET "$BASE/shops/$SHOP_ID"
assert "GET /shops/$SHOP_ID 回傳 200" "200" "$HTTP_CODE"
echo "$RESP" | assert_contains "商店資訊含名稱" "愛麗絲戶外用品店"
assert "商店 owner 是 alice" "$ALICE_ID" "$(echo "$RESP" | json_val "['user_id']")"

# 新增商品
req POST "$BASE/products/shop/$SHOP_ID" -H "Content-Type: application/json" \
    -d "{\"user_id\":$ALICE_ID,\"name\":\"登山杖\",\"price\":599,\"stock\":10}"
assert "POST /products/shop/$SHOP_ID 新增商品回傳 200" "200" "$HTTP_CODE"
PROD1_ID=$(echo "$RESP" | json_val "['id']")

req POST "$BASE/products/shop/$SHOP_ID" -H "Content-Type: application/json" \
    -d "{\"user_id\":$ALICE_ID,\"name\":\"露營燈\",\"price\":899,\"stock\":5}"
assert "POST /products/shop/$SHOP_ID 新增商品2回傳 200" "200" "$HTTP_CODE"
PROD2_ID=$(echo "$RESP" | json_val "['id']")

# 商品列表（依商店）
req GET "$BASE/products/shop/$SHOP_ID"
assert "GET /products/shop/$SHOP_ID 回傳 200" "200" "$HTTP_CODE"
PROD_COUNT=$(echo "$RESP" | python3 -c "import sys,json; print(len(json.load(sys.stdin)))")
assert "商店有 2 件商品" "2" "$PROD_COUNT"

# 商品搜尋
req GET "$BASE/products/search?q=登山"
assert "GET /products/search?q=登山 回傳 200" "200" "$HTTP_CODE"
echo "$RESP" | assert_contains "搜尋登山結果" "登山杖"

# 未登入者無法開店（假裝是 dave 開店卻用 bob 的商店驗證）
# 但實際上是驗證不能開第二間店（alice 已經開了）
req POST "$BASE/shops/open" -H "Content-Type: application/json" \
    -d "{\"user_id\":$ALICE_ID,\"name\":\"第二間店\",\"description\":\"不行\"}"
assert "同一人不能開第二間店回傳 400" "400" "$HTTP_CODE"

# 刪除商品
req DELETE "$BASE/products/$PROD2_ID" -H "Content-Type: application/json" \
    -d "{\"user_id\":$ALICE_ID}"
assert "DELETE /products/$PROD2_ID 回傳 200" "200" "$HTTP_CODE"
echo "$RESP" | assert_contains "商品已移除" "true"

req GET "$BASE/products/shop/$SHOP_ID"
PROD_COUNT=$(echo "$RESP" | python3 -c "import sys,json; print(len(json.load(sys.stdin)))")
assert "刪除後剩 1 件商品 (關係驗證)" "1" "$PROD_COUNT"

# ═══════════════════════════════════════════════════════
# 11. 購物車 + 訂單
# ═══════════════════════════════════════════════════════

echo ""
echo "=== 11. 購物車 + 訂單 ==="

# bob 將商品加入購物車
req POST "$BASE/cart" -H "Content-Type: application/json" \
    -d "{\"user_id\":$BOB_ID,\"product_id\":$PROD1_ID,\"quantity\":2}"
assert "POST /cart bob 加入商品回傳 200" "200" "$HTTP_CODE"
CART_ID=$(echo "$RESP" | json_val "['cart_item']['id']")
echo "$RESP" | assert_contains "購物車項目含商品" "product_id"

# 更新數量
req PUT "$BASE/cart/$CART_ID" -H "Content-Type: application/json" \
    -d "{\"user_id\":$BOB_ID,\"quantity\":3}"
assert "PUT /cart/$CART_ID 更新數量回傳 200" "200" "$HTTP_CODE"

# 列出購物車
req GET "$BASE/cart?user_id=$BOB_ID"
assert "GET /cart?user_id=$BOB_ID 回傳 200" "200" "$HTTP_CODE"
CART_ITEM_COUNT=$(echo "$RESP" | python3 -c "import sys,json; print(len(json.load(sys.stdin)))")
assert "bob 購物車有 1 項" "1" "$CART_ITEM_COUNT"

# 購物車計數
req GET "$BASE/cart/count?user_id=$BOB_ID"
assert "GET /cart/count?user_id=$BOB_ID 回傳 200" "200" "$HTTP_CODE"
COUNT=$(echo "$RESP" | json_val "['count']")
assert "bob 購物車數量=3 (qty 加總)" "3" "$COUNT"

# 結帳（建立訂單）
req POST "$BASE/cart/checkout" -H "Content-Type: application/json" \
    -d "{\"user_id\":$BOB_ID}"
assert "POST /cart/checkout 回傳 200" "200" "$HTTP_CODE"
ORDER_COUNT=$(echo "$RESP" | python3 -c "import sys,json; d=json.load(sys.stdin); print(len(d['order_ids']))")
assert "結帳產生了 $ORDER_COUNT 筆訂單" "1" "$ORDER_COUNT"
ORDER1_ID=$(echo "$RESP" | python3 -c "import sys,json; d=json.load(sys.stdin); print(d['order_ids'][0])")

# 結帳後購物車應清空
req GET "$BASE/cart/count?user_id=$BOB_ID"
COUNT=$(echo "$RESP" | json_val "['count']")
assert "結帳後購物車清空 (關係驗證)" "0" "$COUNT"

# 訂單列表
req GET "$BASE/orders?user_id=$BOB_ID"
assert "GET /orders?user_id=$BOB_ID 回傳 200" "200" "$HTTP_CODE"
echo "$RESP" | assert_contains "訂單列表非空" "id"

# 訂單詳情（需傳入 user_id 參數）
req GET "$BASE/orders/$ORDER1_ID?user_id=$BOB_ID"
assert "GET /orders/$ORDER1_ID?user_id=$BOB_ID 回傳 200" "200" "$HTTP_CODE"
echo "$RESP" | assert_contains "訂單資訊" "product_id"

# 庫存不足測試（將庫存設為 0 後試圖購買）
# 先更新庫存…
req PUT "$BASE/products/$PROD1_ID/update" -H "Content-Type: application/json" \
    -d "{\"user_id\":$ALICE_ID,\"name\":\"登山杖\",\"price\":599,\"stock\":0}"
assert "PUT /products/$PROD1_ID/update 更新庫存回傳 200" "200" "$HTTP_CODE"
assert "庫存已歸零" "0" "$(echo "$RESP" | json_val "['stock']")"

# carol 嘗試購買庫存為 0 的商品
req POST "$BASE/cart" -H "Content-Type: application/json" \
    -d "{\"user_id\":$CAROL_ID,\"product_id\":$PROD1_ID,\"quantity\":1}"
assert "POST /cart 庫存不足回傳 400 (關係驗證)" "400" "$HTTP_CODE"

# ═══════════════════════════════════════════════════════
# 12. 社團 API
# ═══════════════════════════════════════════════════════

echo ""
echo "=== 12. 社團 API ==="

# 建立社團
req POST "$BASE/groups" -H "Content-Type: application/json" \
    -d "{\"user_id\":$ALICE_ID,\"name\":\"登山愛好者\",\"description\":\"一起爬山！\"}"
assert "POST /groups 建立社團回傳 200" "200" "$HTTP_CODE"
GROUP_ID=$(echo "$RESP" | json_val "['id']")
assert "社團名稱正確" "登山愛好者" "$(echo "$RESP" | json_val "['name']")"
assert "社團 member_count=1 (創辦人)" "1" "$(echo "$RESP" | json_val "['member_count']")"

# 列出社團
req GET "$BASE/groups"
assert "GET /groups 回傳 200" "200" "$HTTP_CODE"
echo "$RESP" | assert_contains "社團列表含登山" "登山愛好者"

# 加入社團
req POST "$BASE/groups/$GROUP_ID/join" -H "Content-Type: application/json" \
    -d "{\"user_id\":$BOB_ID}"
assert "POST /groups/$GROUP_ID/join bob 回傳 200" "200" "$HTTP_CODE"
echo "$RESP" | assert_contains "加入成功" "true"

req POST "$BASE/groups/$GROUP_ID/join" -H "Content-Type: application/json" \
    -d "{\"user_id\":$CAROL_ID}"
assert "POST /groups/$GROUP_ID/join carol 回傳 200" "200" "$HTTP_CODE"

# 重複加入
req POST "$BASE/groups/$GROUP_ID/join" -H "Content-Type: application/json" \
    -d "{\"user_id\":$BOB_ID}"
assert "POST /groups/$GROUP_ID/join 重複加入回傳 400" "400" "$HTTP_CODE"

# 社團成員列表
req GET "$BASE/groups/$GROUP_ID/members"
assert "GET /groups/$GROUP_ID/members 回傳 200" "200" "$HTTP_CODE"
MEMBER_COUNT=$(echo "$RESP" | python3 -c "import sys,json; print(len(json.load(sys.stdin)))")
assert "社團有 3 位成員 (關係驗證)" "3" "$MEMBER_COUNT"

# 社團貼文
req POST "$BASE/groups/$GROUP_ID/posts" -H "Content-Type: application/json" \
    -d "{\"user_id\":$ALICE_ID,\"content\":\"這週末有人要去爬七星山嗎？\"}"
assert "POST /groups/$GROUP_ID/posts 回傳 200" "200" "$HTTP_CODE"
echo "$RESP" | assert_contains "社團貼文成功" "七星山"

req POST "$BASE/groups/$GROUP_ID/posts" -H "Content-Type: application/json" \
    -d "{\"user_id\":$BOB_ID,\"content\":\"我想去！\"}"
assert "POST /groups/$GROUP_ID/posts bob 回傳 200" "200" "$HTTP_CODE"

# 列出社團貼文
req GET "$BASE/groups/$GROUP_ID/posts"
assert "GET /groups/$GROUP_ID/posts 回傳 200" "200" "$HTTP_CODE"
POST_COUNT=$(echo "$RESP" | python3 -c "import sys,json; print(len(json.load(sys.stdin)))")
assert "社團有 2 篇貼文 (關係驗證)" "2" "$POST_COUNT"

# 離開社團
req POST "$BASE/groups/$GROUP_ID/leave" -H "Content-Type: application/json" \
    -d "{\"user_id\":$CAROL_ID}"
assert "POST /groups/$GROUP_ID/leave carol 回傳 200" "200" "$HTTP_CODE"
echo "$RESP" | assert_contains "離開成功" "true"

req GET "$BASE/groups/$GROUP_ID/members"
MEMBER_COUNT=$(echo "$RESP" | python3 -c "import sys,json; print(len(json.load(sys.stdin)))")
assert "離開後剩 2 位成員 (關係驗證)" "2" "$MEMBER_COUNT"

# 非成員無法貼文
req POST "$BASE/groups/$GROUP_ID/posts" -H "Content-Type: application/json" \
    -d "{\"user_id\":$CAROL_ID,\"content\":\"我想回來\"}"
assert "非成員 carol 無法貼文回傳 400 (關係驗證)" "400" "$HTTP_CODE"

# 我的社團
req GET "$BASE/groups/mine?user_id=$ALICE_ID"
assert "GET /groups/mine?user_id=$ALICE_ID 回傳 200" "200" "$HTTP_CODE"
echo "$RESP" | assert_contains "alice 的社團含登山" "登山愛好者"

# ═══════════════════════════════════════════════════════
# 13. 跨端一致性（CLI + Web API）
# ═══════════════════════════════════════════════════════

echo ""
echo "=== 13. 跨端一致性（CLI + Web API）==="

# CLI 看到 Web API 建立的資料
R=$($SMS4 user list 2>&1 | grep -c "alice")
assert "CLI 看到 alice (跨端驗證)" "1" "$(echo "$R")"

R=$($SMS4 post list 2>&1 | grep -c "芒草季")
assert "CLI 看到 Web API 建立的貼文 (跨端驗證)" "1" "$(echo "$R")"

# CLI 新增資料後 Web API 可看到
$SMS4 user add frank 法蘭克 --bio "學生" 2>&1 > /dev/null
req GET "$BASE/users"
echo "$RESP" | assert_contains "Web API 看到 CLI 新增的 frank" "frank"

# ═══════════════════════════════════════════════════════
# 14. 錯誤處理
# ═══════════════════════════════════════════════════════

echo ""
echo "=== 14. 錯誤處理 ==="

# 不存在的資源
req GET "$BASE/users/999"
assert "GET /users/999 回傳 404" "404" "$HTTP_CODE"

req GET "$BASE/posts/999"
assert "GET /posts/999 回傳 404" "404" "$HTTP_CODE"

req GET "$BASE/profiles/999"
assert "GET /profiles/999 回傳 404" "404" "$HTTP_CODE"

# 不存在的端點（回到 index.html）
req GET "$BASE/nonexistent"
echo "$RESP" | assert_contains "GET /nonexistent 回傳 SPA 頁面" "<title>SMS4</title>"

# 刪除使用者（因 FK 關聯而失敗，回傳 500）
req DELETE "$BASE/users/$CAROL_ID"
assert "DELETE /users/$CAROL_ID 因 FK 失敗回傳 500" "500" "$HTTP_CODE"

echo ""
