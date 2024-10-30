use salvo::handler;

// 执行指定蓝图
#[handler]
pub async fn handle_exec_blueprint() -> &'static str {
    println!("Exec blueprint");

    "ok"
}

// #[handler]
// pub async fn hello() -> &'static str {
//     "Hello World"
// }
