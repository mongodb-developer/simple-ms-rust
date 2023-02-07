use axum::http::StatusCode;

pub async fn get() -> StatusCode {
    StatusCode::OK
}
