use std::sync::Arc;

use webdav_handler::fs::{DavDirEntry, DavMetaData};
use futures::FutureExt;

use super::simplefs::SimpleFileSystem;



// 最顶层的文件，用于提供Collection选择入口
#[derive(Debug, Clone)]
struct CollectionSet {
    root: Arc<SimpleFileSystem>,
}


impl DavDirEntry for CollectionSet {
    fn name(&self) -> Vec<u8> {
        const ROOT_NAME: &str = "root";
        ROOT_NAME.as_bytes().to_vec()
    }
    
    fn metadata<'a>(&'a self) -> webdav_handler::fs::FsFuture<Box<dyn DavMetaData>> {
        async {
            let meta_child = CollectionSet{
                root: self.root.clone(),
            };
            Ok(Box::new(meta_child) as Box<dyn DavMetaData>)
        }.boxed()
    }
    
}

impl DavMetaData for CollectionSet {
    fn len(&self) -> u64 {
        0
    }

    fn modified(&self) -> webdav_handler::fs::FsResult<std::time::SystemTime> {
        Ok(std::time::SystemTime::now())
    }

    fn is_dir(&self) -> bool {
        true
    }
}