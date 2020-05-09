use std::pin::Pin;
use async_std::task::{Context, Poll};
use async_std::stream;
use async_std::io;
use async_std::net;
#[cfg(unix)]
use async_std::os::unix::net as unix;
use crate::Stream;

#[derive(Debug)]
pub enum Incoming<'a> {
    Inet(net::Incoming<'a>),
    #[cfg(unix)]
    Unix(unix::Incoming<'a>)
}

impl<'a> From<net::Incoming<'a>> for Incoming<'a> {
    fn from(s: net::Incoming<'_>) -> Incoming {
        Incoming::Inet(s)
    }
}

#[cfg(unix)]
impl<'a> From<unix::Incoming<'a>>for Incoming<'a> {
    fn from(s: unix::Incoming<'_>) -> Incoming {
        Incoming::Unix(s)
    }
}

impl<'a> stream::Stream for Incoming<'a> {
    type Item = io::Result<Stream>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        match Pin::into_inner(self) {
            Incoming::Inet(ref mut s) => {
                Pin::new(s).poll_next(cx).map(|opt| opt.map(|res| res.map(Stream::Inet)))
            }
            #[cfg(unix)]
            Incoming::Unix(ref mut s) => {
                Pin::new(s).poll_next(cx).map(|opt| opt.map(|res| res.map(Stream::Unix)))
            }
        }
    }
}
