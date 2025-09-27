mod config;
mod response;
mod handlers;
mod models;
mod middleware;

use crate::config::Config;
use anyhow::{Context, Result};
use axum::{
    extract::{FromRef, FromRequestParts, State},
    http::{request::Parts, StatusCode},
    middleware as axum_middleware,
    response::{Html, IntoResponse, Json, Response},
    routing::{get, post, put, delete},
    Router,
};
use axum_server::{Handle, tls_rustls::RustlsConfig};
use serde::{Deserialize, Serialize};
use std::net::{Ipv4Addr, SocketAddr};
use std::time::Duration;
use sqlx::SqlitePool;
use tokio::signal;
use tracing::{debug, info, warn, error, instrument};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use tower_http::{
    cors::CorsLayer,
    services::ServeDir,
};

#[derive(Clone)]
struct AppState {
    db: SqlitePool,
}

#[tokio::main]
async fn main() -> Result<()> {
    println!("Starting server...");

    // RustlsのCryptoProviderを初期化
    rustls::crypto::aws_lc_rs::default_provider()
        .install_default()
        .expect("Failed to install default crypto provider");

    // 設定を.envから読み込む
    let config = Config::from_env().context("Failed to load configuration")?;

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::new(&config.rust_log),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // バインドするアドレスとポートを指定
    let listener = SocketAddr::from((Ipv4Addr::LOCALHOST, config.port.parse()?));

    // 認証鍵の読み込み
    let tls_config = RustlsConfig::from_pem_file(
        "cert.pem",
        "key.pem"
    )
        .await
        .expect("TLS config error");
    debug!("Loaded TLS config");

    // データベースの接続
    let pool = SqlitePool::connect(&config.database_url).await?;
    debug!("Connected to database");

    let app_state = AppState { db : pool };

    // ルーティングなどの設定
    let app = Router::new()
        .layer(CorsLayer::permissive())
        .layer(axum_middleware::from_fn(middleware::logging::logging_middleware))
        .nest_service("/static", ServeDir::new("./static"))
        .route(
            "/",
            get(response::root),
        )
        .route(
            "/post",
            get(handlers::posts::get_posts).post(handlers::posts::create_post)
        )
        .with_state(app_state);

    let handle = Handle::new();

    // Ctrl+Cシグナル監視タスクをspawnする
    let shutdown_handler = handle.clone();
    tokio::spawn(async move {
        let ctrl_c = async {
            signal::ctrl_c()
                .await
                .expect("failed to install Ctrl+C handler");
        };

        #[cfg(unix)]
        let terminate = async {
            signal::unix::signal(signal::unix::SignalKind::terminate())
                .expect("failed to install signal handler")
                .recv()
                .await;
        };

        #[cfg(not(unix))]
        let terminate = std::future::pending::<()>();

        tokio::select! {
            _ = ctrl_c => {},
            _ = terminate => {},
        }

        info!("Received shutdown signal, exiting...");
        shutdown_handler.graceful_shutdown(Some(Duration::from_secs(1)));
    });

    // info!("Listening on https://{}", listener.to_string());
    info!("Listening on http://{}", listener.to_string());

    // サーバー起動(HTTPS)
    // axum_server::bind_rustls(listener, tls_config)
    //     .handle(handle)
    //     .serve(app.into_make_service())
    //     .await?;

    // サーバー起動(HTTP)
    axum_server::bind(listener)
        .handle(handle)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}