use async_trait::async_trait;
use super::{JsonValue, StorageError};
use crate::model::*;

#[async_trait]
pub trait TaskManager: Send + Sync {
    async fn create_task(
        &self,
        task_type: TaskType,
        task_params: String,
    ) -> Result<Task,StorageError>;

    async fn query_tasks(
        &self,
        task_type: Option<TaskType>,
        task_status: Option<TaskStatus>,
    ) -> Result<Vec<Task>,StorageError>;

    async fn update_task(
        &self,
        task: Task,
    ) -> Result<Task,StorageError>;
}