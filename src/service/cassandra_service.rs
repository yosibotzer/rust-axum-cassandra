use std::sync::Arc;

use scylla::{
    client::session::Session,
    response::query_result::QueryResult,
    serialize::row::SerializeRow,
};
use tracing::error;

use crate::model::{api::{TestBoolRequest, TestMapRequest, TestSetRequest}, row::TestRow, service_state::ServiceState};
use crate::service::error::InternalError;

const FETCH_TEST_CQL: &str = "select test_bool, test_set, test_map from rust.test where test_id = ?";
const SET_TEST_BOOL_CQL: &str = "UPDATE rust.test using ttl ? SET test_bool = ? WHERE test_id = ?";
const SET_TEST_MAP_CQL: &str = "UPDATE rust.test using ttl ? SET test_map = test_map + ? WHERE test_id = ?";
const SET_TEST_SET_CQL: &str = "UPDATE rust.test using ttl ? SET test_set = test_set + ? WHERE test_id = ?";



async fn execute(session: &Session, cql : &str, values: impl SerializeRow) -> Result<QueryResult, InternalError> {

    let prepared = session
        .prepare(cql)
        .await?;
    
    let query_result = session
        .execute_unpaged(&prepared, values)
        .await?;

    Ok(query_result)
}



pub async fn fetch(service_state: Arc<ServiceState>, test_id: String) -> Result<Option<TestRow>, InternalError> {

    let cql_values = (test_id, );

    let result = execute(&service_state.session, FETCH_TEST_CQL, cql_values).await?;

    let test_option = result
        .into_rows_result()?
        .maybe_first_row()?;

    Ok(test_option)
}

pub async fn set_test_bool(service_state: Arc<ServiceState>, request: TestBoolRequest) -> Result<(), InternalError> {
    
    let cql_values = (request.ttl, request.test_bool, request.test_id);

    execute(&service_state.session, SET_TEST_BOOL_CQL, cql_values).await?;

    Ok(())
}

pub async fn set_test_map(service_state: Arc<ServiceState>, request: TestMapRequest) -> Result<(), InternalError> {
    
    let cql_values = (request.ttl, request.test_map, request.test_id);

    execute(&service_state.session, SET_TEST_MAP_CQL, cql_values).await?;

    Ok(())
}

pub async fn set_test_set(service_state: Arc<ServiceState>, request: TestSetRequest) -> Result<(), InternalError> {
    
    let cql_values = (request.ttl, request.test_set, request.test_id);

    execute(&service_state.session, SET_TEST_SET_CQL, cql_values).await?;

    Ok(())
}