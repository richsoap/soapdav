use webdav_handler::fs::DavFileSystem;

use crate::adapter::storage::SelectorSet;

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

pub trait CollectionFS:DavFileSystem {
    fn define_collection(&self, params: DefineCollectionParams) -> Result<DefineCollectionResult, FilesystemError>;
    fn remove_collection(&self, params: RemoveCollectionParams) -> Result<RemoveCollectionResult, FilesystemError>;
}

#[derive(Debug, Clone)]
pub enum FilesystemError {
    // 这里定义 Filesystem 相关的错误
}