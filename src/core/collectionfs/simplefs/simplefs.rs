use std::env::split_paths;
use std::fs::{File, Metadata, OpenOptions};
use std::io::{Error as IoError, ErrorKind as IoErrorKind, Read, Result as IoResult, Seek, Write};
use std::path::Path;
use std::sync::Arc;
use std::time::SystemTime;

use futures::stream::iter;
use futures::FutureExt;
use percent_encoding::percent_decode;
use webdav_handler::davpath::DavPath;
use webdav_handler::fs::{
    BoxCloneFs, DavDirEntry, DavFile, DavFileSystem, FsError, FsResult, FsStream, ReadDirMeta,
};

use crate::adapter::storage::{ListSelectorSetParams, SelectorSetStorage};
use crate::core::collectionfs::CollectionFS;

use super::staticdir::StaticDir;

#[derive(Debug, Clone)]
pub struct SimpleFileSystem {
    selector_set_storage: Arc<dyn SelectorSetStorage>,
    // 这里需要根据实际情况定义 CollectionFileSystem 的字段
}

impl SimpleFileSystem {
    fn split_path(path: &DavPath) -> Result<Vec<String>, std::str::Utf8Error> {
        match percent_decode(path.as_bytes()).decode_utf8() {
            Ok(cs) => Ok(cs.into_owned().split('/').map(|s| s.to_string()).collect()),
            Err(e) => Err(e),
        }
    }
    fn read_dir_stream<'a>(
        &'a self,
        paths: &Vec<String>,
        meta: webdav_handler::fs::ReadDirMeta,
    ) -> FsResult<FsStream<Box<dyn DavDirEntry>>> {
        self.read_root_dir_stream(meta)
    }

    fn read_root_dir_stream<'a>(
        &'a self,
        meta: ReadDirMeta,
    ) -> FsResult<FsStream<Box<dyn DavDirEntry>>> {
        let selector_sets = self
            .selector_set_storage
            .list_selector_set(ListSelectorSetParams { name: &vec![] });
        if selector_sets.is_err() {}
        match selector_sets {
            Err(_) => Err(FsError::GeneralFailure),
            Ok(result) => {
                let dirs: Vec<Box<dyn DavDirEntry>> = result
                    .selector_set
                    .iter()
                    .map(StaticDir::from)
                    .map(|x| Box::new(x) as Box<dyn DavDirEntry>)
                    .collect();
                Ok(Box::pin(iter(dirs)))
            }
        }
    }
}

impl DavFileSystem for SimpleFileSystem {
    fn open<'a>(
        &'a self,
        path: &'a webdav_handler::davpath::DavPath,
        options: webdav_handler::fs::OpenOptions,
    ) -> webdav_handler::fs::FsFuture<Box<dyn DavFile>> {
        todo!()
    }

    fn read_dir<'a>(
        &'a self,
        path: &'a webdav_handler::davpath::DavPath,
        meta: webdav_handler::fs::ReadDirMeta,
    ) -> webdav_handler::fs::FsFuture<webdav_handler::fs::FsStream<Box<dyn DavDirEntry>>> {
        async move {
            match SimpleFileSystem::split_path(path) {
                Ok(paths) => self.read_dir_stream(&paths, meta),
                Err(_) => Err(FsError::NotFound),
            }
        }
        .boxed()
    }

    fn metadata<'a>(
        &'a self,
        path: &'a webdav_handler::davpath::DavPath,
    ) -> webdav_handler::fs::FsFuture<Box<dyn webdav_handler::fs::DavMetaData>> {
        todo!()
    }
}

impl CollectionFS for SimpleFileSystem {
    fn define_collection(
        &self,
        params: crate::core::collectionfs::DefineCollectionParams,
    ) -> Result<
        crate::core::collectionfs::DefineCollectionResult,
        crate::core::collectionfs::FilesystemError,
    > {
        todo!()
    }

    fn remove_collection(
        &self,
        params: crate::core::collectionfs::RemoveCollectionParams,
    ) -> Result<
        crate::core::collectionfs::RemoveCollectionResult,
        crate::core::collectionfs::FilesystemError,
    > {
        todo!()
    }
}
