use std::convert::Infallible;
use std::net::SocketAddr;

use tokio;
use hyper::{
    Method, Body, Request, Response, Server, StatusCode,
    service::{service_fn, make_service_fn},
};
use hyper::body::HttpBody as _;

async fn handler(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let mut response = Response::new(Body::empty());

    match (req.method(), req.uri().path()) {
        (&Method::GET, "/") => {
            *response.body_mut() = Body::from("Try POSTing data to /echo");
        },
        (&Method::POST, "/echo") => {
            *response.body_mut() = req.into_body();
        },
        _ => {
            *response.status_mut() = StatusCode::NOT_FOUND;
        }
    }

    Ok(response)
}

async fn signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("failed to install CTRL+C signal handler");
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));

    let service = make_service_fn(|_conn| async {
        Ok::<_, Infallible>(service_fn(handler))
    });

    let server = Server::bind(&addr).serve(service);
    let server = server.with_graceful_shutdown(signal());

    match server.await {
        Ok(_) => Ok(()),
        Err(e) => Err(anyhow::Error::new(e)),
    }
}
