use std::{os::macos::raw, ptr::null};

use super::*;
use ::scraper::{Html, Selector};
use async_trait::async_trait;
use reqwest::Client;

pub struct WnacgScraper {
    client: Client,
    title_selector: FieldSelector,
    author_selector: FieldSelector,
    label_selector: FieldSelector,
    page_selector: FieldSelector,
    uploader_selector: FieldSelector,
    category_selector: FieldSelector,
    language_selector: FieldSelector,
}

struct FieldSelector {
    selector: Selector,
    field: String,
}

impl FieldSelector {
    fn new(path: &str, field: &str) -> Result<Self, ScraperError> {
        Ok(FieldSelector {
            selector: Selector::parse(path).map_err(|err| ScraperError::InitializeFailed {
                field: field.into(),
                message: err.to_string(),
            })?,
            field: field.to_string(),
        })
    }
}

impl WnacgScraper {
    pub fn new() -> Result<Self, ScraperError> {
        let result = Self {
            client: Client::new(),
            title_selector: FieldSelector::new("", "title")?,
            author_selector: FieldSelector::new("", "author")?,
            label_selector: FieldSelector::new("", "label")?,
            page_selector: FieldSelector::new("", "page")?,
            uploader_selector: FieldSelector::new("", "uploader")?,
            category_selector: FieldSelector::new("", "category")?,
            language_selector: FieldSelector::new("", "language")?,
        };
        Ok(result)
    }

    /// 从URL中提取作品ID
    fn extract_id_from_url(url: &str) -> Option<u32> {
        url.split("aid-").nth(1)?.split('.').next()?.parse().ok()
    }

    fn select_one_field(document: &Html, selector: &FieldSelector) -> Result<String, ScraperError> {
        document
            .select(&selector.selector)
            .next()
            .map(|e| e.text().collect::<String>().trim().into())
            .ok_or(ScraperError::ElementNotFound {
                name: selector.field.clone(),
            })
    }
}

#[async_trait]
impl Scraper for WnacgScraper {
    async fn scrape_comic(&self, url: String) -> Result<ComicMetadata, ScraperError> {
        // 发送HTTP请求
        let response =
            self.client
                .get(&url)
                .send()
                .await
                .map_err(|err| ScraperError::NetWorkError {
                    message: err.to_string(),
                })?;

        // 获取HTML内容
        let html = response
            .text()
            .await
            .map_err(|err| ScraperError::ParseError {
                message: err.to_string(),
            })?;

        // 解析文档
        let document = Html::parse_document(&html);
        let mut meta = ComicMetadata::default();

        // 提取ID（示例从URL解析）
        {
            meta.id = Self::extract_id_from_url(&url).ok_or(ScraperError::ParseError {
                message: "Invalid URL format".into(),
            })?;
        }
        // 提取文字类型的原始数值
        {
            meta.title = Self::select_one_field(&document, &self.title_selector)?;
            meta.author = Self::select_one_field(&document, &self.author_selector)?;
            meta.uploader = Self::select_one_field(&document, &self.uploader_selector)?;
            meta.category = Self::select_one_field(&document, &self.category_selector)?;
            meta.language = Self::select_one_field(&document, &self.language_selector)?;
        }
        {
            // 提取标签
            let raw_labels = Self::select_one_field(&document, &self.label_selector)?;
            meta.labels = raw_labels
                .split_terminator(';')
                .map(|s| s.to_string())
                .collect();
        }
        {
            // 提取页数
            meta.pages = Self::select_one_field(&document, &self.page_selector)?.parse()?;
        }
        {
            // 设置已有值
            meta.url = url;
            meta.source = "wnacg".into();
        }
        Ok(meta)
    }

    async fn search_comic(&self, name: String) -> Result<ComicMetadata, ScraperError> {
        // 构造搜索URL（示例格式）
        todo!()
    }
}
