use engine_share::entity::exception::node::NodeError;
use engine_share::entity::flow::flow::FlowData;
use engine_share::entity::flow::node::Node;
use engine_share::entity::services::Service;
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
pub extern "C" fn handle_service(service: Service) -> Result<(), String> {
    handler_service(service);
    Ok(())
}


#[no_mangle]
#[allow(improper_ctypes_definitions)]
// 函数调用入口（处理器）
pub extern "C" fn handle_func(node: Node, flow_data: &mut FlowData) -> Result<(), NodeError> {
    println!("hello world -> {}", node.handler);
    handler_func(node, flow_data);
    Ok(())
}

#[no_mangle]
#[allow(improper_ctypes_definitions)]
// 初始化调用入口
pub extern "C" fn init() -> bool {
    println!("fuck all");
    true
}

#[no_mangle]
#[allow(improper_ctypes_definitions)]
// 销毁调用入口
pub extern "C" fn destroy() -> bool {
    true
}
