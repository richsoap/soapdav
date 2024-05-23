use std::fmt::Debug;

use super::Selectors;
use mockall::automock;
use thiserror::Error;

// 定义 SelectorSetStorage 错误, 用于处理可能出现的错误情况
#[derive(Error, Debug)]
pub enum SelectorSetStorageError {
}

// SelectorSetStorage trait
#[automock]
pub trait SelectorSetStorage: Send + Sync + Debug {
    fn define_selector_set(&self, params: DefineSelectorSetParams) -> Result<DefineSelectorSetResult, SelectorSetStorageError>;
    
    fn remove_selector_set(&self, params: RemoveSelectorSetParams) -> Result<RemoveSelectorSetResult, SelectorSetStorageError>;
    
    fn list_selector_set<'a>(&self, params: ListSelectorSetParams<'a>) -> Result<ListSelectorSetResult, SelectorSetStorageError>;
}

// SelectorSet 的定义
#[derive(Debug, Clone)]
pub struct SelectorSet {
    pub name: String,
    pub selectors: Selectors,
    pub required_selectors: Selectors,
    pub modified_time: std::time::SystemTime,
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