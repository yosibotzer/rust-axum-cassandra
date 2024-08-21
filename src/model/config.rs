
use serde::Deserialize;



#[derive(Debug, Deserialize, Clone)]
pub struct CassandraConfig {
    pub known_nodes: String,
    pub request_timeout_millis: u64,
} 

