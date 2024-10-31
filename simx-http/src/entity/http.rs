use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct HttpConfig {
    // 获取监听地址
    pub addr: String,
    // 获取监听端口
    pub port: u16,
    // 获取工作线程数
    pub workers: usize,
    // 获取临时文件夹
    pub temp_dir: String,
    pub cli_colors: bool,
    // 最大线程，按照引擎最大线程的一半
    pub max_blocking: usize,
}

impl Default for HttpConfig {
    fn default() -> Self {
        HttpConfig {
            addr: "0.0.0.0".to_string(),
            port: 9802,
            workers: 3,
            temp_dir: "temp".to_string(),
            cli_colors: true,
            max_blocking: 10
        }
    }
}

pub enum HttpMethod {
    GET,
    POST,
    PUT,
    DELETE,
    HEAD,
    OPTIONS,
    PATCH,
    TRACE,
    CONNECT,
}