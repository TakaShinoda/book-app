use super::id::BookId;

pub mod event;

// 読み込みに使用する型
#[derive(Debug)]
pub struct Book {
    pub id: BookId,
    pub title: String,
    pub author: String,
    pub isbn: String,
    pub description: String,
}
