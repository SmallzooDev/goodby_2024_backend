use tracing_subscriber::{
    fmt,
    EnvFilter,
    Layer,
    layer::SubscriberExt,
    util::SubscriberInitExt,
};
use tracing_appender::rolling::{RollingFileAppender, Rotation};
use std::error::Error;

pub fn setup_logging() -> Result<(), Box<dyn Error>> {
    let file_appender = RollingFileAppender::new(
        Rotation::DAILY,
        "logs",
        "ticket-draw.log",
    );

    // 파일 - 티켓과 추첨 관련 로그만 저장
    let file_layer = fmt::layer()
        .json()
        .with_file(true)
        .with_line_number(true)
        .with_writer(file_appender)
        .with_filter(EnvFilter::new("user_ticket=info,prize=info"));

    // 콘솔 - 모든 로그 출력
    let console_layer = fmt::layer()
        .compact()
        .with_filter(
            EnvFilter::new("server=info")
                .add_directive("auth=info".parse().unwrap())
                .add_directive("user_ticket=info".parse().unwrap())
                .add_directive("prize=info".parse().unwrap())
        );

    tracing_subscriber::registry()
        .with(console_layer)
        .with(file_layer)
        .try_init()?;

    Ok(())
} 