use thiserror::Error;

// 定义 SelectorStorage 错误, 用于处理可能出现的错误情况
#[derive(Error, Debug)]
pub enum SelectorStorageError {}

// SelectorStorage trait
pub trait SelectorStorage {
    fn define_selector(
        &self,
        params: DefineSelectorParams,
    ) -> Result<DefineSelectorResult, SelectorStorageError>;
    fn list_selector(
        &self,
        params: ListSelectorParams,
    ) -> Result<ListSelectorResult, SelectorStorageError>;
}

pub type Selectors = Vec<Selector>;

// Selector 的定义
#[derive(Debug, Clone)]
pub struct Selector {
    key: String,
    name: String,
    value: Vec<String>,
}

impl Selector {
    pub fn is_missing_value(&self) -> bool {
        self.value.len() == 0
    }
}

// 请求的参数定义
#[derive(Debug, Clone)]
pub struct ListSelectorParams {
    key: Vec<String>,
    name: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct DefineSelectorParams {
    key: String,
    name: String,
}

// 响应的结果定义
#[derive(Debug, Clone)]
pub struct ListSelectorResult {
    selectors: Selectors,
}

#[derive(Debug, Clone)]
pub struct DefineSelectorResult {}
