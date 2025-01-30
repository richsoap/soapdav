use super::{
    comics::{self},
    StorageError,
};
use crate::model::*;
use async_trait::async_trait;
use chrono::{DateTime, Local};
use diesel::{
    helper_types::IntoBoxed, query_dsl::methods::FilterDsl, BoxableExpression, QueryDsl,
    SqliteConnection,
};
use serde::{Deserialize, Serialize};

// ---------- 元数据结构体 ----------

// ---------- 查询条件 ----------
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ComicQuery {
    pub id_in: Option<Vec<i64>>,
    pub title_in: Option<Vec<String>>,
    pub author_in: Option<Vec<String>>,
    pub labels_in: Option<Vec<String>>,
    pub pages_gt: Option<i16>,
    pub pages_lt: Option<i16>,
    pub source_in: Option<Vec<String>>,
    pub uploader_in: Option<Vec<String>>,
    pub category_in: Option<Vec<String>>,
    pub language_in: Option<Vec<String>>,
    pub score_gt: Option<i16>,
    pub score_lt: Option<i16>,
    pub create_gt: Option<DateTime<Local>>,
    pub create_lt: Option<DateTime<Local>>,
    pub update_gt: Option<DateTime<Local>>,
    pub update_lt: Option<DateTime<Local>>,
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
    pub page: i64,
    pub page_size: i64,
}

impl Default for PageParams {
    fn default() -> Self {
        Self {
            page: 1,
            page_size: 50,
        }
    }
}

// ---------- 分页结果 ----------
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PagedResult<T> {
    pub data: Vec<T>,
    pub total: i64,
    pub page: i64,
    pub page_size: i64,
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
    async fn delete(&self, id: i64) -> Result<(), StorageError>;

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
