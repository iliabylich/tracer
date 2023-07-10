use axum::{routing::get, Router};
use clap::Parser;
use std::net::SocketAddr;
use tower_http::trace::{self, TraceLayer};
use tracing::Level;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    port: u16,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    tracing_subscriber::fmt()
        .with_target(false)
        .compact()
        .init();

    let app = Router::new().route("/", get(root)).layer(
        TraceLayer::new_for_http()
            .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
            .on_response(trace::DefaultOnResponse::new().level(Level::INFO)),
    );

    let addr = SocketAddr::from(([127, 0, 0, 1], args.port));
    tracing::info!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn root() -> &'static str {
    "Tracer's root"
}
