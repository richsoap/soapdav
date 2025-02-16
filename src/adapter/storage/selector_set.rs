use std::{fmt::Debug, time};

use super::{Selector, Selectors};
use mockall::automock;
use serde::{Deserialize, Serialize};
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
    fn define_selector_set<'a >(&'a self, params: &'a DefineSelectorSetParams) -> Result<DefineSelectorSetResult, SelectorSetStorageError>;
    
    fn remove_selector_set<'a >(&'a  self, params: &'a RemoveSelectorSetParams) -> Result<RemoveSelectorSetResult, SelectorSetStorageError>;
    
    fn list_selector_set<'a>(&self, params: &'a ListSelectorSetParams) -> Result<ListSelectorSetResult, SelectorSetStorageError>;

    fn get_selector_set_by_name<'a>(&self,name: &'a String) -> Result<SelectorSet, SelectorSetStorageError> {
        let params = ListSelectorSetParams{
            names: vec![name.to_string()],
        };
        let result = self.list_selector_set(&params);
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
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelectorSet {
    pub name: String,
    pub static_selectors: Selectors,
    pub dynamic_selectors: Selectors,
    pub modified_time: Option<std::time::SystemTime>,
}

impl SelectorSet {
    pub fn new(name: &String) -> Self {
        SelectorSet{
            name: name.to_string(),
            static_selectors: vec![],
            dynamic_selectors: vec![],
            modified_time: Some(time::SystemTime::now()),
        }
    }

    pub fn is_full(&self) -> bool {
        self.get_next_required_index().is_none()
    }

    pub fn add_required_value(&mut self, value: String) {
        match self.get_next_required_index() {
            Some(index) => self.dynamic_selectors.get_mut(index).unwrap().add_value(value),
            None => return,
        }
    }

    fn get_next_required_index(&self) -> Option<usize> {
        for (i, s) in self.dynamic_selectors.iter().enumerate() {
            if s.is_missing_value() {
                return Some(i);
            }
        }
        None
    }

    pub fn get_next_required_selector(&self) -> Option<&Selector> {
        match self.get_next_required_index() {
            Some(v) => self.dynamic_selectors.get(v),
            None => None,
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
    pub names: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct ListSelectorSetParams {
    pub names: Vec<String>,
}

// 响应的结果定义
#[derive(Debug, Clone)]
pub struct DefineSelectorSetResult {
}

#[derive(Debug, Clone)]
pub struct RemoveSelectorSetResult {
    pub names: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct ListSelectorSetResult {
    pub selector_set: Vec<SelectorSet>,
}