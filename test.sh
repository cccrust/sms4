#!/usr/bin/env bash
# test.sh — 執行所有測試（單元測試 + API 整合測試 + 端到端測試）
set -uo pipefail

ROOT="$(cd "$(dirname "$0")" && pwd)"
ALL_PASS=0
ALL_FAIL=0
FAILED_SUITES=""

run_suite() {
    local name="$1" cmd="$2"
    echo ""
    echo "╔═══════════════════════════════════════════"
    echo "║  $name"
    echo "╚═══════════════════════════════════════════"
    echo ""

    set +e
    eval "$cmd"
    local rc=$?
    set -e

    if [ $rc -eq 0 ]; then
        ALL_PASS=$((ALL_PASS + 1))
        echo ""
        echo "  ✅ $name 通過"
    else
        ALL_FAIL=$((ALL_FAIL + 1))
        FAILED_SUITES="$FAILED_SUITES  - $name"$'\n'
        echo ""
        echo "  ❌ $name 失敗（exit code: $rc）"
    fi
    echo ""
    echo "─────────────────────────────────────────"
}

cd "$ROOT"

# ── 1. Rust 單元測試 ──
run_suite "Rust 單元測試 (cargo test)" "cargo test 2>&1"

# ── 2. Web API 整合測試 ──
run_suite "Web API 整合測試 (test_api.sh)" "./test_api.sh 2>&1"

# ── 3. 端到端測試 ──
run_suite "端到端整合測試 (test_e2e.sh)" "./test_e2e.sh 2>&1"

# ── 結果總計 ──
echo ""
echo "╔═══════════════════════════════════════════"
echo "║  測試結果總計"
echo "╚═══════════════════════════════════════════"
echo ""
echo "  通過: $ALL_PASS / 失敗: $ALL_FAIL / 總計: $((ALL_PASS + ALL_FAIL))"
echo ""

if [ $ALL_FAIL -gt 0 ]; then
    echo "❌ 以下測試套件失敗："
    echo -n "$FAILED_SUITES"
    exit 1
else
    echo "✅ 所有測試通過！"
    exit 0
fi
