use std::convert::Infallible;
use std::error::Error;
use std::io::Write;
use std::net::SocketAddr;
use std::str::FromStr;
use std::sync::Arc;
use std::time::SystemTime;

use hyper;
use soapdav::adapter::storage::{
    ListSelectorSetResult, MockSelectorSetStorage, SelectorSet, SelectorSetStorage,
};
use soapdav::SimpleFileSystem;

use log::{info, warn};
use webdav_handler::body::Body;
use webdav_handler::{fakels, DavHandler};

#[derive(Clone)]
struct Server {
    dh: DavHandler,
}

impl Server {
    pub fn new() -> Self {
        let mut selector_set_storage = MockSelectorSetStorage::new();
        {
            selector_set_storage
                .expect_list_selector_set()
                .returning(|_| {
                    Ok(ListSelectorSetResult {
                        selector_set: vec![SelectorSet::new(&String::from("a"))],
                    })
                });
        }
        let simplefs = SimpleFileSystem {
            selector_set_storage: (Arc::new(selector_set_storage) as Arc<dyn SelectorSetStorage>) ,
            selector_storage: todo!(),
        };
        let config = DavHandler::builder()
            .filesystem(Box::new(simplefs))
            .locksystem(fakels::FakeLs::new())
            .autoindex(true, None);
        Server {
            dh: config.build_handler(),
        }
    }

    async fn handle(
        &self,
        req: hyper::Request<hyper::Body>,
    ) -> Result<hyper::Response<Body>, Infallible> {
        Ok(self.dh.handle(req).await)
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
