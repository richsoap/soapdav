use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use async_trait::async_trait;
use super::StorageError;
use crate::model::*;

// ---------- 元数据结构体 ----------

// ---------- 查询条件 ----------
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ComicQuery {
    pub id_in: Option<Vec<u32>>,
    pub title_in: Option<Vec<String>>,
    pub author_in: Option<Vec<String>>,
    pub labels_in: Option<Vec<String>>,
    pub pages_gt: Option<u16>,
    pub pages_let: Option<u16>,
    pub source_in: Option<Vec<String>>,
    pub uploader_in: Option<Vec<String>>,
    pub category_in: Option<Vec<String>>,
    pub language_in: Option<Vec<String>>,
    pub score_gt: Option<u16>,
    pub score_let: Option<u16>,
    pub update_gt: Option<DateTime<Local>>,
    pub update_let: Option<DateTime<Local>>,
    pub create_gt: Option<DateTime<Local>>,
    pub create_let: Option<DateTime<Local>>,
}

// ---------- 排序规则 ----------
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OrderableColumn {
    Id,
    Title,
    Author,
    Pages,
    Uploader,
    Category,
    Language,
    Score,
    CreatedAt,
    UpdatedAt,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SortDirection {
    Asc,
    Desc,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComicOrder {
    pub by: OrderableColumn,
    pub direction: SortDirection,
}

// ---------- 分页参数 ----------
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageParams {
    pub page: u32,
    pub page_size: u32,
}

// ---------- 分页结果 ----------
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PagedResult<T> {
    pub data: Vec<T>,
    pub total: u64,
    pub page: u32,
    pub page_size: u32,
}

// ---------- 列属性的分布情况 -------
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CountableColumn {
    Author,
    Labels,
    Source,
    Uploader,
    Category,
    Language,
    Score,
}

pub struct ColumnDistributionResult {
    pub name: String,
    pub count: u64,
}

// ---------- Manager Trait ----------
#[async_trait]
pub trait ComicManager {
    /// 创建漫画元数据
    async fn create(&self, comic: &ComicMetadata) -> Result<ComicMetadata, StorageError>;

    /// 更新漫画元数据
    async fn update(&self, comic: &ComicMetadata) -> Result<(), StorageError>;

    /// 删除漫画元数据
    async fn delete(&self, id: u32) -> Result<(), StorageError>;

    /// 分页查询（支持 In 条件和排序）
    async fn query(
        &self,
        query: ComicQuery,
        order: Option<Vec<ComicOrder>>,
        page: Option<PageParams>,
    ) -> Result<PagedResult<ComicMetadata>, StorageError>;

    // 获取某列的数据分布
    async fn query_distribution(
        &self,
        query: ComicQuery,
    ) -> Result<Vec<ColumnDistributionResult>, StorageError>;
}