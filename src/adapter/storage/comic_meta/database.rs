use chrono::NaiveDateTime;
use ::diesel::{Connection, SqliteConnection};
use ::diesel::prelude::*;
use serde::{Serialize, Deserialize};
use crate::model::*;
use super::*;

use crate::adapter::storage::error::StorageError;

table! {
    comics(id) {
        id -> BigInt,
        title -> Text,
        author -> Text,
        pages -> SmallInt,
        url -> Text,
        source -> Text,
        uploader -> Text,
        category -> Text,
        language -> Text,
        score -> SmallInt,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    labels(id) {
        id -> BigInt,
        comic_id -> Integer,
        label -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

#[derive(Insertable, Queryable, AsChangeset)]
#[diesel(table_name = comics)]
/// 漫画作品元数据结构
struct ComicMetadataEntity {
    /// 作品唯一标识
    pub id: i64,
    /// 完整名称
    pub title: String,
    /// 作者姓名
    pub author: String,
    /// 总页数
    pub pages: i16,
    /// 资源地址
    pub url: String,
    /// 来源网站标识
    pub source: String,
    /// 资源上传者
    pub uploader: String,
    /// 作品分类
    pub category: String,
    /// 本地化信息
    pub language: String,
    /// 评分
    pub score: i16,
    /// 创建时间
    pub created_at: NaiveDateTime,
    /// 修改时间
    pub updated_at: NaiveDateTime,
}

impl From<ComicMetadata> for ComicMetadataEntity {
    fn from(value: ComicMetadata) -> Self {
        ComicMetadataEntity{
            id: value.id,
            title: value.title,
            author: value.author,
            pages: value.pages,
            url: value.url,
            source: value.source,
            uploader: value.uploader,
            category: value.category,
            language: value.language,
            score: value.score,
            created_at: value.created_at.naive_local(),
            updated_at: value.updated_at.naive_local(),
            
        }
    }
}

struct ComicLabelEntity {
    pub id: u64,
    pub comic_id: u64,
    pub label: String,
}

pub struct DieselManager {
    sqlLite: SqliteConnection,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DataBaseType {
    // MySQL,
    // PostgressSQL,
    SQLite,
}

impl DieselManager {
    pub fn new(database_type: DataBaseType, url: String) -> Result<Self, StorageError> {
        Ok(Self {
            sqlLite: match database_type {
                // DataBaseType::MySQL => {
                //     MysqlConnection::establish(&url).map_err(|err| StorageError::NetWorkError {
                //         message: err.to_string(),
                //     })
                // }
                DataBaseType::SQLite => {
                    SqliteConnection::establish(&url).map_err(|err| StorageError::NetWorkError {
                        message: err.to_string(),
                    })
                }
            },
        })
    }
}

#[async_trait]
impl ComicManager for DatabaseManager {
    async fn create(&self, comic: &ComicMetadata) -> Result<(), ManagerError> {
        let entity = comic.into();
        database::insert_into(comics::table)
            .values(&entity)
            .execute(&self.conn)
            .map_err(|e| ManagerError::Database(e.to_string()))?;
        Ok(())
    }

    async fn query(
        &self,
        query: ComicQuery,
        order: Option<ComicOrder>,
        page: Option<PageParams>,
    ) -> Result<PagedResult<ComicMetadata>, ManagerError> {
        let mut query_builder = comics::table.into_boxed();

        // 添加 In 条件
        if let Some(ids) = query.id_in {
            query_builder = query_builder.filter(comics::id.eq_any(ids));
        }
        // 其他字段条件...

        // 添加排序
        if let Some(order) = order {
            match order.by {
                ComicOrderBy::CreatedAt => {
                    if order.direction == SortDirection::Asc {
                        query_builder = query_builder.order(comics::created_at.asc());
                    } else {
                        query_builder = query_builder.order(comics::created_at.desc());
                    }
                } // 其他字段排序...
            }
        }

        // 分页
        if let Some(page) = page {
            let offset = (page.page - 1) * page.page_size;
            query_builder = query_builder
                .limit(page.page_size as i64)
                .offset(offset as i64);
        }

        // 执行查询并转换为 PagedResult
        let data: Vec<ComicEntity> = query_builder
            .load(&self.conn)
            .map_err(|e| ManagerError::Database(e.to_string()))?;

        Ok(PagedResult {
            data: data.into_iter().map(|e| e.into()).collect(),
            total: 100, // 实际需要查询总数
            page: page.unwrap_or_default().page,
            page_size: page.unwrap_or_default().page_size,
        })
    }

    // 其他方法实现...
}
