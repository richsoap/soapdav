use std::default;

use super::*;
use crate::model::*;
use ::diesel::prelude::*;
use ::diesel::{Connection, SqliteConnection};
use async_trait::async_trait;
use chrono::{DateTime, Local, NaiveDateTime, TimeZone};
use diesel::dsl::count;
use diesel::query_builder;
use diesel::r2d2::ConnectionManager;
use r2d2::Pool;
use serde::{Deserialize, Serialize};

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
        ComicMetadataEntity {
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
            created_at: value.created_at.naive_utc(),
            updated_at: value.updated_at.naive_utc(),
        }
    }
}

impl From<ComicMetadataEntity> for ComicMetadata {
    fn from(value: ComicMetadataEntity) -> Self {
        ComicMetadata {
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
            created_at: match Local.from_local_datetime(&value.created_at) {
                chrono::offset::LocalResult::Single(t) => t,
                chrono::offset::LocalResult::Ambiguous(t, _) => t,
                chrono::offset::LocalResult::None => Local::now(),
            },
            updated_at: match Local.from_local_datetime(&value.updated_at) {
                chrono::offset::LocalResult::Single(t) => t,
                chrono::offset::LocalResult::Ambiguous(t, _) => t,
                chrono::offset::LocalResult::None => Local::now(),
            },
            labels: vec![],
        }
    }
}

struct ComicLabelEntity {
    pub id: u64,
    pub comic_id: u64,
    pub label: String,
}

pub struct DatabaseManager {
    pool: Pool<ConnectionManager<SqliteConnection>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DataBaseType {
    // MySQL,
    // PostgressSQL,
    SQLite,
}

impl DatabaseManager {
    pub fn new(database_type: DataBaseType, url: String) -> Result<Self, StorageError> {
        let manager = ConnectionManager::new(&url);
        let pool = Pool::builder()
            .build(manager)
            .map_err(|e| StorageError::InvalidParams {
                message: e.to_string(),
                params: url.clone(),
            })?;
        Ok(Self { pool: pool })
    }

    pub fn apply_filters<'a>(
        query: &'a ComicQuery,
        query_builder: comics::BoxedQuery<'a, diesel::sqlite::Sqlite>,
    ) -> comics::BoxedQuery<'a, diesel::sqlite::Sqlite> {
        let mut query_builder = query_builder;
        // TODO: labels需要加Join
        if let Some(vs) = &query.id_in {
            query_builder = query_builder.filter(comics::id.eq_any(vs));
        }
        if let Some(vs) = &query.title_in {
            query_builder = query_builder.filter(comics::title.eq_any(vs));
        }
        if let Some(vs) = &query.author_in {
            query_builder = query_builder.filter(comics::author.eq_any(vs));
        }
        if let Some(vs) = &query.source_in {
            query_builder = query_builder.filter(comics::source.eq_any(vs));
        }
        if let Some(vs) = &query.uploader_in {
            query_builder = query_builder.filter(comics::uploader.eq_any(vs));
        }
        if let Some(vs) = &query.language_in {
            query_builder = query_builder.filter(comics::language.eq_any(vs));
        }
        if let Some(v) = query.pages_gt {
            query_builder = query_builder.filter(comics::pages.gt(v));
        }
        if let Some(v) = query.pages_lt {
            query_builder = query_builder.filter(comics::pages.lt(v));
        }
        if let Some(v) = query.score_gt {
            query_builder = query_builder.filter(comics::score.gt(v));
        }
        if let Some(v) = query.score_lt {
            query_builder = query_builder.filter(comics::score.lt(v));
        }
        if let Some(v) = query.create_gt {
            query_builder = query_builder.filter(comics::created_at.gt(v.naive_utc()));
        }
        if let Some(v) = query.create_lt {
            query_builder = query_builder.filter(comics::created_at.lt(v.naive_utc()));
        }
        if let Some(v) = query.update_gt {
            query_builder = query_builder.filter(comics::updated_at.gt(v.naive_utc()));
        }
        if let Some(v) = query.update_lt {
            query_builder = query_builder.filter(comics::updated_at.lt(v.naive_utc()));
        }
        query_builder
    }

    pub fn apply_orders<'a>(
        os: Vec<ComicOrder>,
        query_builder: comics::BoxedQuery<'a, diesel::sqlite::Sqlite>,
    ) -> comics::BoxedQuery<'a, diesel::sqlite::Sqlite> {
        let mut query_builder = query_builder;
        for o in os.iter() {
            query_builder = match o.by {
                OrderableColumn::Id => match o.direction {
                    SortDirection::Asc => query_builder.order(comics::id.asc()),
                    SortDirection::Desc => query_builder.order(comics::id.desc()),
                },
                OrderableColumn::Title => match o.direction {
                    SortDirection::Asc => query_builder.order(comics::title.asc()),
                    SortDirection::Desc => query_builder.order(comics::title.desc()),
                },
                OrderableColumn::Author => match o.direction {
                    SortDirection::Asc => query_builder.order(comics::author.asc()),
                    SortDirection::Desc => query_builder.order(comics::author.desc()),
                },
                OrderableColumn::Pages => match o.direction {
                    SortDirection::Asc => query_builder.order(comics::pages.asc()),
                    SortDirection::Desc => query_builder.order(comics::pages.desc()),
                },
                OrderableColumn::Uploader => match o.direction {
                    SortDirection::Asc => query_builder.order(comics::uploader.asc()),
                    SortDirection::Desc => query_builder.order(comics::uploader.desc()),
                },
                OrderableColumn::Category => match o.direction {
                    SortDirection::Asc => query_builder.order(comics::category.asc()),
                    SortDirection::Desc => query_builder.order(comics::category.desc()),
                },
                OrderableColumn::Language => match o.direction {
                    SortDirection::Asc => query_builder.order(comics::language.asc()),
                    SortDirection::Desc => query_builder.order(comics::language.desc()),
                },
                OrderableColumn::Score => match o.direction {
                    SortDirection::Asc => query_builder.order(comics::score.asc()),
                    SortDirection::Desc => query_builder.order(comics::score.desc()),
                },
                OrderableColumn::CreatedAt => match o.direction {
                    SortDirection::Asc => query_builder.order(comics::created_at.asc()),
                    SortDirection::Desc => query_builder.order(comics::created_at.desc()),
                },
                OrderableColumn::UpdatedAt => match o.direction {
                    SortDirection::Asc => query_builder.order(comics::updated_at.asc()),
                    SortDirection::Desc => query_builder.order(comics::updated_at.desc()),
                },
            };
        }
        query_builder
    }
}

