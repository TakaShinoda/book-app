use std::net::{Ipv4Addr, SocketAddr};

use adapter::database::connect_database_with;
use anyhow::{Error, Ok, Result};
use api::route::{book::build_book_routers, health::build_health_check_routers};
use axum::Router;
use registry::AppRegistry;
use shared::config::AppConfig;
use tokio::net::TcpListener;

use shared::env::{which, Environment};
use tokio::sync::broadcast::error;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::EnvFilter;

use anyhow::Context;
use tower_http::trace::{DefaultMakeSpan, DefaultOnRequest, DefaultOnResponse, TraceLayer};
use tower_http::LatencyUnit;
use tracing::Level;

// tokio ランタイム上で動かすために必要なマクロ、このマクロを使用することで main 関数を非同期化
#[tokio::main]
async fn main() -> Result<()> {
    init_logger()?;
    bootstrap().await
}

// ロガーを初期化する関数
fn init_logger() -> Result<()> {
    let log_level = match which() {
        Environment::Development => "debug",
        Environment::Production => "info",
    };

    // ログレベルを設定
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| log_level.into());

    // ログ出力形式を設定
    let subscriber = tracing_subscriber::fmt::layer()
        .with_file(true)
        .with_line_number(true)
        .with_target(false);

    tracing_subscriber::registry()
        .with(subscriber)
        .with(env_filter)
        .try_init()?;

    Ok(())
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
        .layer(cors())
        // リクエストとレスポンス時にログを出力するレイヤーの追加
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::new().level(Level::INFO))
                .on_request(DefaultOnRequest::new().level(Level::INFO))
                .on_response(
                    DefaultOnResponse::new()
                        .level(Level::INFO)
                        .latency_unit(LatencyUnit::Millis),
                ),
        )
        .with_state(registory);

    // サーバーを起動
    let addr = SocketAddr::new(Ipv4Addr::LOCALHOST.into(), 8080);
    let listener = TcpListener::bind(&addr).await?;

    tracing::info!("Listening on {}", addr);

    axum::serve(listener, app)
        .await
        .context("Unexpected error happened in server")
        .inspect_err(|e| {
            tracing::error!(
                error.cause_chain = ?e,
                error.message = %e,
                "Unexpected error"
            )
        })
}
