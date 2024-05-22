use std::sync::Arc;
use std::time::SystemTime;
use std::io::{Read, Write, Seek, Result as IoResult, Error as IoError, ErrorKind as IoErrorKind};
use std::fs::{File, Metadata, OpenOptions};
use std::path::Path;

use webdav_handler::fs::{DavDirEntry, DavFile, DavFileSystem, BoxCloneFs};

use crate::adapter::storage::SelectorSetStorage;
use crate::core::collectionfs::CollectionFS;

#[derive(Debug, Clone)]
pub struct SimpleFileSystem {
    selector_set_storage: Arc<dyn SelectorSetStorage>,
    // 这里需要根据实际情况定义 CollectionFileSystem 的字段
}

impl DavFileSystem for SimpleFileSystem {
    fn open<'a>(&'a self, path: &'a webdav_handler::davpath::DavPath, options: webdav_handler::fs::OpenOptions) -> webdav_handler::fs::FsFuture<Box<dyn DavFile>> {
        todo!()
    }

    fn read_dir<'a>(
        &'a self,
        path: &'a webdav_handler::davpath::DavPath,
        meta: webdav_handler::fs::ReadDirMeta,
    ) -> webdav_handler::fs::FsFuture<webdav_handler::fs::FsStream<Box<dyn DavDirEntry>>> {
        todo!()
    }

    fn metadata<'a>(&'a self, path: &'a webdav_handler::davpath::DavPath) -> webdav_handler::fs::FsFuture<Box<dyn webdav_handler::fs::DavMetaData>> {
        todo!()
    }
}

impl CollectionFS for SimpleFileSystem {
    fn define_collection(&self, params: crate::core::collectionfs::DefineCollectionParams) -> Result<crate::core::collectionfs::DefineCollectionResult, crate::core::collectionfs::FilesystemError> {
        todo!()
    }

    fn remove_collection(&self, params: crate::core::collectionfs::RemoveCollectionParams) -> Result<crate::core::collectionfs::RemoveCollectionResult, crate::core::collectionfs::FilesystemError> {
        todo!()
    }
}
