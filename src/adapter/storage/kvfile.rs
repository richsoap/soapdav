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
    fn list_file<'a>(
        &'a self,
        params: &'a ListFileParams,
    ) -> Result<ListFileResult, KVFileStorageError>;

    fn add_file<'a>(
        &'a self,
        params: &'a AddFileParams,
    ) -> Result<AddFileResult, KVFileStorageError>;

    fn remove_file<'a>(
        &'a self,
        params: &'a RemoveFileParams,
    ) -> Result<RemoveFileResult, KVFileStorageError>;

    fn set_label<'a>(
        &'a self,
        params: &'a SetLabelParams,
    ) -> Result<SetLabelResult, KVFileStorageError>;
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

    pub fn from_pair((k, v): (&String, &String)) -> KV {
        KV::new(k.clone(), v.clone())
    }

    pub fn to_pair(&self) -> (String, String) {
        (self.key.clone(), self.value.clone())
    }

    pub fn to_hash_map(kvs: &KVs) -> HashMap<String, String> {
        kvs.iter().map(KV::to_pair).collect()
    }

    pub fn from_hash_map(kvs: HashMap<String, String>) -> KVs {
        kvs.iter().map(KV::from_pair).collect()
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
