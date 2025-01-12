use axum::{extract::State, http::StatusCode};
use registry::AppRegistry;

// ヘルスチェックのハンドラ
pub async fn health_check() -> StatusCode {
    StatusCode::OK
}

// State に登録されている AppRegistry を取り出す
pub async fn health_check_db(State(registry): State<AppRegistry>) -> StatusCode {
    // health_check_repository メソッドを経由してリポジトリの処理を呼び出す
    if registry.health_check_repository().check_db().await {
        StatusCode::OK
    } else {
        StatusCode::INTERNAL_SERVER_ERROR
    }
}
