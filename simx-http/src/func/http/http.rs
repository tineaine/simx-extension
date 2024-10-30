use engine_share::entity::exception::node::NodeError;
use engine_share::entity::flow::flow::FlowData;
use engine_share::entity::flow::node::Node;

pub fn handler_http(node: Node, flow_data: &mut FlowData) -> Result<(), NodeError> {
    let handler_path: Vec<_> = node.handler.split(".").collect();
    match handler_path[2] {
        "get" => { request_http(node, flow_data) }
        "post" => {Ok(())}
        "put" => {Ok(())}
        "delete" => {Ok(())}
        "request" => {Ok(())}
        _ => {Ok(())}
    }
}

fn request_http(node: Node, flow_data: &mut FlowData) -> Result<(), NodeError> {
    println!("{:?}", node);
    println!("{:?}", flow_data);
    Ok(())
}
