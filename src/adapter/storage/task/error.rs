use serde_json::Value as JsonValue;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum TaskError {
    #[error("Task not found: {task_id}")]
    NotFound {
        task_id: i64,
    },

    #[error("Invalid task status transition: {from} -> {to}")]
    InvalidStatusTransition {
        from: String,
        to: String,
    },

    #[error("Invalid task parameters: {message}")]
    InvalidParams {
        message: String,
        params: JsonValue,
    },

    #[error("Concurrent modification detected")]
    ConcurrentUpdate,
}

// 可选：自定义错误转换
impl From<serde_json::Error> for TaskError {
    fn from(err: serde_json::Error) -> Self {
        TaskError::InvalidParams {
            message: format!("JSON parse error: {}", err),
            params: JsonValue::Null,
        }
    }
}
