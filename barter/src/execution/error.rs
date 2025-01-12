use barter_execution::error::ClientError;
use barter_instrument::index::error::IndexError;
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Represents all errors generated by the execution link.
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Deserialize, Serialize, Error)]
pub enum ExecutionError {
    /// Indicates an invalid execution manager configuration.
    #[error("ExecutionManager config invalid: {0}")]
    Config(String),

    /// Represents an error that occurred whilst mapping exchange-centric data structures into
    /// their indexed counterparts.
    #[error("IndexError: {0}")]
    Index(#[from] IndexError),

    /// Represents all errors produced by an
    /// [`ExecutionClient`](barter_execution::client::ExecutionClient).
    #[error("{0}")]
    Client(#[from] ClientError),
}
