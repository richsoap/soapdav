use std::convert::Infallible;
use std::error::Error;
use std::net::SocketAddr;
use std::str::FromStr;
use std::sync::Arc;

use soapdav::adapter::storage::{
    ListSelectorSetResult, MockSelectorSetStorage, SelectorSet, SelectorSetStorage,
};
use soapdav::SimpleFileSystem;
use hyper;

use log::info;
use webdav_handler::body::Body;
use webdav_handler::{fakels, DavHandler};

#[derive(Clone)]
struct Server {
    dh: DavHandler,
}

impl Server {
    pub fn new() -> Self {
        let mut config = DavHandler::builder();
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
        let simplefs =
            SimpleFileSystem::new(&(Arc::new(selector_set_storage) as Arc<dyn SelectorSetStorage>));

        config = config.filesystem(Box::new(simplefs));
        config = config.locksystem(fakels::FakeLs::new());
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
    info!("before new server");

    let dav_server = Server::new();
    info!("after new Server");
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
    info!("before server started");

    let server = hyper::Server::try_bind(&addr)?
        .serve(make_service);
    info!("server started at {}", addr);
    let _ = server.await;
    Ok(())
}
