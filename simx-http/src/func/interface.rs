use engine_common::entity::flow::flow::FlowData;
use engine_common::entity::flow::node::Node;


pub fn handler_func(node: Node, flow_data: &mut FlowData) {
    match node.handler.as_str() {
        "http" => { }
        "socket" => { }
        _ => {}
    };
}