use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Command execution failed: {0}")]
    CommandExecution(String),

    #[error("JSON parsing failed: {0}")]
    JsonParsing(#[from] serde_json::Error),

    #[error("Environment variable not found: {0}")]
    EnvVar(#[from] std::env::VarError),

    #[error("Schema validation failed: {0}")]
    SchemaValidation(String),

    #[error("Transaction failed: {0}")]
    TransactionFailed(String),

    #[error("Query failed: {0}")]
    QueryFailed(String),

    #[error("Contract error: {0}")]
    ContractError(String),
} 