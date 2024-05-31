use std::fs::File;

use thiserror::Error;
use webdav_handler::fs::DavFileSystem;

use crate::adapter::storage::{self, KVFileStorageError, SelectorSet, SelectorStorageError, KV};

#[derive(Debug, Clone)]
pub struct DefineCollectionParams {
    selector_set: SelectorSet,
}

#[derive(Debug, Clone)]
pub struct DefineCollectionResult {
    id: u64,
}

#[derive(Debug, Clone)]
pub struct RemoveCollectionParams {
    name: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct RemoveCollectionResult {}

pub struct AddFileParams {
    pub kvs: Vec<KV>,
}

pub struct AddFileResult {

}

pub type DefineSelectorParams = storage::DefineSelectorParams;

pub struct DefineSelectorResult {}



pub trait CollectionFS:DavFileSystem {
    fn add_file(&mut self, params: &AddFileParams) -> Result<AddFileResult, FilesystemError>;
    fn define_selector(&mut self, params: &DefineSelectorParams) -> Result<DefineSelectorResult, FilesystemError>;
    fn define_collection(&mut self, params: &DefineCollectionParams) -> Result<DefineCollectionResult, FilesystemError>;
    fn remove_collection(&self, params: &RemoveCollectionParams) -> Result<RemoveCollectionResult, FilesystemError>;
}

#[derive(Debug, Clone, Error)]
pub enum FilesystemError {
    #[error("NotFound")]
    NotFound,
}

impl From<SelectorStorageError> for FilesystemError {
    fn from(value: SelectorStorageError) -> Self {
        match value {
            SelectorStorageError::NotFound => FilesystemError::NotFound,
        }
    }
}

impl From<KVFileStorageError> for FilesystemError {
    fn from(value: KVFileStorageError) -> Self {
        match value {
            KVFileStorageError::NotFound => FilesystemError::NotFound,
        }
    }
}