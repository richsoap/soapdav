mod manager;
mod database;

// 导出依赖类型（方便外部使用）
use serde_json::Value as JsonValue;
use super::error::*;
pub use database::*;
pub use manager::*;