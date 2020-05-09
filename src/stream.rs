use std::pin::Pin;
use async_std::io;
use async_std::net;
use async_std::prelude::*;
use async_std::task::{Context, Poll};
#[cfg(unix)]
use async_std::os::unix::net as unix;
use crate::SocketAddr;

#[derive(Debug)]
pub enum Stream {
    Inet(net::TcpStream),
    #[cfg(unix)]
    Unix(unix::UnixStream)
}

impl From<net::TcpStream> for Stream {
    fn from(s: net::TcpStream) -> Stream {
        Stream::Inet(s)
    }
}

#[cfg(unix)]
impl From<unix::UnixStream> for Stream {
    fn from(s: unix::UnixStream) -> Stream {
        Stream::Unix(s)
    }
}

impl Stream {
    pub async fn connect(s: &SocketAddr) -> io::Result<Self> {
        match s {
            SocketAddr::Inet(s) => net::TcpStream::connect(s).await.map(Stream::Inet),
            #[cfg(unix)]
            SocketAddr::Unix(s) => unix::UnixStream::connect(s).await.map(Stream::Unix),
        }
    }

    pub fn local_addr(&self) -> io::Result<SocketAddr> {
        match self {
            Stream::Inet(s) => s.local_addr().map(SocketAddr::Inet),
            #[cfg(unix)]
            Stream::Unix(s) => s.local_addr().map(|e| e.into())
        }
    }

    pub fn peer_addr(&self) -> io::Result<SocketAddr> {
        match self {
            Stream::Inet(s) => s.peer_addr().map(SocketAddr::Inet),
            #[cfg(unix)]
            Stream::Unix(s) => s.peer_addr().map(|e| e.into())
        }
    }

    pub fn shutdown(&self, t: net::Shutdown) -> io::Result<()> {
        match self {
            Stream::Inet(s) => s.shutdown(t),
            #[cfg(unix)]
            Stream::Unix(s) => s.shutdown(t)
        }
    }
}

impl io::Read for &Stream {

    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut [u8],
    ) -> Poll<io::Result<usize>> {
        match Pin::into_inner(self) {
            Stream::Inet(s) => {
                (&mut (&*s)).read(buf);
                Pin::new(&mut &*s).poll_read(cx, buf)
            },
            #[cfg(unix)]
            Stream::Unix(s) => {
                (&mut (&*s)).read(buf);
                Pin::new(&mut &*s).poll_read(cx, buf)
            },
        }
    }
}

impl io::Write for &Stream {

    fn poll_write(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<io::Result<usize>> {
        match Pin::into_inner(self) {
            Stream::Inet(s) => {
                (&mut (&*s)).write(buf);
                Pin::new(&mut &*s).poll_write(cx, buf)
            },
            #[cfg(unix)]
            Stream::Unix(s) => {
                (&mut (&*s)).write(buf);
                Pin::new(&mut &*s).poll_write(cx, buf)
            },
        }
    }

    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        match Pin::into_inner(self) {
            Stream::Inet(s) => {
                (&mut (&*s)).flush();
                Pin::new(&mut &*s).poll_flush(cx)
            },
            #[cfg(unix)]
            Stream::Unix(s) => {
                (&mut (&*s)).flush();
                Pin::new(&mut &*s).poll_flush(cx)
            },
        }
    }

    fn poll_close(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        match Pin::into_inner(self) {
            Stream::Inet(s) => {
                (&mut (&*s)).flush();
                Pin::new(&mut &*s).poll_close(cx)
            },
            #[cfg(unix)]
            Stream::Unix(s) => {
                (&mut (&*s)).flush();
                Pin::new(&mut &*s).poll_close(cx)
            },
        }
    }
}

impl io::Read for Stream {
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut [u8],
    ) -> Poll<io::Result<usize>> {
        Pin::new(&mut &*self).poll_read(cx, buf)
    }
}

impl io::Write for Stream {
    fn poll_write(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<io::Result<usize>> {
        Pin::new(&mut &*self).poll_write(cx, buf)
    }

    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        Pin::new(&mut &*self).poll_flush(cx)
    }

    fn poll_close(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        Pin::new(&mut &*self).poll_close(cx)
    }
}
