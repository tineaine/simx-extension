use salvo::prelude::TcpListener;
use salvo::{Listener, Router, Server};
use crate::handler::flow::handle_exec_blueprint;
use crate::handler::information::hello;

mod handler;

#[no_mangle]
#[allow(improper_ctypes_definitions)]
// 测试方法，引擎启用扩展后，会自动调用此方法测试
pub extern "C" fn test() -> bool { true }


#[no_mangle]
#[allow(improper_ctypes_definitions)]
// 初始化调用入口
pub extern "C" fn init() -> bool {
    let future = async {
        tracing_subscriber::fmt().init();
        let router = Router::new()
            .get(hello)
            .post(handle_exec_blueprint);
        let acceptor = TcpListener::new("127.0.0.1:9802").bind().await;
        Server::new(acceptor).serve(router).await;
    };
    println!("Simx engine restful has started in 9802");
    tokio::runtime::Runtime::new().unwrap().block_on(future);
    true
}

#[no_mangle]
#[allow(improper_ctypes_definitions)]
// 销毁调用入口
pub extern "C" fn destroy() -> bool {
    true
}
