use axum::{
    middleware as axum_middleware,
    response::Response,
};
use hyper::Request;
use tracing::debug;

pub async fn logging_middleware(req: Request<axum::body::Body>, next: axum_middleware::Next) -> Response {
    debug!("Received request: {} {}", req.method(), req.uri());
    let response = next.run(req).await;
    debug!("Responded with: {}", response.status());
    response
}