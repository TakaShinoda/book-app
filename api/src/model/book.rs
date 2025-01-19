use kernel::model::book::{event::CreateBook, Book};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// POST /books に送られてきた際の body に入っている JSON を表現する型
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")] // フロントでキャメルケースで扱うため変換
pub struct CreateBookRequest {
    pub title: String,
    pub author: String,
    pub isbn: String,
    pub description: String,
}

//CreateBook への From トレイト実装
impl From<CreateBookRequest> for CreateBook {
    fn from(value: CreateBookRequest) -> Self {
        let CreateBookRequest {
            title,
            author,
            isbn,
            description,
        } = value;

        Self {
            title,
            author,
            isbn,
            description,
        }
    }
}

// 蔵書データ取得の際の応答形式
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BookResponse {
    pub id: Uuid,
    pub title: String,
    pub author: String,
    pub isbn: String,
    pub description: String,
}

impl From<Book> for BookResponse {
    fn from(value: Book) -> Self {
        let Book {
            id,
            title,
            author,
            isbn,
            description,
        } = value;

        Self {
            id,
            title,
            author,
            isbn,
            description,
        }
    }
}
