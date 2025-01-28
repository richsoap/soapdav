use async_trait::async_trait;
use super::{Task, TaskStatus, JsonValue, TaskError};

#[async_trait]
pub trait TaskManager: Send + Sync {

    async fn create_task(
        &self,
        task_type: String,
        task_params: JsonValue,
    ) -> Result<Task,TaskError>;

    async fn query_tasks(
        &self,
        task_type: Option<String>,
        task_status: Option<TaskStatus>,
    ) -> Result<Vec<Task>,TaskError>;

    async fn update_task(
        &self,
        task: Task,
    ) -> Result<Task,TaskError>;
}