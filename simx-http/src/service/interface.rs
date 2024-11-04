use crate::entity::http::HttpConfig;
use engine_share::entity::services::Service;
use salvo::prelude::*;
use salvo::Router;

pub async fn handler_service(service: Service) {
    start_service(service).await;
}

// 根据指定的配置，开启服务监听
pub async fn start_service(service: Service) {
    let conf: HttpConfig = serde_json::from_value(service.data).expect("Unrecognized service configuration.");
    let address = format!("{}:{}", conf.addr, conf.port);
    tracing_subscriber::fmt().init();

    let router = Router::new().get(hello);
    let acceptor = TcpListener::new(address).bind().await;
    Server::new(acceptor).serve(router).await;
}

pub async fn stop_service(service: Service) {}

#[handler]
async fn hello() -> &'static str {
    "Hello World"
}