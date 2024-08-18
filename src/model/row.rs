use std::collections::HashMap;

use scylla::FromRow;
use serde::Serialize;



#[derive(Debug, FromRow, Serialize)]
pub struct TestRow {
    pub test_bool: Option<bool>,
    pub test_set: Option<Vec<i32>>,
    pub test_map: Option<HashMap<String, i32>>,
}
