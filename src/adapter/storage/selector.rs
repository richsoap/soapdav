use mockall::automock;
use std::fmt::Debug;
use thiserror::Error;

// 定义 SelectorStorage 错误, 用于处理可能出现的错误情况
#[derive(Error, Debug)]
pub enum SelectorStorageError {
    #[error("NotFound")]
    NotFound,
}

// SelectorStorage trait
#[automock]
pub trait SelectorStorage: Send + Sync + Debug {
    fn define_selector(
        &mut self,
        params: &DefineSelectorParams,
    ) -> Result<DefineSelectorResult, SelectorStorageError>;

    fn list_selector(
        &self,
        params: &ListSelectorParams,
    ) -> Result<ListSelectorResult, SelectorStorageError>;

    fn get_selector_by_key(&self, key: String) -> Result<Selector, SelectorStorageError> {
        match self.list_selector(&ListSelectorParams { key: vec![key] }) {
            Ok(res) => match res.selectors.get(0) {
                Some(v) => Ok(v.clone()),
                None => Err(SelectorStorageError::NotFound),
            },
            Err(e) => Err(e),
        }
    }
}

pub type Selectors = Vec<Selector>;

// Selector 的定义
#[derive(Debug, Clone)]
pub struct Selector {
    // TODO: 现在没想太清楚name的管理逻辑，所以先只留一个key好了
    // name: String,
    pub key: String,
    pub value: Vec<String>,
}

impl Selector {
    pub fn is_missing_value(&self) -> bool {
        self.value.len() == 0
    }

    pub fn get_key(&self)->String {
        self.key.clone()
    }

    pub fn add_value(&mut self, value: String) {
        self.value.push(value)
    }
}

// 请求的参数定义
#[derive(Debug, Clone)]
pub struct ListSelectorParams {
    pub key: Vec<String>,
    // name: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct DefineSelectorParams {
    pub key: String,
    pub default_value: String,
    pub set_default_for_history: bool,
}

// 响应的结果定义
#[derive(Debug, Clone)]
pub struct ListSelectorResult {
    pub default_selector: Selectors,
    pub selectors: Selectors,
}

#[derive(Debug, Clone)]
pub struct DefineSelectorResult {}
