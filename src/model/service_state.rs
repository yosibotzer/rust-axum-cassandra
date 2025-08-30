use scylla::client::session::Session;

use super::config::CassandraConfig;



pub struct ServiceState {
    pub config: CassandraConfig,
    pub session: Session,
}