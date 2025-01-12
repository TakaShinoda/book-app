// kernel レイヤーでは Book に対する書き込み（作成、編集）をイベントとして扱う

pub struct CreateBook {
    pub title: String,
    pub author: String,
    pub isbn: String,
    pub description: String,
}
