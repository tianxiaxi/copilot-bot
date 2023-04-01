use std::{net::SocketAddr, str::FromStr};

use tracing::info;
use tracing_subscriber::fmt::{time, writer::MakeWriterExt};
use copilot_web::routers;
use copilot_config::{version, APPCONF};
use copilot_utils::file;

/// initliaze the logger with tracing/tracing-subscriber/tracing-appender
fn init_log() {
    let path = file::get_execute_path().join("logs");
    println!("The log file path: {:?}", path.display());
    // install global collector configured based on RUST_LOG env var.
    let stdout = std::io::stdout.with_max_level(tracing::Level::INFO);
    let logfile = tracing_appender::rolling::daily(path, "copilot-web.log");
    let (logfile, _) = tracing_appender::non_blocking(logfile);
    let subscriber = tracing_subscriber::fmt()
        .compact()
        .with_writer(stdout.and(logfile))
        .with_file(true)
        .with_line_number(true)
        .with_thread_ids(true)
        .with_target(false)
        .with_timer(time::LocalTime::rfc_3339())
        .finish();
    tracing::subscriber::set_global_default(subscriber).unwrap();
}

#[tokio::main]
async fn main() {
    init_log();

    println!("{:?}", APPCONF.server);

    info!("Current version for copilot-bot: {:?}", version::version());

    let app = routers::get_app_routers();

    // run web server with hyper
    let url = format!("{}:{}", APPCONF.server.host, APPCONF.server.port);
    info!("Start web server at http://{}", url);

    axum::Server::bind(&SocketAddr::from_str(&url).unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
