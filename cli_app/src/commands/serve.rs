use std::{
    net::{IpAddr, Ipv4Addr, SocketAddr},
    sync::Arc,
};

use anyhow::Ok;
use clap::{Arg, ArgMatches, Command, value_parser};
use tower_http::trace::TraceLayer;
use tracing::{Level, level_filters::LevelFilter};
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt};

use crate::{settings::Settings, shutdown, state::ApplicationState};

pub const COMMAND_NAME: &str = "serve";

pub fn configure() -> Command {
    Command::new(COMMAND_NAME)
        .about("Start the HTTP server")
        .arg(
            Arg::new("port")
                .short('p')
                .long("port")
                .value_name("PORT")
                .help("TCP port to listen on")
                .default_value("8080")
                .value_parser(value_parser!(u16)),
        )
}

pub fn handle(matches: &ArgMatches, settings: &Settings) -> anyhow::Result<()> {
    let port = *matches.get_one("port").unwrap_or(&8080);
    println!("Start the HTTP server on port {}", port);

    start_tokio(port, settings)?;

    Ok(())
}

/// 启动一个Tokio运行时并运行HTTP服务器
///
/// # 参数
///
/// * `port` - 服务器监听的端口号
/// * `settings` - 应用程序的配置设置
///
/// # 返回值
///
/// 如果成功，返回`Ok(())`；如果失败，返回`Err`，其中包含错误信息
fn start_tokio(port: u16, settings: &Settings) -> anyhow::Result<()> {
    // 创建一个新的Tokio运行时构建器
    tokio::runtime::Builder::new_multi_thread()
        // 启用所有Tokio特性
        .enable_all()
        // 构建Tokio运行时
        .build()?
        // 在Tokio运行时上运行异步任务
        .block_on(async move {
            // 创建一个新的追踪订阅器
            let subscriber = tracing_subscriber::registry()
                // 设置日志级别为TRACE
                .with(LevelFilter::from_level(Level::TRACE))
                // 添加默认的格式化层
                .with(fmt::Layer::default());

            // 初始化追踪订阅器
            subscriber.init();

            //数据库连接
            let db_url = settings
                .database
                .url
                .clone()
                .expect("Database URL is not set");

            let pool = sqlx::postgres::PgPoolOptions::new()
                .max_connections(5)
                .connect(&db_url)
                .await
                .expect("Failed to create database connection pool");

            // 创建一个新的应用程序状态
            let state = Arc::new(ApplicationState::new(settings, pool)?);
            // 配置应用程序的路由
            let router = crate::api::configure(state).layer(TraceLayer::new_for_http());
            // 创建一个新的套接字地址
            let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), port);
            // 绑定到套接字地址并监听
            let listener = tokio::net::TcpListener::bind(addr).await?;
            // 启动HTTP服务器并处理请求
            axum::serve(listener, router.into_make_service())
                // 使用优雅关闭信号处理程序
                .with_graceful_shutdown(shutdown::shutdown_signal())
                .await?;

            Ok(())
        })?;

    Ok(())
}
