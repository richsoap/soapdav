use std::collections::HashMap;
use super::{SelectorStorage, Selectors};
use thiserror::Error;

// 定义 KVFileStorage 错误, 用于处理可能出现的错误情况
#[derive(Error, Debug)]
pub enum KVFileStorageError {
}

// KVFileStorage trait
pub trait KVFileStorage: SelectorStorage {
    fn list_file(&self, params: ListFileParams) -> Result<ListFileResult, KVFileStorageError>;

    fn add_file(&self, params: AddFileParams) -> Result<AddFileResult, KVFileStorageError>;

    fn remove_file (&self, params: RemoveFileParams) -> Result<RemoveFileResult, KVFileStorageError>;

    fn set_label(&self, params: SetLabelParams) -> Result<SetLabelResult, KVFileStorageError>;
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
}

// KVFile 的定义
#[derive(Debug, Clone)]
pub struct KVFile {
    id: u64,
    label: KVs,
}

// 请求的参数与结果定义
#[derive(Debug, Clone)]
pub struct ListFileParams {
    ids: Vec<u64>,
    selector: Selectors,
}

#[derive(Debug, Clone)]
pub struct ListFileResult {
    files: Vec<KVFile>,
}

#[derive(Debug, Clone)]
pub struct SetLabelParams {
    id: u64,
    label: HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct SetLabelResult {

}

#[derive(Debug, Clone)]
pub struct RemoveFileParams {
    id: Vec<u64>,
}

#[derive(Debug, Clone)]
pub struct RemoveFileResult {

}

#[derive(Debug, Clone)]
pub struct AddFileParams {
    label: HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct AddFileResult {
    id: u64,
    label: HashMap<String, String>,
}