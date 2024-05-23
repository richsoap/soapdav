//
//  Sample application.
//
//  Listens on localhost:4918, plain http, no ssl.
//  Connect to http://localhost:4918/
//

use std::convert::Infallible;
use std::error::Error;
use std::net::SocketAddr;
use std::str::FromStr;
use std::sync::Arc;

use hello_rust::adapter::storage::{
    ListSelectorSetResult, MockSelectorSetStorage, SelectorSet, SelectorSetStorage,
};
use hello_rust::SimpleFileSystem;
use hyper;

use webdav_handler::{body::Body, time::UtcOffset};
use webdav_handler::{fakels, localfs, memfs, memls, DavConfig, DavHandler};

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

    let server = hyper::Server::try_bind(&addr)?
        .serve(make_service);

    let _ = server.await;
    Ok(())
}
