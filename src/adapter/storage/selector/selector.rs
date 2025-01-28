use mockall::automock;
use serde::{Deserialize, Serialize};
use std::{
    collections::{HashMap, HashSet},
    fmt::Debug,
};
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
    fn define_selector<'a>(
        &'a self,
        params: &'a DefineSelectorParams,
    ) -> Result<DefineSelectorResult, SelectorStorageError>;

    fn list_selector<'a>(
        &'a self,
        params: &'a ListSelectorParams,
    ) -> Result<ListSelectorResult, SelectorStorageError>;

    fn get_selector_by_key<'a>(&'a self, key: String) -> Result<Selector, SelectorStorageError> {
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
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Selector {
    // TODO: 现在没想太清楚name的管理逻辑，所以先只留一个key好了
    // name: String,
    pub key: String,
    pub value: HashSet<String>,
}

impl Selector {
    pub fn new(key: String, value: Vec<String>) -> Self {
        Selector {
            key,
            value: value.into_iter().collect(),
        }
    }

    pub fn is_missing_value(&self) -> bool {
        self.value.len() == 0
    }

    pub fn get_key(&self) -> String {
        self.key.clone()
    }

    pub fn add_value(&mut self, value: String) {
        self.value.insert(value);
    }

    pub fn merge(&self, other: &Selector) -> Selector {
        let mut value = self.value.clone();
        for v in &other.value {
            if !value.contains(v) {
                value.insert(v.clone());
            }
        }
        Selector {
            key: self.key.clone(),
            value: value,
        }
    }

    pub fn is_match(&self, kvs: &HashMap<String, String>) -> bool {
        match kvs.get(&self.key) {
            Some(v) => self.value.contains(v),
            None => false,
        }
    }

    pub fn is_match_selectors(selectors: &Selectors, kvs: &HashMap<String, String>) -> bool {
        for s in selectors {
            if !s.is_match(kvs) {
                return false;
            }
        }
        true
    }
}

// 请求的参数定义
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListSelectorParams {
    pub key: Vec<String>,
    // name: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DefineSelectorParams {
    pub key: String,
    pub default_value: String,
    pub set_default_for_history: bool,
}

// 响应的结果定义
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListSelectorResult {
    pub default_selector: Selectors,
    pub selectors: Selectors,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DefineSelectorResult {}
