> Unified types for asynchronous TCP & Unix sockets.

The types provided by this crate allow for writing socket-type-agnostic network applications that treat UNIX sockets in the same way as IPv4 and IPv6.

This package is built using [async-std](https://github.com/async-rs/async-std) and is inspired by the [multisock](https://crates.io/crates/multisock) crate.

## Example

```rs
let address = SocketAddr::from_str("unix:/tmp/sock").await.unwrap(); // use unix socket
let address = SocketAddr::from_str("127.0.0.1:4445").await.unwrap(); // use tcp address
let listener = Listener::bind(&address).await.unwrap();

while let Some(stream) = listener.incoming().next().await {
    ...
}
```
