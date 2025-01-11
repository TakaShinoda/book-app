use async_trait::async_trait;

#[async_trait]
// Send と Sync は HealthCheckRepository のスーパートレイト
pub trait HealthCheckRepository: Send + Sync {
    async fn check_db(&self) -> bool;
}
