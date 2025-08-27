use scylla::client::session::Session;

use super::config::CassandraConfig;


#[derive(Debug)]
pub struct ServiceState {
    pub config: CassandraConfig,
    pub session: Session,
}