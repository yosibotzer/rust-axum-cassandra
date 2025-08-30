


#[derive(thiserror::Error, Debug)]
pub enum InternalError {

    #[error("Database prepare error: {0}")]
    PrepareError(#[from] scylla::errors::PrepareError),

    #[error("Database execution error: {0}")]
    ExecutionError(#[from] scylla::errors::ExecutionError),

    #[error("Database session error: {0}")]
    NewSessionError(#[from] scylla::errors::NewSessionError),
    
    #[error("Database rows error: {0}")]
    RowsError(#[from] scylla::errors::RowsError),

    #[error("Database maybe first row error: {0}")]
    MaybeFirstRowError(#[from] scylla::errors::MaybeFirstRowError),
    
    #[error("Database into rows result error: {0}")]
    IntoRowsResultError(#[from] scylla::errors::IntoRowsResultError),

}