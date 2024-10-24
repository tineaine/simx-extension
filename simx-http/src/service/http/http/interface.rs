use engine_common::entity::services::Service;
use rocket::config::LogLevel;
use rocket::{build, routes, Config};
use crate::entity::http::HttpConfig;
use crate::service::http::http::handler::common::welcome_info;
use crate::service::http::http::handler::script::handle_search_script;
use crate::service::http::http::handler::status::{handle_version_current, handle_version_latest, handle_version_list};

pub async fn start_net_watcher(service: Service) {
    let http_config: HttpConfig = serde_json::from_str(service.data.as_str()).unwrap();

    // http 配置
    let config = Config {
        profile: Default::default(),
        // 不打印Rocket的日志
        log_level: LogLevel::Off,
        // 绑定的端口
        port: http_config.port,
        // 工作线程数
        workers: http_config.workers,
        // 最大线程
        max_blocking: http_config.max_blocking,
        ident: Default::default(),
        ip_header: None,
        // 监听地址
        address: http_config.addr.parse().unwrap(),
        // 缓存目录
        temp_dir: http_config.temp_dir.into(),
        keep_alive: 0,
        shutdown: Default::default(),
        // 是否显示控制台颜色
        cli_colors: http_config.cli_colors,
        limits: Default::default(),
        __non_exhaustive: (),
    };
    // 挂载到simx
    // simx中包含所有操作相关内容
    // 此处阻塞了系统的运行，如果后续需要修改，可以去掉 await
    build().configure(config.clone()).mount("/simx", routes![
        // 系统基础信息
        welcome_info,
        handle_version_current,
        handle_version_list,
        handle_version_latest,
        // 脚本相关
        // handle_list_script,
        handle_search_script,
        // 流程相关
        // handle_list_flow,
        // handle_search_flow,
        // handle_exec_flow_by_path
    ]).launch().await.expect("Cannot load rest services.");
}