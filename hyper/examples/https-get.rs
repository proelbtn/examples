use tokio;
use hyper::Client;
use hyper::body::HttpBody as _;
use hyper_tls::HttpsConnector;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let https = HttpsConnector::new();
    let client = Client::builder()
        .build::<_, hyper::Body>(https);
    let uri = "https://httpbin.org/get".parse()?;
    let mut resp = client.get(uri).await?;
    let body = resp.body_mut().data().await;

    println!("Response: {:?}", body);

    Ok(())
}
