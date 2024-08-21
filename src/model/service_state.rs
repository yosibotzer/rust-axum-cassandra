use scylla::Session;

use super::config::CassandraConfig;


#[derive(Debug)]
pub struct ServiceState {
    pub config: CassandraConfig,
    pub session: Session,
}