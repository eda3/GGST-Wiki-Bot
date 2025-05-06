## 1. コーディング規約

### 1.1 Rust スタイルガイドに準拠

* [Rust公式スタイルガイド](https://doc.rust-lang.org/1.0.0/style/README.html) に従うこと
* API 設計の観点では [Rust API Design Guidelines](https://rust-lang.github.io/api-guidelines/) も参照

### 1.2 名前の付け方

| 要素名       | 命名規則                   | 例               |
| --------- | ---------------------- | --------------- |
| モジュール     | `snake_case`           | `graphql_query` |
| 構造体 / 列挙型 | `PascalCase`           | `PageResponse`  |
| 関数名       | `snake_case`           | `fetch_pages`   |
| 定数        | `SCREAMING_SNAKE_CASE` | `MAX_LIMIT`     |
| マクロ       | `snake_case!`          | `println!`      |

### 1.3 ドキュメントコメント

* 公開される関数や型には `///` を使って意味や引数、戻り値を明記する

```rust
/// Wikiのページ情報を格納する構造体
///
/// GraphQL APIから受け取った情報を表す
pub struct Page {
    pub id: usize,
    pub title: String,
    pub path: String,
}

impl Page {
    /// ページタイトルを表示する
    pub fn print_title(&self) {
        println!("タイトル: {}", self.title);
    }
}
```

---

## 2. エラーハンドリング

### 2.1 `panic!` を避け、`Result` を使う

* 本番コードでは基本的に `panic!` を使わない
* 失敗の可能性がある処理は `Result<T, E>` を返す
* `?` 演算子を使って簡潔にエラーを伝播させる

```rust
fn parse_config(json: &str) -> Result<Config, String> {
    let config: Config = serde_json::from_str(json)
        .map_err(|e| format!("設定ファイルの読み込み失敗: {e}"))?;
    Ok(config)
}
```

### 2.2 明確なエラーメッセージを書く

* ユーザーや開発者が原因を特定できる内容にする

```rust
Err("GraphQLエンドポイントに接続できませんでした: URL不正".into())
```

### 2.3 `Option` の扱いは明示的に

* `unwrap()` は基本NG
* `unwrap_or` や `map`, `and_then` で安全に処理

```rust
let token = env::var("AUTH_TOKEN").ok().unwrap_or_default();
```

### 2.4 カスタムエラー型の定義

* 複数の種類のエラーを扱う場合は `enum` で分類しよう

```rust
#[derive(Debug)]
enum BotError {
    Config(String),
    Network(String),
    Unexpected(String),
}
```

### 2.5 JavaScriptや外部API由来のエラーはラップ

```rust
let value = js_sys::Reflect::get(&obj, &JsValue::from_str("token"))
    .map_err(|e| JsValue::from_str(&format!("JSの値取得失敗: {e:?}")))?;
```

### 2.6 結果を無視するときは `let _ =` 明示 + コメント

```rust
let _ = client.send_message(); // エラー無視: ログ用途の通知なので
```

### 2.7 ネストを減らすために早期リターンを活用

```rust
fn validate_token(token: &str) -> Result<(), String> {
    if token.is_empty() {
        return Err("トークンが空です".into());
    }
    Ok(())
}
```

---

## 3. テスト方針

### 3.1 単体テスト（ユニットテスト）

* モジュール内に `#[cfg(test)]` でテストを書く

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_title() {
        let page = Page { id: 1, title: "トップ".into(), path: "home".into() };
        assert_eq!(page.title, "トップ");
    }
}
```

### 3.2 エラーケースの検証

* 失敗すべき処理には `#[should_panic]` を使う（panic使用時）

### 3.3 エンドツーエンドテスト（E2E）

* Botのレスポンス、WebAPI連携は擬似的にモックして検証
* 将来的には Discord API モック or e2e botテスト環境を整備

---

## 4. 保守性・パフォーマンス

### 4.1 アロケーション削減

* 無駄なメモリ確保（例：Vecの再生成）を避ける

```rust
let mut logs = Vec::with_capacity(100);
```

### 4.2 バッチ処理

* WebAPIへの連続リクエストなどは可能な限りまとめて送る

### 4.3 非同期処理は `tokio` を使い `await` 駆動で組む

---

## 5. セキュリティ

### 5.1 入力値検証

* 外部からの入力（特にトークンやコマンド引数）は必ずチェック

### 5.2 エラーメッセージやログに機密情報を含めない

* トークンなどは絶対に表示しない

---

## 6. リファクタリング・レビュー指針

* 機能追加とリファクタは明確に分けてコミット
* 小さな単位で変更・確認できるようにする
* `cargo fmt`・`cargo clippy` を適用
* `Result` や `Option` の型扱いは慎重に

---

このスタイルガイドは、eda3が運営するRust製Discord Bot開発チーム「GGST-Wiki-Bot」における、**品質・保守性・安全性の共通基盤**として運用されます。

随時アップデートしていきます。
