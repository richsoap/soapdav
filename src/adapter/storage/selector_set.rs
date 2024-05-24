use std::{fmt::Debug, time};

use super::Selectors;
use mockall::automock;
use thiserror::Error;

// 定义 SelectorSetStorage 错误, 用于处理可能出现的错误情况
#[derive(Error, Debug)]
pub enum SelectorSetStorageError {
    #[error("NotFound")]
    NotFound,
}

// SelectorSetStorage trait
// TODO:(yangqinglong) add modifier selector_set api
#[automock]
pub trait SelectorSetStorage: Send + Sync + Debug {
    fn define_selector_set(&self, params: DefineSelectorSetParams) -> Result<DefineSelectorSetResult, SelectorSetStorageError>;
    
    fn remove_selector_set(&self, params: RemoveSelectorSetParams) -> Result<RemoveSelectorSetResult, SelectorSetStorageError>;
    
    fn list_selector_set<'a>(&self, params: ListSelectorSetParams<'a>) -> Result<ListSelectorSetResult, SelectorSetStorageError>;

    fn get_selector_set_by_name<'a>(&self,name: &String) -> Result<SelectorSet, SelectorSetStorageError> {
        let params = ListSelectorSetParams{
            name: &vec![name.to_string()],
        };
        let result = self.list_selector_set(params);
        match result {
            Ok(res) => match res.selector_set.get(0) {
                Some(v) => Ok(v.clone()),
                None => Err(SelectorSetStorageError::NotFound),
            },
            Err(e) => Err(e),
        }
    }
}

// SelectorSet 的定义
#[derive(Debug, Clone)]
pub struct SelectorSet {
    pub name: String,
    pub selectors: Selectors,
    pub required_selectors: Selectors,
    pub modified_time: std::time::SystemTime,
}

impl SelectorSet {
    pub fn new(name: &String) -> Self {
        SelectorSet{
            name: name.to_string(),
            selectors: vec![],
            required_selectors: vec![],
            modified_time: time::SystemTime::now(),
        }
    }
}

// 请求的参数定义
#[derive(Debug, Clone)]
pub struct DefineSelectorSetParams {
    pub selector_sets: Vec<SelectorSet>,
}

#[derive(Debug, Clone)]
pub struct RemoveSelectorSetParams {
    pub name: String,
}

#[derive(Debug, Clone)]
pub struct ListSelectorSetParams<'a> {
    pub name:&'a Vec<String>,
}

// 响应的结果定义
#[derive(Debug, Clone)]
pub struct DefineSelectorSetResult {

}

#[derive(Debug, Clone)]
pub struct RemoveSelectorSetResult {
    pub name: String,
}

#[derive(Debug, Clone)]
pub struct ListSelectorSetResult {
    pub selector_set: Vec<SelectorSet>,
}