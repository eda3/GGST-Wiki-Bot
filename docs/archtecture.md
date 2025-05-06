# GGST-Wiki-Bot アーキテクチャ設計書

## 概要

このプロジェクトは、Rustで構築された非同期GraphQLクライアントBotであり、Wiki.jsのGraphQL APIを通じて、特定のページ情報を取得・加工・表示することを目的としています。今後、Discord Botとの連携を視野に入れています。

---

## 全体構成（モジュール分割）

```

src/
├── main.rs               // エントリポイント。非同期タスクの起動と全体制御
├── config.rs             // .envファイルからの環境変数読み込み
├── wiki\_client.rs        // GraphQLリクエストの送受信を行うAPIクライアント
├── model.rs              // GraphQLレスポンス用の構造体定義
├── error.rs              // 共通エラー型と簡易ハンドリング

```

---

## 非同期タスク構成（tokioベース）

- `main.rs` は `#[tokio::main]` により非同期実行環境を提供
- `wiki_client.rs` の `fetch_page_list()` などは `async fn` として非同期呼び出し可能
- `reqwest::Client` はシングルトンで生成し、各モジュールに渡す形を検討（Arc化予定）

---

## モジュール責務一覧

### `config.rs`

- `.env` ファイルの読み込み
- 必須値（APIエンドポイント、トークンなど）のバリデーション
- `struct AppConfig` で型安全に扱えるようにする

### `wiki_client.rs`

- GraphQLクエリを構築・送信・レスポンス処理
- クエリは `serde_json::json!()` を使って記述
- 今後、ページ取得以外のクエリ追加も想定

### `model.rs`

- GraphQLのレスポンスJSONに対応した構造体
- `#[derive(Deserialize)]` を使い、`serde_json` でマッピング
- `PageListResponse`, `PageItem` などを定義

### `error.rs`

- カスタムエラー型 `BotError` の定義（reqwest/serde/ioなどのラップ）
- `Result<T, BotError>` 形式で関数間のエラー共有を簡潔化

---

## 初期フェーズの流れ（ページ一覧取得）

1. `.env` 読み込み → `AppConfig` 構築（config.rs）
2. HTTPクライアント生成（wiki_client.rs）
3. `fetch_page_list()` 実行 → GraphQL APIへPOST
4. レスポンスを `model.rs` の構造体でデシリアライズ
5. 結果を `println!` などで整形出力

---

## 今後の展望（将来的追加予定）

- Discord Bot統合（twilightを使用予定）
- Wiki.jsのページ更新機能の追加（GraphQL Mutation）
- Rustの `axum` や `tower` を使ったHTTPエンドポイント提供（Webhook対応など）
- 自動投稿・キャッシュ機構・編集通知など

---

## 設計思想（関数型っぽく）

- 各モジュールは「純粋関数的」に入力→出力を意識
- 状態共有は `Arc<Mutex<>>` により明示的に制御（副作用の見える化）
- `Result<T, E>` を中心に据えたエラーハンドリングの簡素化と明示化
- 非同期タスクと関数の境界は、原則として型レベルで明示的に管理

---

## 使用ライブラリ

- `dotenvy`：環境変数の読み込み
- `reqwest`：非同期HTTPクライアント
- `serde / serde_json`：構造体とJSONの相互変換
- `tokio`：非同期ランタイム（デフォルト）

---

## 補足

この設計書は今後の拡張に備えて柔軟に更新されていきます。PR歓迎。