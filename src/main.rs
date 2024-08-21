mod model;
mod service;
mod controller;

use std::env;

use axum::Router;

use controller::router::get_service_routes;
use model::{config::CassandraConfig, service_state};
use scylla::{statement::Consistency, ExecutionProfile, Session, SessionBuilder};
use strum_macros::EnumString;
use tokio::net::TcpListener;
use tracing::{info, level_filters::LevelFilter};
use tracing_subscriber::{layer::SubscriberExt, EnvFilter, Registry};


#[derive(Debug, EnumString, strum_macros::Display)]
enum RunMode {
    Dev,
    Prod,
    Test,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let run_mode_str = env::var("RUN_MODE").unwrap_or_else(|_| RunMode::Dev.to_string());

    let run_mode: RunMode = run_mode_str.parse()?;

    set_tracing(&run_mode)?;

    let cassandra_config = CassandraConfig::new(&run_mode)?;

    let cassandra_session = config_cassandra_session(&cassandra_config).await?;

    let service_state = service_state::ServiceState {
        config: cassandra_config,
        session: cassandra_session,
    };

    let listener = TcpListener::bind("0.0.0.0:3000").await?;

    let app = Router::new()
        .merge(get_service_routes(service_state));
    
    info!("starting server");

    axum::serve(listener, app).await?;

    Ok(())
}

async fn config_cassandra_session(service_config: &CassandraConfig) -> Result<Session, Box<dyn std::error::Error>> {

    let execution_profile = ExecutionProfile::builder()
        .consistency(Consistency::LocalOne)
        .request_timeout(Some(std::time::Duration::from_millis(service_config.request_timeout_millis)))
        .build();
    
    let scylla_session: Session = SessionBuilder::new()
        .known_nodes(service_config.known_nodes.split(','))
        .default_execution_profile_handle(execution_profile.into_handle())
        .build()
        .await?;
    
    Ok(scylla_session)
}

fn set_tracing(run_mode: &RunMode) -> Result<(), Box<dyn std::error::Error>> {
    
    match run_mode {
        
        RunMode::Dev | RunMode::Test => {
            
            let env_filter = EnvFilter::try_from_default_env()
                .or_else(|_| EnvFilter::try_new(LevelFilter::INFO.to_string()))?;
            
            let simple_collector = tracing_subscriber::registry()
                .with(tracing_subscriber::fmt::layer())
                .with(env_filter);
            
            tracing::subscriber::set_global_default(simple_collector)?;
            
            Ok(())
        }
        RunMode::Prod => {
            
            let env_filter = EnvFilter::try_from_default_env()
                .or_else(|_| EnvFilter::try_new(LevelFilter::WARN.to_string()))?;
            
            let logger = tracing_logstash::Layer::default().event_format(
                tracing_logstash::logstash::LogstashFormat::default()
                    .with_constants(vec![("service.name", "rust-axum-postgres".to_owned())]),
            );

            let collector = Registry::default()
                .with(logger)
                .with(env_filter);
            
            tracing::subscriber::set_global_default(collector)?;
            
            Ok(())
        }
    }
}


