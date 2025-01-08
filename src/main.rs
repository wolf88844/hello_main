use axum::{routing::get, Router};
use hello_main::Setting;

#[tokio::main]
async fn main() {
    let setting = Setting::new();
    let app = Router::new().route("/",get(hello));
    let port = setting.get_port();
    let addr = format!("0.0.0.0:{}",port);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener,app).await.unwrap();
}

async fn hello()->&'static str{
    "Hello, world!"
}
