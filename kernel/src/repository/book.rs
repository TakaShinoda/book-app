use crate::model::{
    book::{event::CreateBook, Book},
    id::BookId,
};
use async_trait::async_trait;
use shared::error::AppResult;

#[async_trait]
pub trait BookRepository: Send + Sync {
    // 蔵書のレコード追加
    async fn create(&self, event: CreateBook) -> AppResult<()>;

    // 蔵書の一覧を取得
    async fn find_all(&self) -> AppResult<Vec<Book>>;

    // 蔵書 ID を指定してデータを取得
    async fn find_by_id(&self, book_id: BookId) -> AppResult<Option<Book>>;
}
