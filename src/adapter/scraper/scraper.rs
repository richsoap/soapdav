use async_trait::async_trait;
use super::*;
use crate::model::ComicMetadata;
use ::scraper::Html;

#[async_trait]
pub trait Scraper: Send + Sync {
    async fn scrape_comic(&self, url: &String) -> Result<ComicMetadata,ScraperError>;

    async fn search_comic(&self, name: &String) -> Result<ComicMetadata,ScraperError>;
}