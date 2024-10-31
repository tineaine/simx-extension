use crate::handler::interface::handler_func;
use crate::service::interface::handler_service;
use engine_share::entity::exception::node::NodeError;
use engine_share::entity::extension::Extension;
use engine_share::entity::flow::flow::FlowData;
use engine_share::entity::flow::node::Node;
use engine_share::entity::services::{Service, ServiceState};

mod handler;
mod service;
pub mod entity;
pub mod common;

#[no_mangle]
#[allow(improper_ctypes_definitions)]
// 测试方法，引擎启用扩展后，会自动调用此方法测试
pub extern "C" fn test() -> bool { true }

#[no_mangle]
#[allow(improper_ctypes_definitions)]
// 服务调用入口
pub async extern "C" fn handle_service(service: Service) -> Result<(), String> {
    let future = async {
        handler_service(service).await;
    };
    tokio::runtime::Runtime::new().unwrap().block_on(future);
    Ok(())
}

#[no_mangle]
#[allow(improper_ctypes_definitions)]
// 函数调用入口（处理器）
pub extern "C" fn handle_func(node: Node, flow_data: &mut FlowData) -> Result<(), NodeError> {
    let mut res: Result<(), NodeError> = Ok(());
    let future = async {
        res = handler_func(node, flow_data).await
    };
    tokio::runtime::Runtime::new().unwrap().block_on(future);
    res
}

#[no_mangle]
#[allow(improper_ctypes_definitions)]
// 初始化调用入口
pub extern "C" fn init() -> bool {
    let future = async {
        handler_service(Service {
            id: "".to_string(),
            name: "http".to_string(),
            version: "".to_string(),
            status: ServiceState {
                enable: true,
                user_count: 0,
            },
            extension: Extension {
                path: None,
                name: "".to_string(),
                version: "".to_string(),
                engine: "".to_string(),
                author: "".to_string(),
                dependencies: vec![],
                entry_lib: "".to_string(),
                init: "".to_string(),
                destroy: "".to_string(),
                handle_func: "".to_string(),
                handle_service: "".to_string(),
            },
            data: serde_json::to_string(&entity::http::HttpConfig::default()).unwrap(),
        }).await;
    };
    tokio::runtime::Runtime::new().unwrap().block_on(future);
    true
}

#[no_mangle]
#[allow(improper_ctypes_definitions)]
// 销毁调用入口
pub extern "C" fn destroy() -> bool {
    true
}
