use futures::FutureExt;
use log::info;
use simplefs::staticfile::StaticFile;
use webdav_handler::fs::{DavDirEntry, DavFile, DavMetaData};
use xml::name;
use crate::core::fs::*;

use crate::adapter::storage::{KVFile, SelectorSet, KV};

#[derive(Debug, Clone)]
pub struct StaticDir {
    name: String,
    modified_time: std::time::SystemTime,
}

impl StaticDir {
    pub fn new(name: &String, modified_time: std::time::SystemTime) -> Self {
        StaticDir { name: name.to_string(), modified_time: modified_time }
    }
}

impl From<&SelectorSet> for StaticDir {
    fn from(value: &SelectorSet) -> Self {
        return StaticDir { name: value.name.clone(), modified_time: value.modified_time }
    }
}

impl From<&String> for StaticDir {
    fn from(name: &String) -> Self {
        return StaticDir { name: name.clone(), modified_time: std::time::SystemTime::now() }
    }
}

impl From<&KVFile> for StaticDir {
    fn from(value: &KVFile) -> Self {
        return StaticDir { 
            name: KV::find_value_default(&value.label, &String::from(TITLE), String::from("untitiled")), 
            // TODO: 使用真时间
            modified_time: std::time::SystemTime::now(),
        }
    }
}

impl DavDirEntry for StaticDir {
    fn name(&self) -> Vec<u8> {
        self.name.to_string().into_bytes()
    }

    fn metadata<'a>(&'a self) -> webdav_handler::fs::FsFuture<Box<dyn DavMetaData>> {
        async {
            Ok(Box::new(self.clone()) as Box<dyn DavMetaData>)
        }.boxed()
    }
}

impl DavMetaData for StaticDir {
    fn len(&self) -> u64 {
        12
    }

    fn modified(&self) -> webdav_handler::fs::FsResult<std::time::SystemTime> {
        Ok(self.modified_time)
    }

    fn is_dir(&self) -> bool {
        true
    }
    
}

impl DavFile for StaticDir {
    fn metadata<'a>(&'a mut self) -> webdav_handler::fs::FsFuture<Box<dyn DavMetaData>> {
        async {
            Ok(Box::new(self.clone()) as Box<dyn DavMetaData>)
        }.boxed()
    }

    fn write_buf<'a>(&'a mut self, buf: Box<dyn bytes::Buf + Send>) -> webdav_handler::fs::FsFuture<()> {
        async {Ok(())}.boxed()
    }

    fn write_bytes<'a>(&'a mut self, buf: bytes::Bytes) -> webdav_handler::fs::FsFuture<()> {
        async {Ok(())}.boxed()
    }

    fn read_bytes<'a>(&'a mut self, count: usize) -> webdav_handler::fs::FsFuture<bytes::Bytes> {
        async {
            Ok(bytes::Bytes::new())
        }.boxed()
    }

    fn seek<'a>(&'a mut self, pos: std::io::SeekFrom) -> webdav_handler::fs::FsFuture<u64> {
        async {
            Ok(0)
        }.boxed()
    }

    fn flush<'a>(&'a mut self) -> webdav_handler::fs::FsFuture<()> {
        async {Ok(())}.boxed()
    }
}