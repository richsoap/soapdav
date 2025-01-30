// 声明子模块
mod manager;
mod memory;

// 导出公共接口
pub use manager::TaskManager;
pub use memory::MemoryTaskManager;

// 导出依赖类型（方便外部使用）
pub use serde_json::Value as JsonValue;
use super::error::*;