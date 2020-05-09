use std::fs::remove_file;
use async_std::prelude::*;
use async_std::task;
use async_uninet::{Listener, SocketAddr};
#[cfg(unix)]
use async_uninet::{Stream};

#[async_std::test]
#[cfg(unix)]
async fn starts_unix_server() {
    match remove_file("./target/tmp.sock") {
        Ok(_) => (),
        Err(_) => (),
    };

    let address = SocketAddr::from_str("unix:./target/tmp.sock").await.unwrap();
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
