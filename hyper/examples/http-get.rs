use tokio;
use hyper::Client;
use hyper::body::HttpBody as _;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Client::new();
    let uri = "http://httpbin.org/get".parse()?;
    let mut resp = cli.get(uri).await?;
    let body = resp.body_mut().data().await;

    println!("Response: {:?}", body);

    Ok(())
}
