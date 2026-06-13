#!/usr/bin/env bash
# seed.sh — 填入大量測試假資料
set -euo pipefail

SMS4=${SMS4:-cargo run --}
DB="sms4-dev.db"

rm -f $DB

echo "=== 1. 初始化資料庫 ==="
SMS4_DB="$DB" $SMS4 init

echo ""
echo "=== 2. 建立使用者 (15 筆) ==="
SMS4_DB="$DB" $SMS4 user add alice    愛麗絲   --bio "喜歡旅行和攝影 📸"
SMS4_DB="$DB" $SMS4 user add bob      鮑勃     --bio "程式設計師 & 咖啡愛好者"
SMS4_DB="$DB" $SMS4 user add carol    卡蘿     --bio "美食部落客"
SMS4_DB="$DB" $SMS4 user add dave     大衛     --bio "音樂創作者"
SMS4_DB="$DB" $SMS4 user add eve      小伊     --bio "瑜伽老師"
SMS4_DB="$DB" $SMS4 user add frank    法蘭克   --bio "股票投資人"
SMS4_DB="$DB" $SMS4 user add grace    葛蕾絲   --bio "插畫家"
SMS4_DB="$DB" $SMS4 user add henry    亨利     --bio "健身教練"
SMS4_DB="$DB" $SMS4 user add iris     艾瑞絲   --bio "美妝達人"
SMS4_DB="$DB" $SMS4 user add jack     傑克     --bio "旅遊部落客"
SMS4_DB="$DB" $SMS4 user add kate     凱特     --bio "書評作家"
SMS4_DB="$DB" $SMS4 user add leo      里歐     --bio "寵物攝影師"
SMS4_DB="$DB" $SMS4 user add may      小梅     --bio "手作達人"
SMS4_DB="$DB" $SMS4 user add nick     尼克     --bio "科技評論"
SMS4_DB="$DB" $SMS4 user add olivia   奧莉維亞 --bio "環保倡議者"

echo ""
echo "=== 3. 建立貼文 (30 筆) ==="

SMS4_DB="$DB" $SMS4 post add 1 "今天天氣真好，去陽明山走了一趟！滿山遍野的芒草超美"
SMS4_DB="$DB" $SMS4 post add 1 "剛看完《原子習慣》，推薦給想要改變生活的人 📚"
SMS4_DB="$DB" $SMS4 post add 2 "Rust 的 borrow checker 真是令人又愛又恨 😅 但寫出來的程式就是安心"
SMS4_DB="$DB" $SMS4 post add 2 "今天喝了一杯衣索比亞 74158 品種的咖啡，果香炸裂！"
SMS4_DB="$DB" $SMS4 post add 3 "台北東區新開的甜點店「甜蜜時光」，提拉米蘇入口即化！"
SMS4_DB="$DB" $SMS4 post add 3 "自製番茄肉醬義大利麵，簡單又好吃 🍝"
SMS4_DB="$DB" $SMS4 post add 4 "新寫了一首鋼琴曲，靈感來自雨天的窗景 🎹"
SMS4_DB="$DB" $SMS4 post add 5 "清晨的瑜珈練習，讓一整天都充滿能量 🧘"
SMS4_DB="$DB" $SMS4 post add 6 "台積電法說會重點整理，毛利率表現優於預期 📈"
SMS4_DB="$DB" $SMS4 post add 7 "新作品完成！水彩畫了一隻慵懶的貓咪 🎨"
SMS4_DB="$DB" $SMS4 post add 8 "深蹲 5x5 挑戰第二週，開始感受到進步了 💪"
SMS4_DB="$DB" $SMS4 post add 9 "今年春夏眼影趨勢：蜜桃色系和大地色系依然主流"
SMS4_DB="$DB" $SMS4 post add 10 "日本京都賞楓攻略，必去景點總整理 🍁"
SMS4_DB="$DB" $SMS4 post add 11 "讀完《挪威的森林》，村上的文字還是那麼令人沉浸"
SMS4_DB="$DB" $SMS4 post add 12 "今天幫一隻黃金獵犬拍寫真，牠的笑容太治癒了 🐕"
SMS4_DB="$DB" $SMS4 post add 13 "新做的皮革零錢包，手縫真的很療癒"
SMS4_DB="$DB" $SMS4 post add 14 "Apple Vision Pro 的空間運算到底是不是下一個風口？"
SMS4_DB="$DB" $SMS4 post add 15 "週末去海邊淨灘，一個小時撿了 5 公斤的垃圾 🌊"
SMS4_DB="$DB" $SMS4 post add 1 "分享一個陽明山秘境：冷水坑往擎天崗的路上"
SMS4_DB="$DB" $SMS4 post add 2 "VS Code 外掛推薦：Rust Analyzer 真的是必裝"
SMS4_DB="$DB" $SMS4 post add 3 "高雄新開的咖啡廳，水泥風格裝潢很適合拍照"
SMS4_DB="$DB" $SMS4 post add 5 "空中瑜珈初體驗，比想像中難好多但很好玩！"
SMS4_DB="$DB" $SMS4 post add 6 "指數投資 vs 主動選股，我的配置心得"
SMS4_DB="$DB" $SMS4 post add 10 "台東池上的稻田真的太美了，台灣的後花園 🌾"
SMS4_DB="$DB" $SMS4 post add 12 "貓咪到底為什麼那麼喜歡紙箱？🐱"
SMS4_DB="$DB" $SMS4 post add 14 "GitHub Copilot 真的讓寫程式效率提升不少"
SMS4_DB="$DB" $SMS4 post add 15 "今天開始挑戰 zero waste 生活，第一週目標減少塑膠"
SMS4_DB="$DB" $SMS4 post add 1 "攝影筆記：清晨的黃金時刻光線最美，推薦大家試試"
SMS4_DB="$DB" $SMS4 post add 2 "為什麼 Rust 的 Option 和 Result 讓錯誤處理這麼優雅？"
SMS4_DB="$DB" $SMS4 post add 3 "台中第二市場的滷肉飯，排隊半小時值得！"

