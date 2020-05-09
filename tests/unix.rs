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

    let address = SocketAddr::from_str("unix:./target/tmp.sock").unwrap();
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
