use std::os::macos::raw;

use super::*;
use crate::model::ComicMetadata;
use ::scraper::{Html, Selector};
use async_trait::async_trait;
use reqwest::Client;
use log::info;

pub struct WnacgScraper {
    client: Client,
    title_selector: FieldSelector,
    label_selector: FieldSelector,
    page_selector: FieldSelector,
    uploader_selector: FieldSelector,
    category_selector: FieldSelector,
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
            title_selector: FieldSelector::new("h2", "title")?,
            label_selector: FieldSelector::new("a.tagshow", "label")?,
            page_selector: FieldSelector::new(".uwconn label:nth-child(2)", "page")?,
            uploader_selector: FieldSelector::new(".uwuinfo p", "uploader")?,
            category_selector: FieldSelector::new(".uwconn label:nth-child(1)", "category")?,
        };
        Ok(result)
    }

    // 作者一般放在标题里，用方括号表示
    fn extract_author_from_title(title: &String) -> Option<String> {
        if let Some(start) = title.find('[') {
            if let Some(end) = title.find(']') {
                return Some(title[start + 1..end].into());
            }
        }
        None
    }

    // 从分类中获取类型和语言
    fn extract_cate_and_lang(s: &String) -> Vec<String> {
        // 按 ： 分割字符串，取第二部分
        let parts = s.split("：").nth(1);
        if let Some(part) = parts {
            // 再按 ／ 分割字符串
            part.split("／").map(|s| s.to_string()).collect()
        } else {
            Vec::new()
        }
    }

    fn select_first_field(
        document: &Html,
        selector: &FieldSelector,
    ) -> Result<String, ScraperError> {
        let result = Self::select_all_field(document, selector);
        if result.is_empty() {
            Err(ScraperError::ElementNotFound {
                name: selector.field.clone(),
            })
        } else {
            Ok(result.get(0).cloned().unwrap_or_default())
        }
    }

    fn select_all_field(document: &Html, selector: &FieldSelector) -> Vec<String> {
        document
            .select(&selector.selector)
            .map(|e| e.text().collect::<String>().trim().to_string())
            .collect()
    }

    // 解析文档
    fn scrape_comic_from_document(
        &self,
        url: &String,
        document: &Html,
    ) -> Result<ComicMetadata, ScraperError> {
        let mut meta = ComicMetadata::default();
        // 提取文字类型的原始数值
        {
            meta.title = Self::select_first_field(&document, &self.title_selector)?;
            meta.author = Self::extract_author_from_title(&meta.title).unwrap_or_default();
            meta.uploader = Self::select_first_field(&document, &self.uploader_selector)?;
            // 类型和语言是放在一起的
            {
                let raw_category = Self::select_first_field(document, &self.category_selector)?;
                let cates = Self::extract_cate_and_lang(&raw_category);
                meta.category = cates.get(0).cloned().unwrap_or_default();
                meta.language = cates.get(1).cloned().unwrap_or_default();
            }
            meta.labels = Self::select_all_field(document, &self.label_selector)
        }
        {
            // 提取页数
            let raw_page = Self::select_first_field(&document, &self.page_selector)?;
            meta.pages = raw_page
                .chars()
                .filter(|c| c.is_digit(10))
                .collect::<String>()
                .parse()?;
        }
        {
            // 设置已有值
            meta.url = url.clone();
            meta.source = "wnacg".into();
        }
        Ok(meta)
    }
}

#[async_trait]
impl Scraper for WnacgScraper {
    async fn scrape_comic(&self, url: &String) -> Result<ComicMetadata, ScraperError> {
        // 发送HTTP请求
        let response =
            self.client
                .get(url)
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
        let document = Html::parse_document(&html);
        self.scrape_comic_from_document(url, &document)
    }

    async fn search_comic(&self, name: &String) -> Result<ComicMetadata, ScraperError> {
        // 构造搜索URL（示例格式）
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::io::{self, Read};
    use std::path::{Path, PathBuf};

    use scraper::Html;
    use serde::{Deserialize, Serialize};

    use crate::model::ComicMetadata;

    use super::super::WnacgScraper;

    #[derive(Serialize, Deserialize, Debug)]
    struct TestCase {
        input: TestInput,
        output: ComicMetadata,
    }

    #[derive(Serialize, Deserialize, Debug)]
    struct TestInput {
        url: String,
        body: String,
    }

    impl PartialEq for ComicMetadata {
        fn eq(&self, other: &Self) -> bool {
            self.title == other.title
                && self.author == other.author
                && self.labels == other.labels
                && self.pages == other.pages
                && self.url == other.url
                && self.source == other.source
                && self.uploader == other.uploader
                && self.category == other.category
                && self.language == other.language
        }
    }

    #[test]
    fn test_scraper_from_document() {
        let testcase_dir = "testdata/wnacg";
        let mut path = PathBuf::from(file!());
        path.pop();
        path.push(testcase_dir);
        let paths = fs::read_dir(path.to_str().unwrap()).unwrap();
        let s = WnacgScraper::new().unwrap();

        for path in paths {
            let path = path.unwrap();
            let mut file = fs::File::open(path.path()).unwrap();
            let mut contents = String::new();
            file.read_to_string(&mut contents).unwrap();
            let testcase: TestCase = serde_yaml::from_str(&contents).unwrap();
            test_scraper_from_document_for_case(&s, &testcase);
        }
    }

    fn test_scraper_from_document_for_case(s: &WnacgScraper, testcase: &TestCase) {
        let document = Html::parse_document(&testcase.input.body);
        let result = s
            .scrape_comic_from_document(&testcase.input.url, &document)
            .unwrap();
        assert_eq!(
            result, testcase.output,
            "meta is not match: url{}",
            testcase.input.url
        )
    }
}
