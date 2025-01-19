use std::net::{Ipv4Addr, SocketAddr};

use adapter::database::connect_database_with;
use anyhow::{Error, Result};
use api::route::{book::build_book_routers, health::build_health_check_routers};
use axum::Router;
use registry::AppRegistry;
use shared::config::AppConfig;
use tokio::net::TcpListener;

// tokio ランタイム上で動かすために必要なマクロ、このマクロを使用することで main 関数を非同期化
#[tokio::main]
async fn main() -> Result<()> {
    bootstrap().await
}

async fn bootstrap() -> Result<()> {
    // AppConfig の生成
    let app_config = AppConfig::new()?;

    // データーベース接続を行うコネクションプールを取り出し
    let pool = connect_database_with(&app_config.database);

    // AppRegistory の生成
    let registory = AppRegistry::new(pool);

    // AppRegistory を Router に登録
    let app = Router::new()
        .merge(build_health_check_routers())
        .merge(build_book_routers())
        .with_state(registory);

    // サーバーを起動
    let addr = SocketAddr::new(Ipv4Addr::LOCALHOST.into(), 8080);
    let listener = TcpListener::bind(&addr).await?;

    println!("Listening on {}", addr);

    axum::serve(listener, app).await.map_err(Error::from)
}
