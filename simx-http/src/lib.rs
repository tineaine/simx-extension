use engine_share::entity::exception::node::NodeError;
use engine_share::entity::extension::Extension;
use engine_share::entity::flow::flow::FlowData;
use engine_share::entity::flow::node::Node;
use engine_share::entity::services::{Service, ServiceState};
use crate::func::interface::handler_func;
use crate::service::interface::handler_service;

mod func;
mod service;
pub mod entity;

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
    println!("serve has started.");
    tokio::runtime::Runtime::new().unwrap().block_on(future);
    Ok(())
}


#[no_mangle]
#[allow(improper_ctypes_definitions)]
// 函数调用入口（处理器）
pub extern "C" fn handle_func(node: Node, flow_data: &mut FlowData) -> Result<(), NodeError> {
    handler_func(node, flow_data)
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
            data: "{\"port\": 8080, \"workers\": 4, \"max_blocking\": 10, \"cli_colors\": true}".to_string(),
        }).await;
    };
    println!("serve has started.");
    tokio::runtime::Runtime::new().unwrap().block_on(future);
    true
}

#[no_mangle]
#[allow(improper_ctypes_definitions)]
// 销毁调用入口
pub extern "C" fn destroy() -> bool {
    true
}
