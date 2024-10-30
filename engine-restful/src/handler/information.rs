use salvo::handler;

// 测试方法，可以调用这个端口查看服务是否启动
#[handler]
pub async fn hello() -> &'static str {
    "Hello World"
}

// 获取当前版本信息

