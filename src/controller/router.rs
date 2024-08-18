

use std::sync::Arc;

use axum::{extract::{Path, State}, http::StatusCode, response::{IntoResponse, Response}, routing::{get, post}, Json, Router};
use scylla::Session;
use tower_http::compression::CompressionLayer;

use crate::{model::api::{TestBoolRequest, TestMapRequest, TestSetRequest}, service::cassandra_service::{self, InternalError}};


pub fn get_service_routes(cassandra_session : Session) -> Router {

    let state = Arc::new(cassandra_session);

    Router::new()
        .route("/status", get(status))
        .route("/test/:test_id", get(fetch))
        .route("/test/bool", post(set_test_bool))
        .route("/test/map", post(set_test_map))
        .route("/test/set", post(set_test_set))
        .with_state(state)
        .layer(CompressionLayer::new())
}

impl IntoResponse for InternalError {
    
    fn into_response(self) -> Response {
        StatusCode::INTERNAL_SERVER_ERROR.into_response()
    }
}
async fn status() -> StatusCode {
    StatusCode::OK
}

async fn fetch(State(session): State<Arc<Session>>, Path(test_id): Path<String>) 
-> Result<Response, InternalError> {

    let test_row_option = cassandra_service::fetch(session, test_id).await?;

    match test_row_option {
        Some(test) => Ok(Json(test).into_response()),
        None => Ok(StatusCode::NO_CONTENT.into_response())
    }
}

async fn set_test_bool(State(session): State<Arc<Session>>, Json(request): Json<TestBoolRequest>) 
-> Result<StatusCode, InternalError> {
    
    cassandra_service::set_test_bool(session, request).await?;

    Ok(StatusCode::OK)
}

async fn set_test_map(State(session): State<Arc<Session>>, Json(request): Json<TestMapRequest>) 
-> Result<StatusCode, InternalError> {
    
    cassandra_service::set_test_map(session, request).await?;

    Ok(StatusCode::OK)
}

async fn set_test_set(State(session): State<Arc<Session>>, Json(request): Json<TestSetRequest>) 
-> Result<StatusCode, InternalError> {
    
    cassandra_service::set_test_set(session, request).await?;

    Ok(StatusCode::OK)
}