use std::net::{Ipv4Addr, SocketAddr};

use anyhow::Result;
use axum::{extract::State, http::StatusCode, routing::get, Router};
use tokio::net::TcpListener;

use sqlx::{postgres::PgConnectOptions, PgPool};

// データベースの接続設定を表す構造体
struct DatabaseConfig {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub database: String,
}

// アプリケーション用のデータベース設定構造体から、Postgres 接続用の構造体に変換する
// PgConnectOptions に From トレイトを実装する
impl From<DatabaseConfig> for PgConnectOptions {
    fn from(cfg: DatabaseConfig) -> Self {
        Self::new()
            .host(&cfg.host)
            .port(cfg.port)
            .username(&cfg.username)
            .password(&cfg.password)
            .database(&cfg.database)
    }
}

// Postgres 専用のコネクションプールを作成する
fn connect_database_with(cfg: DatabaseConfig) -> PgPool {
    PgPool::connect_lazy_with(cfg.into())
}

// ハンドラ
async fn hello_world() -> &'static str {
    "Hello, world"
}

// ヘルスチェックのハンドラ
async fn health_check() -> StatusCode {
    StatusCode::OK
}

// データベースのヘルスチェックを行うハンドラ
// State(db): State<PgPool>: with_state によって流し込まれた sqlx::PgPool
async fn health_check_db(State(db): State<PgPool>) -> StatusCode {
    let connection_result = sqlx::query("SELECT 1").fetch_one(&db).await;
    match connection_result {
        Ok(_) => StatusCode::OK,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

#[tokio::test]
async fn hello_world_works() {
    let response_message = hello_world().await;

    assert_eq!(response_message, "Hello, world");
}

// 非同期関数のテスト実行に tokio::test マクロを使う
#[tokio::test]
async fn health_check_works() {
    // health_check 関数を呼び出す。await して結果を得る
    let status_code = health_check().await;

    // 関数を実行した結果が、200 OK であることを確かめる
    assert_eq!(status_code, StatusCode::OK);
}

// sqlx 専用テストランナー
#[sqlx::test]
async fn health_check_db_works(pool: sqlx::PgPool) {
    let status_code = health_check_db(State(pool)).await;
    assert_eq!(status_code, StatusCode::OK);
}

// tokio ランタイム上で動かすために必要なマクロ、このマクロを使用することで main 関数を非同期化
#[tokio::main]
async fn main() -> Result<()> {
    // データベースへの接続情報を定義する
    let database_cfg = DatabaseConfig {
        host: "localhost".into(),
        port: 5432,
        username: "app".into(),
        password: "passwd".into(),
        database: "app".into(),
    };

    // コネクションプールを作成する
    let conn_pool = connect_database_with(database_cfg);

    // ルーターの設定
    let app = Router::new()
        .route("/hello", get(hello_world))
        .route("/health", get(health_check))
        .route("/health/db", get(health_check_db))
        .with_state(conn_pool); // ルーターの State にプールを登録し、各ハンドラで使えるようにする

    // ローカルホスト 8080 番ポートでリクエストを待ち受ける
    let addr = SocketAddr::new(Ipv4Addr::LOCALHOST.into(), 8080);

    // 上記で設定したアドレスでバインドしたリスナーを立ち上げる
    let listener = TcpListener::bind(addr).await?;

    println!("Listening on {}", addr);

    // 起動する際に、ルーターを axum サーバーに登録する
    Ok(axum::serve(listener, app).await?)
}
