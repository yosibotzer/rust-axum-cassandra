
use serde::Deserialize;



#[derive(Debug, Deserialize, Clone)]
pub struct CassandraConfig {
    pub known_nodes: Vec<String>,
    pub request_timeout_millis: u64,
} 

