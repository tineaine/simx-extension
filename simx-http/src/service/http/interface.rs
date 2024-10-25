use engine_share::entity::services::Service;
use crate::service::http::http::interface::start_net_watcher;


pub fn handle_service_http(service: Service) {
    serve(service)
}

pub fn serve(service: Service) {
    let future = async {
        start_net_watcher(service).await;
    };
    println!("Simx serve has started.");
    tokio::runtime::Runtime::new().unwrap().block_on(future);
}