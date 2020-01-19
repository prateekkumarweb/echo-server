use hyper::{body::HttpBody, Body, Client, Method, Request, Uri};
use hyper_tls::HttpsConnector;
use tokio::io::{stdout, AsyncWriteExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let client = Client::new();
    let uri = "http://httpbin.org/ip".parse()?;
    let mut resp = client.get(uri).await?;
    println!("Response: {}", resp.status());
    while let Some(chunk) = resp.body_mut().data().await {
        stdout().write_all(&chunk?).await?;
    }

    let req = Request::builder()
        .method(Method::POST)
        .uri("http://httpbin.org/post")
        .header("content-type", "application/json")
        .body(Body::from(r#"{"library": "hyper"}"#))?;
    let client = Client::new();
    let mut resp = client.request(req).await?;
    println!("Response: {}", resp.status());
    while let Some(chunk) = resp.body_mut().data().await {
        stdout().write_all(&chunk?).await?;
    }

    let client = Client::new();
    let ip_fut = async {
        let resp = client
            .get(Uri::from_static("http://httpbin.org/ip"))
            .await?;
        hyper::body::to_bytes(resp.into_body()).await
    };
    let headers_fut = async {
        let resp = client
            .get(Uri::from_static("http://httpbin.org/headers"))
            .await?;
        hyper::body::to_bytes(resp.into_body()).await
    };
    let (ip, headers) = futures::try_join!(ip_fut, headers_fut)?;
    println!("IP: {:?}\nHeaders: {:?}", ip, headers);

    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);
    let uri = "https://httpbin.org/ip".parse()?;
    let mut resp = client.get(uri).await?;
    println!("Response: {}", resp.status());
    while let Some(chunk) = resp.body_mut().data().await {
        stdout().write_all(&chunk?).await?;
    }
    Ok(())
}
