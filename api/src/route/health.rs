use axum::{routing::get, Router};
use registry::AppRegistry;

use crate::handler::health::{health_check, health_check_db};

// ヘルスチェックに関連するルーターをまとめる
// Router の State が AppRegistry となるため、Router の型引数に指定
pub fn build_health_check_routers() -> Router<AppRegistry> {
    let routers = Router::new()
        .route("/", get(health_check))
        .route("/db", get(health_check_db));
    Router::new().nest("/health", routers)
}
