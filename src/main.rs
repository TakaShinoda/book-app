use std::net::{Ipv4Addr, SocketAddr};

use anyhow::Result;
use axum::{http::StatusCode, routing::get, Router};
use tokio::net::TcpListener;

// ハンドラ
async fn hello_world() -> &'static str {
    "Hello, world"
}

pub async fn health_check() -> StatusCode {
    StatusCode::OK
}

// tokio ランタイム上で動かすために必要なマクロ、このマクロを使用することで main 関数を非同期化
#[tokio::main]
async fn main() -> Result<()> {
    // ルーターの設定
    let app = Router::new()
        .route("/hello", get(hello_world))
        .route("/health", get(health_check));

    // ローカルホスト 8080 番ポートでリクエストを待ち受ける
    let addr = SocketAddr::new(Ipv4Addr::LOCALHOST.into(), 8080);

    // 上記で設定したアドレスでバインドしたリスナーを立ち上げる
    let listener = TcpListener::bind(addr).await?;

    println!("Listening on {}", addr);

    // 起動する際に、ルーターを axum サーバーに登録する
    Ok(axum::serve(listener, app).await?)
}
