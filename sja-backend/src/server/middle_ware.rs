use axum::{body::Body, extract::Request, middleware::Next, response::Response};
use chrono::{Local, SubsecRound};

pub async fn logging_middleware(req: Request<Body>, next: Next) -> Response {
    let call_time = Local::now().round_subsecs(0);
    let req_uri = req.uri().clone();
    log::info!("{}: Received a request to {}", &call_time, req_uri);

    let response = next.run(req).await;
    log::info!(
        "{}: Response to {}: {}",
        &call_time,
        req_uri,
        response.status()
    );

    response
}
