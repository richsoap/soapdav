

use async_trait::async_trait;
use serde_json::Value as JsonValue;
use super::{TaskError, Task, TaskStatus, TaskManager};
use chrono::{DateTime, Local};

// 示例实现：内存后端（用于测试）
pub struct MemoryTaskManager {
    storage: tokio::sync::RwLock<Vec<Task>>,
}

impl MemoryTaskManager {
    pub fn new() -> Self {
        Self {
            storage: tokio::sync::RwLock::new(Vec::new()),
        }
    }
}

#[async_trait]
impl TaskManager for MemoryTaskManager {
    async fn create_task(
        &self,
        task_type: String,
        task_params: JsonValue,
    ) -> Result<Task, TaskError> {
        let mut storage = self.storage.write().await;
        let new_task = Task {
            id: storage.len() as i64 + 1,
            task_type,
            task_status: TaskStatus::Pending,
            task_params,
            created_at: Local::now(),
            updated_at: Local::now(),
        };
        storage.push(new_task.clone());
        Ok(new_task)
    }

    async fn query_tasks(
        &self,
        task_type: Option<String>,
        task_status: Option<TaskStatus>,
    ) -> Result<Vec<Task>,TaskError> {
        let storage = self.storage.read().await;
        Ok(storage
            .iter()
            .filter(|t| task_type.as_ref().map_or(true, |tt| &t.task_type == tt))
            .filter(|t| task_status.map_or(true, |ts| t.task_status == ts))
            .cloned()
            .collect())
    }

    async fn update_task(
        &self,
        task: Task,
    ) -> Result<Task,TaskError> {
        let mut storage = self.storage.write().await;
        let index = storage.iter()
            .position(|t| t.id == task.id)
            .ok_or_else(|| TaskError::NotFound { task_id: task.id })?;
        
        let mut updated = task.clone();
        updated.updated_at = Local::now();
        storage[index] = updated.clone();
        Ok(updated)
    }
}