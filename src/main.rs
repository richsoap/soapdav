use std::convert::Infallible;
use std::error::Error;
use std::io::Write;
use std::net::SocketAddr;
use std::str::FromStr;
use std::sync::Arc;

use http::Response;
use hyper::{self, body};
use soapdav::adapter::storage::selector::mem::MemSelectorSetStorage;
use soapdav::adapter::storage::selector::MemFileKVFileStorage;
use soapdav::{AddFileParams, CollectionFS, DefineCollectionParams, DefineSelectorParams, RemoveCollectionParams, SimpleFileSystem};

use log::info;
use webdav_handler::body::Body;
use webdav_handler::{fakels, DavHandler};

#[derive(Clone)]
struct Server {
    dh: DavHandler,
    fs: SimpleFileSystem,
}

impl Server {
    pub fn new() -> Self {
        let selector_set_storage = Arc::new(MemSelectorSetStorage::new());
        let new_kv = Arc::new(MemFileKVFileStorage::new());
        let simplefs = SimpleFileSystem::new(selector_set_storage, new_kv.clone(), new_kv);
        let config = DavHandler::builder()
            .filesystem(Box::new(simplefs.clone()))
            .locksystem(fakels::FakeLs::new())
            .autoindex(true, None);
        Server {
            dh: config.build_handler(),
            fs: simplefs,
        }
    }

    async fn handle(
        &self,
        req: hyper::Request<hyper::Body>,
    ) -> Result<hyper::Response<Body>, Infallible> {
        match (req.method(), req.uri().path()) {
            (_, "/manage/add_file") => return self.add_file(req).await,
            (_, "/manage/define_collection") => return self.define_collection(req).await,
            (_, "/manage/remove_collection") => return self.remove_collection(req).await,
            (_, "/manage/define_selector") => return self.define_selector(req).await,
            (method, path) => {
                log::info!("receive dav request, method={}, path={}", method, path);
                return Ok(self.dh.handle(req).await);
            },
        }
    }

    async fn add_file(
        &self,
        req: hyper::Request<hyper::Body>,
    ) -> Result<hyper::Response<Body>, Infallible> {
        let whole_body = body::to_bytes(req.into_body()).await.unwrap();
        let str_body = std::str::from_utf8(&whole_body).unwrap();
        let params: AddFileParams = serde_json::from_str(str_body).unwrap();
        match self.fs.add_file(&params) {
            Ok(r) => Ok(Response::new(Body::from(
                serde_json::to_string(&r).unwrap(),
            ))),
            Err(_) => Ok(Response::new(Body::from(String::from("NotOk")))),
        }
    }

    async fn define_collection(
        &self,
        req: hyper::Request<hyper::Body>,
    ) -> Result<hyper::Response<Body>, Infallible> {
        let whole_body = body::to_bytes(req.into_body()).await.unwrap();
        let str_body = std::str::from_utf8(&whole_body).unwrap();
        let params: DefineCollectionParams = serde_json::from_str(str_body).unwrap();
        match self.fs.define_collection(&params) {
            Ok(r) => Ok(Response::new(Body::from(
                serde_json::to_string(&r).unwrap(),
            ))),
            Err(_) => Ok(Response::new(Body::from(String::from("NotOk")))),
        }
    }

    async fn remove_collection(
        &self,
        req: hyper::Request<hyper::Body>,
    ) -> Result<hyper::Response<Body>, Infallible> {
        let whole_body = body::to_bytes(req.into_body()).await.unwrap();
        let str_body = std::str::from_utf8(&whole_body).unwrap();
        let params: RemoveCollectionParams = serde_json::from_str(str_body).unwrap();
        match self.fs.remove_collection(&params) {
            Ok(r) => Ok(Response::new(Body::from(
                serde_json::to_string(&r).unwrap(),
            ))),
            Err(_) => Ok(Response::new(Body::from(String::from("NotOk")))),
        }
    }

    async fn define_selector(
        &self,
        req: hyper::Request<hyper::Body>,
    ) -> Result<hyper::Response<Body>, Infallible> {
        let whole_body = body::to_bytes(req.into_body()).await.unwrap();
        let str_body = std::str::from_utf8(&whole_body).unwrap();
        let params: DefineSelectorParams = serde_json::from_str(str_body).unwrap();
        match self.fs.define_selector(&params) {
            Ok(r) => Ok(Response::new(Body::from(
                serde_json::to_string(&r).unwrap(),
            ))),
            Err(_) => Ok(Response::new(Body::from(String::from("NotOk")))),
        }
    }
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn Error>> {
    env_logger::Builder::new()
        .format(|buf, record| {
            writeln!(
                buf,
                "{}:{} [{}] - {}",
                record.file().unwrap_or("unknown"),
                record.line().unwrap_or(0),
                record.level(),
                record.args()
            )
        })
        .filter(None, log::LevelFilter::Debug)
        .init();

    let dav_server = Server::new();
    let make_service = hyper::service::make_service_fn(|_| {
        let dav_server = dav_server.clone();
        async move {
            let func = move |req| {
                let dav_server = dav_server.clone();
                async move { dav_server.clone().handle(req).await }
            };
            Ok::<_, hyper::Error>(hyper::service::service_fn(func))
        }
    });

    let addr = format!("0.0.0.0:{}", 9876);
    let addr = SocketAddr::from_str(&addr)?;
    let server = hyper::Server::try_bind(&addr)?.serve(make_service);
    info!("server started at {}", addr);
    let _ = server.await;
    Ok(())
}
