use async_std::io;
use async_std::net;
#[cfg(unix)]
use async_std::os::unix::net as unix;
use crate::Incoming;
use crate::SocketAddr;
use crate::Stream;

#[derive(Debug)]
pub enum Listener {
    Inet(net::TcpListener),
    #[cfg(unix)]
    Unix(unix::UnixListener)
}

impl From<net::TcpListener> for Listener {
    fn from(s: net::TcpListener) -> Listener {
        Listener::Inet(s)
    }
}

#[cfg(unix)]
impl From<unix::UnixListener> for Listener {
    fn from(s: unix::UnixListener) -> Listener {
        Listener::Unix(s)
    }
}

impl Listener {
    pub async fn bind(s: &SocketAddr) -> io::Result<Listener> {
        match s {
            SocketAddr::Inet(s) => net::TcpListener::bind(s).await.map(Listener::Inet),
            #[cfg(unix)]
            SocketAddr::Unix(s) => unix::UnixListener::bind(s).await.map(Listener::Unix)
        }
    }

    pub async fn accept(&self) -> io::Result<(Stream, SocketAddr)> {
        match self {
            Listener::Inet(l) => l.accept().await.map(|(s,e)| (s.into(), e.into())),
            #[cfg(unix)]
            Listener::Unix(l) => l.accept().await.map(|(s,e)| (s.into(), e.into()))
        }
    }

    pub fn incoming(&self) -> Incoming<'_> {
        match self {
            Listener::Inet(l) => Incoming::from(l.incoming()),
            #[cfg(unix)]
            Listener::Unix(l) => Incoming::from(l.incoming()),
        }
    }
}
