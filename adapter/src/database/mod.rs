use shared::config::DatabaseConfig;
use sqlx::{postgres::PgConnectOptions, PgPool};

pub mod model;

// DatabaseConfig から PgConnectOptions に変換する関数
fn make_pg_connect_options(cfg: &DatabaseConfig) -> PgConnectOptions {
    PgConnectOptions::new()
        .host(&cfg.host)
        .port(cfg.port)
        .username(&cfg.username)
        .password(&cfg.password)
        .database(&cfg.database)
}

// sqlx::PgPool をラップする
#[derive(Clone)]
pub struct ConnectionPool(PgPool);

impl ConnectionPool {
    // sqlx::PgPool への参照を取得する
    pub fn inner_ref(&self) -> &PgPool {
        &self.0
    }
}

// Postgres 専用のコネクションプールを作成
// 返り値を ConnectionPool に変換し、内部実装もそれに合わせて修正
pub fn connect_database_with(cfg: &DatabaseConfig) -> ConnectionPool {
    ConnectionPool(PgPool::connect_lazy_with(make_pg_connect_options(cfg)))
}
