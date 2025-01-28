// 声明子模块
mod models;
mod manager;
mod memory;
mod error;

// 导出公共接口
pub use models::{Task, TaskStatus};
pub use manager::TaskManager;
pub use memory::MemoryTaskManager;
pub use error::TaskError;

// 导出依赖类型（方便外部使用）
pub use serde_json::Value as JsonValue;