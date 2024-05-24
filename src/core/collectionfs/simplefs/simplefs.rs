use std::collections::VecDeque;
use std::sync::Arc;
use std::time::SystemTime;

use futures::stream::iter;
use futures::FutureExt;
use log::info;
use mockall::predicate::path;
use percent_encoding::percent_decode;
use webdav_handler::davpath::DavPath;
use webdav_handler::fs::{
    DavDirEntry, DavFile, DavFileSystem, DavMetaData, FsError, FsResult, FsStream, ReadDirMeta,
};

use crate::adapter::storage::{
    ListSelectorSetParams, SelectorSet, SelectorSetStorage, SelectorStorage,
};
use crate::core::collectionfs::CollectionFS;

use super::staticdir::StaticDir;
use super::staticfile::StaticFile;

#[derive(Debug, Clone)]
pub struct SimpleFileSystem {
    pub selector_set_storage: Arc<dyn SelectorSetStorage>,
    pub selector_storage: Arc<dyn SelectorStorage>,
    // 这里需要根据实际情况定义 CollectionFileSystem 的字段
}

impl SimpleFileSystem {
    fn split_path(path: &DavPath) -> Result<Vec<String>, std::str::Utf8Error> {
        match percent_decode(path.as_bytes()).decode_utf8() {
            Ok(cs) => Ok(cs
                .into_owned()
                .split('/')
                .map(|s| s.to_string())
                .filter(|x| !x.is_empty())
                .collect()),
            Err(e) => Err(e),
        }
    }

    fn read_dir_stream<'a>(
        &'a self,
        paths: &Vec<String>,
        meta: webdav_handler::fs::ReadDirMeta,
    ) -> FsResult<FsStream<Box<dyn DavDirEntry>>> {
        info!("path={:?}", paths);
        let mut tokens = VecDeque::from(paths.clone());
        // 根目录
        if tokens.is_empty() {
            return self.read_root_dir_stream(meta);
        }
        // 构造筛选器组
        let mut selector_set = match self
            .selector_set_storage
            .get_selector_set_by_name(&tokens.pop_front().unwrap())
        {
            Ok(v) => v,
            Err(_e) => return Err(FsError::NotFound),
        };
        // 将路径中的参数逐个填到selector中
        while !tokens.is_empty() && !selector_set.is_full() {
            let selector_value = tokens.pop_front().unwrap();
            // TODO: 带点的都是特殊说明文件，做特殊处理
            if selector_value.starts_with('.') {
            } else {
                selector_set.add_required_value(selector_value);
            }
        }
        // 筛选器还没有满，找到下一个筛选项，并将可选结果以目录的形式返回
        if !selector_set.is_full() {
            return self.read_selecting_dir_stream(selector_set, meta)
        }
        // 筛选器已填满，说明可以执行筛选操作
        let selectors = match self
            .selector_storage
            .list_selector_for_selector_set(selector_set)
        {
            Ok(v) => v.selectors,
            Err(e) => return Err(FsError::NotFound),
        };
        Err(FsError::NotFound)
    }

    fn read_root_dir_stream<'a>(
        &'a self,
        meta: ReadDirMeta,
    ) -> FsResult<FsStream<Box<dyn DavDirEntry>>> {
        let selector_sets = self
            .selector_set_storage
            .list_selector_set(ListSelectorSetParams { name: &vec![] });
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

    fn read_selecting_dir_stream<'a>(
        &'a self,
        selector_set: SelectorSet,
        meta: ReadDirMeta,
    ) -> FsResult<FsStream<Box<dyn DavDirEntry>>> {
        let next_selector = selector_set.get_next_required_selector().unwrap();
        let next_selector = match self
            .selector_storage
            .get_selector_by_key(next_selector.get_key())
        {
            Ok(v) => v,
            Err(_) => return Err(FsError::NotFound),
        };
        let mut dirs: Vec<Box<dyn DavDirEntry>> = next_selector
            .value
            .iter()
            .map(StaticDir::from)
            .map(|x| Box::new(x) as Box<dyn DavDirEntry>)
            .collect();
        dirs.push(Box::new(StaticFile::new(next_selector.key, None, None)) as Box<dyn DavDirEntry>);
        Ok(Box::pin(iter(dirs)))
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
        async {
            let meta = StaticDir::new(&String::from("root"), SystemTime::now());
            Ok(Box::new(meta) as Box<dyn DavMetaData>)
        }
        .boxed()
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
