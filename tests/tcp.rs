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
