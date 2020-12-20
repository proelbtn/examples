use anyhow::{Error, Result};
use bytes::Bytes;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::UdpSocket;
use tracing::{Level, error, info, trace, span, field};


async fn ping_pong(sock: Arc<UdpSocket>, buf: Bytes, addr: SocketAddr) -> Result<()> {
    sock.send_to(&buf, &addr).await?;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let sock = Arc::new(UdpSocket::bind("[::]:8080".parse::<SocketAddr>().unwrap()).await?);

    let mut buf = vec![0; 2048];
    loop {
        let sock = sock.clone();

        trace!("waiting new connection");
        let (len, addr) = sock.recv_from(&mut buf).await?;
        let buf = Bytes::copy_from_slice(&buf[..len]);

        tokio::spawn(async move {
            let span = span!(Level::INFO, "connection_handler", from = field::display(addr));
            let _enter = span.enter();
            trace!("recv data: {:?}", buf);

            match ping_pong(sock, buf, addr).await {
                Err(err) => error!("error occured: {:?}", err),
                _ => (),
            };
        });
    }

    Ok(())
}