#[async_trait]
impl ComicManager for DatabaseManager {
    async fn create(&self, comic: &ComicMetadata) -> Result<ComicMetadata, StorageError> {
        let mut conn = self
            .pool
            .get()
            .map_err(|e| StorageError::NetWorkError(e.to_string()))?;
        let entity: ComicMetadataEntity = comic.clone().into();
        conn.transaction(|conn| {
            diesel::insert_into(comics::table)
                .values(&entity)
                .execute(conn)?;
            // TODO: label也要单独插入一下
            Ok(entity.into())
        })
    }

    async fn query(
        &self,
        query: ComicQuery,
        order: Option<Vec<ComicOrder>>,
        page: Option<PageParams>,
    ) -> Result<PagedResult<ComicMetadata>, StorageError> {
        let mut conn = self
            .pool
            .get()
            .map_err(|e| StorageError::NetWorkError(e.to_string()))?;

        // 统计数量
        let mut count_query = comics::table.into_boxed();
        count_query = Self::apply_filters(&query, count_query);
        count_query = Self::apply_orders(order.clone().unwrap_or_default(), count_query);
        let amount: i64 = count_query.select(count(comics::id)).first(&mut conn)?;

        // 获取详情
        let mut query_builder = comics::table.into_boxed();
        query_builder = Self::apply_filters(&query, query_builder);
        query_builder = Self::apply_orders(order.unwrap_or_default(), query_builder);
        let page_param = page.unwrap_or_default();
        let offset = (page_param.page - 1) * page_param.page_size;
        query_builder = query_builder
            .limit(page_param.page_size as i64)
            .offset(offset as i64);

        let data: Vec<ComicMetadataEntity> = query_builder
            .load(&mut conn)
            .map_err(|e| StorageError::NetWorkError(e.to_string()))?;

        // 执行查询并转换为 PagedResult

        Ok(PagedResult {
            data: data.into_iter().map(|e| e.into()).collect(),
            total: amount, // 实际需要查询总数
            page: page_param.page,
            page_size: page_param.page_size,
        })
    }

    /// 更新漫画元数据
    async fn update(&self, comic: &ComicMetadata) -> Result<(), StorageError> {
        todo!()
    }

    /// 删除漫画元数据
    async fn delete(&self, id: i64) -> Result<(), StorageError> {
        todo!()
    }
    // 获取某列的数据分布
    async fn query_distribution(
        &self,
        query: ComicQuery,
    ) -> Result<Vec<ColumnDistributionResult>, StorageError> {
        todo!()
    }
}

impl ComicOrder {}
