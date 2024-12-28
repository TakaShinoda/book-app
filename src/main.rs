use std::net::{Ipv4Addr, SocketAddr};

use axum::{routing::get, Router};
use tokio::net::TcpListener;

// ハンドラ
async fn hello_world() -> &'static str {
    "Hello, world"
}

// tokio ランタイム上で動かすために必要なマクロ、このマクロを使用することで main 関数を非同期化
#[tokio::main]
async fn main() {
    // ルーターの設定
    // GET: /hello に対して、hello_world 関数を実行する
    let app = Router::new().route("/hello", get(hello_world));

    // ローカルホスト 8080 番ポートでリクエストを待ち受ける
    let addr = SocketAddr::new(Ipv4Addr::LOCALHOST.into(), 8080);

    // 上記で設定したアドレスでバインドしたリスナーを立ち上げる
    let listener = TcpListener::bind(addr).await.unwrap();

    println!("Listening on {}", addr);

    // 起動する際に、ルーターを axum サーバーに登録する
    axum::serve(listener, app).await.unwrap();
}
