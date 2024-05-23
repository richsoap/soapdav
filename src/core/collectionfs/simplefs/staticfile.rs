use std::cmp::{max, min};

use bytes::Bytes;
use futures::FutureExt;
use webdav_handler::fs::{DavFile, DavMetaData};



#[derive(Debug, Clone)]
struct StaticFile {
    name: String,
    modified_time: std::time::SystemTime,
    body: Bytes,
    offset: usize,
}

impl DavMetaData for StaticFile {
    fn len(&self) -> u64 {
        self.body.len() as u64
    }

    fn modified(&self) -> webdav_handler::fs::FsResult<std::time::SystemTime> {
        Ok(self.modified_time)
    }

    fn is_dir(&self) -> bool {
        false
    }
}

impl DavFile for StaticFile {
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
        async move {
            let end = min(self.offset + count, self.body.len());
            let result = self.body.slice(self.offset..end);
            self.offset = end;
            Ok(result)
        }.boxed()
    }

    fn seek<'a>(&'a mut self, pos: std::io::SeekFrom) -> webdav_handler::fs::FsFuture<u64> {
        async move {
            self.offset = match pos {
                std::io::SeekFrom::Start(v) => v as usize,
                std::io::SeekFrom::End(v) => max(0, self.body.len() - v as usize),
                std::io::SeekFrom::Current(v) => min(self.offset + v as usize, self.body.len()),
            };
            Ok(self.offset as u64)
        }.boxed()
    }

    fn flush<'a>(&'a mut self) -> webdav_handler::fs::FsFuture<()> {
        async {Ok(())}.boxed()
    }
}