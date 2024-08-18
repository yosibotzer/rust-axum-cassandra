use std::sync::Arc;

use scylla::{serialize::row::SerializeRow, QueryResult, Session};
use tracing::error;

use crate::model::{api::{TestBoolRequest, TestMapRequest, TestSetRequest}, row::TestRow};


const FETCH_TEST_CQL: &str = "select test_bool, test_set, test_map from rust.test where test_id = ?";
const SET_TEST_BOOL_CQL: &str = "UPDATE rust.test using ttl ? SET test_bool = ? WHERE test_id = ?";
const SET_TEST_MAP_CQL: &str = "UPDATE rust.test using ttl ? SET test_map = test_map + ? WHERE test_id = ?";
const SET_TEST_SET_CQL: &str = "UPDATE rust.test using ttl ? SET test_set = test_set + ? WHERE test_id = ?";

pub struct InternalError;

async fn execute(session: &Session, cql : &str, values: impl SerializeRow) -> Result<QueryResult, InternalError> {

    let prepared = session
        .prepare(cql)
        .await
        .map_err(|e| map_error(Box::new(e)))?;
    
    let query_result = session
        .execute(&prepared, values)
        .await
        .map_err(|e| map_error(Box::new(e)))?;

    Ok(query_result)
}

fn map_error(cassandra_error: Box<dyn std::error::Error>) -> InternalError {
    error!("Error: {:?}", cassandra_error);
    InternalError
}

pub async fn fetch(session: Arc<Session>, test_id: String) -> Result<Option<TestRow>, InternalError> {

    let cql_values = (test_id, );

    let result = execute(&session, FETCH_TEST_CQL, cql_values).await?;

    let test_option = result
        .maybe_first_row_typed::<TestRow>()
        .map_err(|e| map_error(Box::new(e)))?;

    Ok(test_option)
}

pub async fn set_test_bool(session: Arc<Session>, request: TestBoolRequest) -> Result<(), InternalError> {
    
    let cql_values = (request.ttl, request.test_bool, request.test_id);

    execute(&session, SET_TEST_BOOL_CQL, cql_values).await?;

    Ok(())
}

pub async fn set_test_map(session: Arc<Session>, request: TestMapRequest) -> Result<(), InternalError> {
    
    let cql_values = (request.ttl, request.test_map, request.test_id);

    execute(&session, SET_TEST_MAP_CQL, cql_values).await?;

    Ok(())
}

pub async fn set_test_set(session: Arc<Session>, request: TestSetRequest) -> Result<(), InternalError> {
    
    let cql_values = (request.ttl, request.test_set, request.test_id);

    execute(&session, SET_TEST_SET_CQL, cql_values).await?;

    Ok(())
}