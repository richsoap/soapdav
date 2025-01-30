use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
/// 漫画作品元数据结构
pub struct ComicMetadata {
    /// 数据库主键
    #[serde(default)]
    pub id: i64,
    /// 完整名称
    #[serde(default)]
    pub title: String,
    /// 作者姓名
    #[serde(default)]
    pub author: String,
    /// 标签
    #[serde(default)]
    pub labels: Vec<String>,
    /// 总页数
    #[serde(default)]
    pub pages: i16,
    /// 资源地址
    #[serde(default)]
    pub url: String,
    /// 来源网站标识
    #[serde(default)]
    pub source: String,
    /// 资源上传者
    #[serde(default)]
    pub uploader: String,
    /// 作品分类
    #[serde(default)]
    pub category: String,
    /// 本地化信息
    #[serde(default)]
    pub language: String,
    /// 评分
    #[serde(default)]
    pub score: i16,
    /// 创建时间
    #[serde(default)]
    pub created_at: DateTime<Local>,
    /// 修改时间
    #[serde(default)]
    pub updated_at: DateTime<Local>,
}

impl ComicMetadata {
    /// 添加新的标签
    fn add_label(&mut self, new_label: &str) {
        self.labels.push(new_label.to_string());
    }

    /// 移除指定标签
    fn remove_label(&mut self, target_label: &str) -> bool {
        if let Some(index) = self.labels.iter().position(|x| x == target_label) {
            self.labels.remove(index);
            true
        } else {
            false
        }
    }
}