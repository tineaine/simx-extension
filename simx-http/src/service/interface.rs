use engine_share::entity::services::Service;
use salvo::Router;
use salvo::prelude::*;

pub async fn handler_service(service: Service) {
    start_service(service).await;
}

// 根据指定的配置，开启服务监听
pub async fn start_service(service: Service) {
    tracing_subscriber::fmt().init();

    let router = Router::new().get(hello);
    let acceptor = TcpListener::new("127.0.0.1:9998").bind().await;
    Server::new(acceptor).serve(router).await;
}

pub async fn stop_service(service: Service) {

}

#[handler]
async fn hello() -> &'static str {
    "Hello World"
}