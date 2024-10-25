use engine_share::entity::services::Service;
use crate::service::http::interface::handle_service_http;

pub fn handler_service(service: Service) {
    match service.name.as_str() {
        "http" | "https" => handle_service_http(service),
        "socket" | "tcp" => {}
        "ws" | "websocket" => {}
        _ => {}
    }
}