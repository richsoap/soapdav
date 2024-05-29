use super::{SelectorStorage, Selectors};
use std::collections::HashMap;
use thiserror::Error;

// 定义 KVFileStorage 错误, 用于处理可能出现的错误情况
#[derive(Error, Debug)]
pub enum KVFileStorageError {
    #[error("NotFound")]
    NotFound,
}

// KVFileStorage trait
pub trait KVFileStorage: SelectorStorage {
    fn list_file(&self, params: ListFileParams) -> Result<ListFileResult, KVFileStorageError>;

    fn add_file(&mut self, params: AddFileParams) -> Result<AddFileResult, KVFileStorageError>;

    fn remove_file(&mut self, params: RemoveFileParams)
        -> Result<RemoveFileResult, KVFileStorageError>;

    fn set_label(&mut self, params: SetLabelParams) -> Result<SetLabelResult, KVFileStorageError>;
}

// KV 定义
#[derive(Debug, Clone)]
pub struct KV {
    key: String,
    value: String,
}

pub type KVs = Vec<KV>;

impl KV {
    pub fn find_value(kvs: &KVs, key: &String) -> Option<String> {
        for kv in kvs {
            if kv.key.eq(key) {
                return Some(kv.value.to_string());
            }
        }
        None
    }

    pub fn new(key: String, value: String) -> KV {
        KV {
            key: key,
            value: value,
        }
    }
}

// KVFile 的定义
#[derive(Debug, Clone)]
pub struct KVFile {
    pub id: u64,
    pub label: KVs,
}

// 请求的参数与结果定义
#[derive(Debug, Clone)]
pub struct ListFileParams {
    pub ids: Vec<u64>,
    pub selectors: Selectors,
}

#[derive(Debug, Clone)]
pub struct ListFileResult {
    pub files: Vec<KVFile>,
}

#[derive(Debug, Clone)]
pub struct SetLabelParams {
    pub id: u64,
    pub label: HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct SetLabelResult {
    pub kvs: HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct RemoveFileParams {
    pub ids: Vec<u64>,
}

#[derive(Debug, Clone)]
pub struct RemoveFileResult {
    pub amount: usize,
}

#[derive(Debug, Clone)]
pub struct AddFileParams {
    pub label: HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct AddFileResult {
    pub id: u64,
    pub label: HashMap<String, String>,
}
