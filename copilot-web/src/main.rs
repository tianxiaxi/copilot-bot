use tracing::{info};
use tracing_subscriber::fmt::{time, writer::MakeWriterExt};
use copilot_web::routers;
use copilot_config::version;

/// initliaze the logger with tracing/tracing-subscriber/tracing-appender
fn init_log() {
    let path = std::env::current_exe().unwrap();
    let log_path = path.parent().expect("Except a directory").join("logs");
    println!("{:?}", log_path);
    // install global collector configured based on RUST_LOG env var.
    let stdout = std::io::stdout.with_max_level(tracing::Level::INFO);
    let logfile = tracing_appender::rolling::daily(log_path, "copilot-web.log");
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

    info!("Current version for copilot-bot: {:?}", version::version());
    info!("Start web server at http://0.0.0.0:3000");

    let app = routers::get_app_routers();

    // run web server with hyper
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
