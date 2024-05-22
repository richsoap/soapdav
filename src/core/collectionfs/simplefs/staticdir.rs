use futures::FutureExt;
use webdav_handler::fs::{DavDirEntry, DavFile, DavMetaData};



#[derive(Debug, Clone)]
struct StaticDir {
    name: String,
    modified_time: std::time::SystemTime,
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
        0
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