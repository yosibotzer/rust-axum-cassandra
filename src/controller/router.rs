

use std::sync::Arc;

use axum::{
    extract::{Path, State}, 
    http::StatusCode, 
    response::{IntoResponse, Response}, 
    routing::{get, post}, 
    Json, 
    Router
};

use tower_http::compression::CompressionLayer;

use crate::model::service_state::ServiceState;
use crate::service::cassandra_service;
use crate::model::api::{TestBoolRequest, TestMapRequest, TestSetRequest};
use crate::service::cassandra_service::InternalError;



pub fn get_service_routes(service_state : ServiceState) -> Router {

    let state = Arc::new(service_state);

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

async fn fetch(State(service_state): State<Arc<ServiceState>>, Path(test_id): Path<String>) 
-> Result<Response, InternalError> {

    let test_row_option = cassandra_service::fetch(service_state, test_id).await?;

    match test_row_option {
        Some(test) => Ok(Json(test).into_response()),
        None => Ok(StatusCode::NO_CONTENT.into_response())
    }
}

async fn set_test_bool(State(service_state): State<Arc<ServiceState>>, Json(request): Json<TestBoolRequest>) 
-> Result<StatusCode, InternalError> {
    
    cassandra_service::set_test_bool(service_state, request).await?;

    Ok(StatusCode::OK)
}

async fn set_test_map(State(service_state): State<Arc<ServiceState>>, Json(request): Json<TestMapRequest>) 
-> Result<StatusCode, InternalError> {
    
    cassandra_service::set_test_map(service_state, request).await?;

    Ok(StatusCode::OK)
}

async fn set_test_set(State(service_state): State<Arc<ServiceState>>, Json(request): Json<TestSetRequest>) 
-> Result<StatusCode, InternalError> {
    
    cassandra_service::set_test_set(service_state, request).await?;

    Ok(StatusCode::OK)
}