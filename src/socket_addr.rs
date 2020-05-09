use std::fmt;
use std::str::FromStr;
use std::path::{Path, PathBuf};
use async_std::net::ToSocketAddrs;
use async_std::net;
#[cfg(unix)]
use async_std::os::unix::net as unix;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SocketAddr {
    Inet(net::SocketAddr),
    #[cfg(unix)]
    Unix(PathBuf)
}

impl From<net::SocketAddr> for SocketAddr {
    fn from(s: net::SocketAddr) -> SocketAddr {
        SocketAddr::Inet(s)
    }
}

#[cfg(unix)]
impl From<unix::SocketAddr> for SocketAddr {
    fn from(s: unix::SocketAddr) -> SocketAddr {
        SocketAddr::Unix(match s.as_pathname() {
            None => Path::new(".sock").to_path_buf(),
            Some(p) => p.to_path_buf()
        })
    }
}

impl fmt::Display for SocketAddr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SocketAddr::Inet(n) => write!(f, "{}", n),
            #[cfg(unix)]
            SocketAddr::Unix(n) => write!(f, "unix:{}", n.to_string_lossy())
        }
    }
}

impl FromStr for SocketAddr {
    type Err = net::AddrParseError;

    #[cfg(unix)]
    fn from_str(s: &str) -> Result<SocketAddr, net::AddrParseError> {
        if s.starts_with("unix:") {
            Ok(SocketAddr::Unix(Path::new(s.trim_start_matches("unix:")).to_path_buf()))
        } else {
            s.parse().map(SocketAddr::Inet)
        }
    }

    #[cfg(not(unix))]
    fn from_str(s: &str) -> Result<SocketAddr, net::AddrParseError> {
        s.parse().map(SocketAddr::Inet)
    }
}

impl SocketAddr {

    pub async fn from_str<S: Into<String>>(txt: S) -> Result<Self, ()> {
        let txt = txt.into();
        if txt.starts_with("unix:") {
            let addr = match txt.parse::<Self>() {
                Ok(addr) => addr,
                Err(_) => return Err(()),
            };
            Ok(Self::from(addr))
        } else {
            let addr = match txt.to_socket_addrs().await {
                Err(_) => return Err(()),
                Ok(mut addr) => match addr.next() {
                    Some(addr) => addr,
                    None => return Err(()),
                },
            };
            Ok(Self::from(addr))
        }
    }

    pub fn is_unix(&self) -> bool {
        match self {
            #[cfg(unix)]
            SocketAddr::Unix(_) => true,
            _ => false,
        }
    }

    pub fn is_inet(&self) -> bool {
        !self.is_unix()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[async_std::test]
    async fn creates_from_inet() {
        let ip4 = SocketAddr::from_str("127.0.0.1:10").await;
        let ip6 = SocketAddr::from_str("[::20]:10").await;
        let url = SocketAddr::from_str("jsonplaceholder.typicode.com:80").await;
        let invalid = SocketAddr::from_str("foo").await;
        assert!(ip4.is_ok());
        assert!(ip6.is_ok());
        assert!(url.is_ok());
        assert!(invalid.is_err());
        assert_eq!(ip4.unwrap().to_string(), "127.0.0.1:10");
        assert_eq!(ip6.unwrap().to_string(), "[::0.0.0.32]:10");
        assert!(url.unwrap().to_string().starts_with("104.")); // 104.28.28.222:80, 104.28.29.222:80
    }

    #[async_std::test]
    #[cfg(unix)]
    async fn creates_from_unix() {
        let unix = SocketAddr::from_str("unix:/tmp/sock").await;
        let invalid = SocketAddr::from_str("/tmp/sock").await;
        assert!(unix.is_ok());
        assert!(invalid.is_err());
    }
}
