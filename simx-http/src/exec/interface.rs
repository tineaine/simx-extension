use crate::exec::http::http::handler_http;
use engine_share::entity::exception::node::NodeError;
use engine_share::entity::flow::flow::FlowData;
use engine_share::entity::flow::node::Node;

pub async fn handler_func(node: Node, flow_data: &mut FlowData) -> Result<(), NodeError> {
    let handler_path: Vec<_> = node.handler.split(".").collect();
    match handler_path[1] {
        "http" => {
            handler_http(node, flow_data).await
        }
        _ => {
            Err(NodeError::HandleNotFound(node.handler))
        }
    }
}