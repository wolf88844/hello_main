use std::{
    collections::HashMap,
    net::{IpAddr, Ipv4Addr, SocketAddr},
    sync::Arc,
};

use clap::{Arg, ArgMatches, Command, value_parser};
use opentelemetry::{
    KeyValue, global,
    trace::{TraceError, TracerProvider},
};
use opentelemetry_otlp::{WithExportConfig, WithHttpConfig};
use opentelemetry_sdk::{
    Resource,
    propagation::TraceContextPropagator,
    runtime,
    trace::{self, RandomIdGenerator, Sampler, Tracer},
};
use tower_http::trace::TraceLayer;
use tracing_subscriber::{Layer, layer::SubscriberExt, util::SubscriberInitExt};

use crate::{
    settings::{OtlpTarget, Settings},
    shutdown,
    state::ApplicationState,
};

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
            // 如果设置中存在OTLP目标，则初始化一个追踪器并创建一个Telemetry层
            let telemetry_layer = if let Some(otlp_targer) = settings.logging.otlp_target.clone() {
                let tracer = init_tracer(&otlp_targer)?;
                Some(tracing_opentelemetry::layer().with_tracer(tracer))
            } else {
                None
            };
            // 创建一个标准输出日志层，并设置过滤条件
            let stdout_log = tracing_subscriber::fmt::layer().with_filter(
                tracing_subscriber::EnvFilter::from_default_env()
                    .add_directive("axum=debug".parse()?)
                    .add_directive("tower_http=debug".parse()?)
                    .add_directive("sqlx=debug".parse()?)
                    .add_directive("sqlx::query=debug".parse()?)
            );

            // 创建一个新的追踪订阅器，包含Telemetry层和标准输出日志层
            let subscriber = tracing_subscriber::registry()
                .with(telemetry_layer)
                .with(stdout_log);

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
            let router = crate::api::configure(state).layer(TraceLayer::new_for_http()); // 创建一个新的套接字地址
            let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), port);
            // 绑定到套接字地址并监听
            let listener = tokio::net::TcpListener::bind(addr).await?;
            // 启动HTTP服务器并处理请求
            axum::serve(listener, router.into_make_service())
                // 使用优雅关闭信号处理程序
                .with_graceful_shutdown(shutdown::shutdown_signal())
                .await?;

            anyhow::Ok(())
        })?;

    Ok(())
}

pub fn init_tracer(otlp_target: &OtlpTarget) -> Result<Tracer, TraceError> {
    global::set_text_map_propagator(TraceContextPropagator::new());

    let otlp_endpoint = otlp_target.address.as_str();

    let mut builder = opentelemetry_otlp::SpanExporter::builder()
        .with_http()
        .with_endpoint(otlp_endpoint);

    if let Some(authorization) = &otlp_target.authorization {
        let mut headers = HashMap::new();
        headers.insert(String::from("Authorization"), authorization.clone());
        builder = builder.with_headers(headers);
    };

    let exporter = builder.build()?;

    let tracer_provider = trace::TracerProvider::builder()
        .with_batch_exporter(exporter, runtime::Tokio)
        .with_sampler(Sampler::AlwaysOn)
        .with_id_generator(RandomIdGenerator::default())
        .with_max_events_per_span(64)
        .with_max_attributes_per_event(16)
        .with_max_events_per_span(16)
        .with_resource(Resource::new(vec![KeyValue::new(
            "service.name",
            "sample_application",
        )]))
        .build();

    Ok(tracer_provider.tracer("sample_application"))
}
