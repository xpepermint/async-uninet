use async_std::prelude::*;
use async_std::task;
use async_uninet::{Listener, SocketAddr, Stream};

#[async_std::test]
async fn starts_inet_server() {
    let address = SocketAddr::from_str("127.0.0.1:4445").await.unwrap();
    let listener = Listener::bind(&address).await.unwrap();

    task::spawn(async move {
        Stream::connect(&address).await.unwrap();
    });

    let mut result = false;
    while let Some(_) = listener.incoming().next().await {
        result = true;
        break;
    }
    assert!(result);
}

#[async_std::test]
async fn performs_http_request() {
    let address = SocketAddr::from_str("google.com:80").await.unwrap();
    let mut stream = Stream::connect(&address).await.unwrap();
    stream.write_all(b"GET / HTTP/1.0\r\n\r\n").await.unwrap();

    let mut res = Vec::new();
    let length = stream.read_to_end(&mut res).await.unwrap();
    
    assert!(length > 0);
}
