# GGST-Wiki-Bot

**GGST初鯖攻略Wiki** を操作・自動化するための、Rust製のBotです。  
GraphQL APIを使って、ページの取得・整理・拡張などを行えます。

## 🔧 機能（予定含む）
- Wiki.jsのGraphQL API経由でページ一覧を取得
- タイトル、パス、タグ、作成者などのメタ情報を抽出
- Discord通知やGitHub Issues連携の自動化（予定）
- CLIやWeb UIでの検索機能の追加（検討中）

## 🛠 使用技術
- Rust（高速・安全な言語）
- GraphQL（柔軟なAPIクエリ）
- cargo（Rustのパッケージ＆ビルド管理）
- GitHub Actions（CI/CD予定）

## ▶️ 実行方法

```bash
git clone https://github.com/eda3/GGST-Wiki-Bot.git
cd GGST-Wiki-Bot
cargo run
````

※ `.env` ファイルなどの認証情報は別途設定が必要になる予定です。

## 🧠 このBotの背景

このBotは、Discordサーバー「[GGST初心者鯖](https://ggst.eda3.dev)」の
Wiki記事管理・拡張をより便利にするために開発されています。

## 📌 ライセンス

このプロジェクトは [MITライセンス](./LICENSE) で公開されています。