echo ""
echo "=== 4. 回覆貼文 ==="
SMS4_DB="$DB" $SMS4 post reply 1 2 "陽明山現在花況如何？想找時間去走走"
SMS4_DB="$DB" $SMS4 post reply 1 3 "求推薦登山路線！"
SMS4_DB="$DB" $SMS4 post reply 1 5 "建議清晨去，人比較少"
SMS4_DB="$DB" $SMS4 post reply 3 1 "我也在學 Rust，真的很有挑戰性"
SMS4_DB="$DB" $SMS4 post reply 3 3 "在哪裡？想吃！有地址嗎？"
SMS4_DB="$DB" $SMS4 post reply 3 5 "改天一起去吃！"
SMS4_DB="$DB" $SMS4 post reply 7 10 "好美的描述，讓我也想去京都了"
SMS4_DB="$DB" $SMS4 post reply 10 1 "京都是四季都適合去的地方！"
SMS4_DB="$DB" $SMS4 post reply 11 1 "我也很喜歡這本！"
SMS4_DB="$DB" $SMS4 post reply 12 7 "黃金獵犬的笑容真的無敵"

echo ""
echo "=== 5. 建立追蹤關係 ==="
# alice 追蹤 bob, carol, eve, jack, kate
SMS4_DB="$DB" $SMS4 follow add 1 2
SMS4_DB="$DB" $SMS4 follow add 1 3
SMS4_DB="$DB" $SMS4 follow add 1 5
SMS4_DB="$DB" $SMS4 follow add 1 10
SMS4_DB="$DB" $SMS4 follow add 1 11
# bob 追蹤 alice, carol, dave, frank, nick
SMS4_DB="$DB" $SMS4 follow add 2 1
SMS4_DB="$DB" $SMS4 follow add 2 3
SMS4_DB="$DB" $SMS4 follow add 2 4
SMS4_DB="$DB" $SMS4 follow add 2 6
SMS4_DB="$DB" $SMS4 follow add 2 14
# carol 追蹤 alice, bob, grace, iris, may
SMS4_DB="$DB" $SMS4 follow add 3 1
SMS4_DB="$DB" $SMS4 follow add 3 2
SMS4_DB="$DB" $SMS4 follow add 3 7
SMS4_DB="$DB" $SMS4 follow add 3 9
SMS4_DB="$DB" $SMS4 follow add 3 13
# 其他人隨機追蹤
SMS4_DB="$DB" $SMS4 follow add 4 1
SMS4_DB="$DB" $SMS4 follow add 4 2
SMS4_DB="$DB" $SMS4 follow add 5 1
SMS4_DB="$DB" $SMS4 follow add 5 3
SMS4_DB="$DB" $SMS4 follow add 6 2
SMS4_DB="$DB" $SMS4 follow add 6 14
SMS4_DB="$DB" $SMS4 follow add 7 1
SMS4_DB="$DB" $SMS4 follow add 8 1
SMS4_DB="$DB" $SMS4 follow add 8 2
SMS4_DB="$DB" $SMS4 follow add 9 3
SMS4_DB="$DB" $SMS4 follow add 10 1
SMS4_DB="$DB" $SMS4 follow add 10 2
SMS4_DB="$DB" $SMS4 follow add 11 1
SMS4_DB="$DB" $SMS4 follow add 11 10
SMS4_DB="$DB" $SMS4 follow add 12 1
SMS4_DB="$DB" $SMS4 follow add 13 7
SMS4_DB="$DB" $SMS4 follow add 14 2
SMS4_DB="$DB" $SMS4 follow add 14 1
SMS4_DB="$DB" $SMS4 follow add 15 1

echo ""
echo "=== 6. 按讚 ==="
# 貼文 1 (愛麗絲的陽明山) 獲得 5 個讚
SMS4_DB="$DB" $SMS4 like add 2 1
SMS4_DB="$DB" $SMS4 like add 3 1
SMS4_DB="$DB" $SMS4 like add 5 1
SMS4_DB="$DB" $SMS4 like add 10 1
SMS4_DB="$DB" $SMS4 like add 11 1

# 貼文 3 (bob 的 Rust) 獲得 4 個讚
SMS4_DB="$DB" $SMS4 like add 1 3
SMS4_DB="$DB" $SMS4 like add 3 3
SMS4_DB="$DB" $SMS4 like add 14 3
SMS4_DB="$DB" $SMS4 like add 6 3

# 貼文 5 (carol 的甜點) 獲得 3 個讚
SMS4_DB="$DB" $SMS4 like add 1 5
SMS4_DB="$DB" $SMS4 like add 2 5
SMS4_DB="$DB" $SMS4 like add 9 5

# 貼文 7 (dave 的鋼琴曲) 獲得 3 個讚
SMS4_DB="$DB" $SMS4 like add 1 7
SMS4_DB="$DB" $SMS4 like add 2 7
SMS4_DB="$DB" $SMS4 like add 10 7

echo ""
echo "=== 假資料填入完成 ==="
echo ""
echo "  使用者: 15 筆"
echo "  貼文:   30 筆（含 10 則回覆）"
echo "  追蹤:   35 筆"
echo "  按讚:   15 筆"
echo ""
echo "啟動互動："
echo "  SMS4_DB=$DB cargo run -- post timeline 1"
echo "  SMS4_DB=$DB cargo run -- post get 1"
