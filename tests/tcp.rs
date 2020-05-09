use async_std::prelude::*;
use async_std::task;
use async_uninet::{Listener, SocketAddr, Stream};

#[async_std::test]
async fn starts_inet_server() {
    let address = SocketAddr::from_str("127.0.0.1:4445").unwrap();
    let listener = Listener::bind(&address).await.unwrap();

    task::spawn(async move {
        Stream::connect(&address).await.unwrap();
    });

    let mut result = false;
    while let Some(stream) = listener.incoming().next().await {
        let mut stream = stream.unwrap();
        stream.write(b"foo").await.unwrap();
        result = true;
        break;
    }
    assert!(result);
}
