use r2d2::ManageConnection;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum StorageError {
    #[error("Task not found: {task_id}")]
    NotFound {
        task_id: i64,
    },

    #[error("Invalid status transition: {from} -> {to}")]
    InvalidStatusTransition {
        from: String,
        to: String,
    },

    #[error("Invalid parameters {params}: {message}")]
    InvalidParams {
        message: String,
        params: String,
    },

    #[error("Concurrent modification detected")]
    ConcurrentUpdate,

    #[error("network error: {0}")]
    NetWorkError(String),

    #[error("database error: {0}")]
    DatabaseError(#[from] diesel::result::Error),
}
