use kernel::model::book::Book;
use uuid::Uuid;

//データベースかのレコードを読み取る時の型
// 各フィールドはカラム名と同じにしておくと取り扱いが楽
pub struct BookRow {
    pub book_id: Uuid,
    pub title: String,
    pub author: String,
    pub isbn: String,
    pub description: String,
}

// BookRow はあくまで内部で利用する構造体
// 戻り値にするときは kernel で定義した Book 構造体の形式に変換する（From トレイトを使う）
impl From<BookRow> for Book {
    fn from(value: BookRow) -> Self {
        // パターンマッチを用いて、BookRow の中身を取り出す
        let BookRow {
            book_id,
            title,
            author,
            isbn,
            description,
        } = value;
        Self {
            id: book_id,
            title,
            author,
            isbn,
            description,
        }
    }
}
