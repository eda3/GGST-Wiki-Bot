// 標準ライブラリのenv（環境変数を読む）を使う
use std::env;

// .envファイルを読み込むためのクレート（ライブラリ）
use dotenvy::dotenv;

// HTTPリクエストを送るためのクライアントを使う
use reqwest::Client;

// JSON形式のデータを扱うためのヘルパー
use serde_json::json;

// エラーを扱いやすくするための型定義
use std::error::Error;

// メイン関数（非同期）
// `#[tokio::main]` は「非同期処理を動かすためのマクロ（魔法みたいなやつ）」
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // `.env` ファイルを読み込んで、環境変数として使えるようにする
    dotenv().ok();

    // 環境変数 "GRAPHQL_ENDPOINT" を取得（必須）
    // 例: https://ggst.eda3.dev/graphql
    let endpoint = env::var("GRAPHQL_ENDPOINT")?;

    // 環境変数 "AUTH_TOKEN" を取得（あれば使う）
    // 空の場合はそのまま進む
    let token = env::var("AUTH_TOKEN").unwrap_or_default();

    // ここで GraphQL に送るクエリ（問い合わせ）を定義
    // 今回はページの一覧（id・タイトル・パス）を取得したい
    let query = json!({
        "query": "{ pages { list { id title path } } }"
    });

    // HTTPクライアントを作成（リクエストを送るための準備）
    let client = Client::new();

    // POSTリクエストを組み立てる
    // - URLはendpoint
    // - Content-TypeはJSON
    // - 中身は query（GraphQL形式の文字列）
    let mut request = client
        .post(&endpoint)
        .header("Content-Type", "application/json")
        .json(&query);

    // 認証トークンが設定されていれば、ヘッダーに追加する
    // "Authorization: Bearer {トークン}" の形式にする必要がある
    if !token.is_empty() {
        request = request.header("Authorization", format!("Bearer {}", token));
    }

    // サーバにリクエストを送信して、レスポンスを受け取る（非同期処理）
    let response = request.send().await?;

    // ステータスコードが「成功（200 OK）」じゃなかったらエラーメッセージを出す
    if !response.status().is_success() {
        eprintln!("GraphQL request failed: {}", response.status());

        // サーバからのエラーメッセージ本文を表示
        let text = response.text().await?;
        eprintln!("Error response: {}", text);

        // 処理を終わる（エラーではなく正常終了）
        return Ok(());
    }

    // サーバから返ってきたデータをJSON形式で読み込む
    let json_response: serde_json::Value = response.json().await?;

    // 画面にきれいに整形して出力（デバッグ用）
    println!("{}", serde_json::to_string_pretty(&json_response)?);

    // 正常終了
    Ok(())
}

