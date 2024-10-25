use engine_share::entity::flow::flow::FlowData;
use engine_share::entity::flow::node::Node;


pub fn handler_func(node: Node, flow_data: &mut FlowData) {
    match node.handler.as_str() {
        "http" => { println!("ooooooook") }
        "socket" => { }
        _ => {}
    };
}