use std::collections::HashMap;

use serde::Deserialize;


#[derive(Debug, Deserialize)]
pub struct TestBoolRequest {
    pub test_id: String,
    pub ttl: i32,
    pub test_bool: bool,
}

#[derive(Debug, Deserialize)]
pub struct TestMapRequest {
    pub test_id: String,
    pub ttl: i32,
    pub test_map: HashMap<String, i32>,
}


#[derive(Debug, Deserialize)]
pub struct TestSetRequest {
    pub test_id: String,
    pub ttl: i32,
    pub test_set: Vec<i32>,
}
