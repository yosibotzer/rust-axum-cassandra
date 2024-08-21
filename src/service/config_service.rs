use config::{Config, ConfigError, File};
use tracing::info;

use crate::{model::config::CassandraConfig, RunMode};



impl CassandraConfig {
    
    pub fn new(run_mode: &RunMode) -> Result<Self, ConfigError> {

        let file_name = format!("config/{}", run_mode.to_string().to_lowercase());

        info!("Loading config file: {}", file_name);
        
        let cnf = Config::builder()
            .add_source(File::with_name(&format!("config/default")).required(true))
            .add_source(File::with_name(&file_name).required(true))
            .add_source(config::Environment::with_prefix("APP"))
            .build()?;

        cnf.try_deserialize()

    }
}