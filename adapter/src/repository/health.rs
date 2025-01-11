use async_trait::async_trait;
use derive_new::new;
use kernel::repository::health::HealthCheckRepository;

use crate::database::ConnectionPool;

// コンストラクタ自動生成
#[derive(new)]
pub struct HealthCheckRepositoryImpl {
    db: ConnectionPool,
}

#[async_trait]
impl HealthCheckRepository for HealthCheckRepositoryImpl {
    async fn check_db(&self) -> bool {
        // クエリ実行結果は Result 型なので、OK なら true を Err なら false を返す
        sqlx::query("SELECT 1")
            .fetch_one(self.db.inner_ref())
            .await
            .is_ok()
    }
}
