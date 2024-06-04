use std::fs::File;

use thiserror::Error;
use webdav_handler::fs::DavFileSystem;

use crate::adapter::storage::{self, KVFileStorageError, SelectorSet, SelectorSetStorageError, SelectorStorageError, KV};

#[derive(Debug, Clone)]
pub struct DefineCollectionParams {
    pub selector_set: SelectorSet,
}

#[derive(Debug, Clone)]
pub struct DefineCollectionResult {
}

#[derive(Debug, Clone)]
pub struct RemoveCollectionParams {
    pub name: Vec<String>,
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
    fn add_file<'a>(&'a self, params: &'a AddFileParams) -> Result<AddFileResult, FilesystemError>;
    fn define_selector<'a >(&'a  self, params: &'a DefineSelectorParams) -> Result<DefineSelectorResult, FilesystemError>;
    fn define_collection<'a >(&'a self, params: &'a DefineCollectionParams) -> Result<DefineCollectionResult, FilesystemError>;
    fn remove_collection<'a >(&'a self, params: &'a RemoveCollectionParams) -> Result<RemoveCollectionResult, FilesystemError>;
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

impl From<SelectorSetStorageError> for FilesystemError {
    fn from(value: SelectorSetStorageError) -> Self {
        match value {
            SelectorSetStorageError::NotFound => FilesystemError::NotFound,
        }
    }
}