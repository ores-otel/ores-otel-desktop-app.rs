#![forbid(unsafe_code)]

use std::io::{Read, Write};
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, TcpStream};
use std::time::Duration;

use crate::state::DesktopState;

const PROBE_TIMEOUT: Duration = Duration::from_millis(500);

/// Resolve only literal loopback HTTP origins. Never resolve arbitrary hostnames
/// or make unexpected outbound requests from the native desktop process.
fn loopback_address(endpoint: &str) -> Option<SocketAddr> {
    let authority = endpoint.strip_prefix("http://")?;
    let authority = authority.strip_suffix('/').unwrap_or(authority);

    let (ip, suffix) = if let Some(suffix) = authority.strip_prefix("127.0.0.1") {
        (IpAddr::V4(Ipv4Addr::LOCALHOST), suffix)
    } else if let Some(suffix) = authority.strip_prefix("localhost") {
        (IpAddr::V4(Ipv4Addr::LOCALHOST), suffix)
    } else if let Some(suffix) = authority.strip_prefix("[::1]") {
        (IpAddr::V6(Ipv6Addr::LOCALHOST), suffix)
    } else {
        return None;
    };
    let port = if suffix.is_empty() {
        80
    } else {
        suffix.strip_prefix(':')?.parse::<u16>().ok().filter(|p| *p != 0)?
    };
    Some(SocketAddr::new(ip, port))
}

/// Non-invasive read-only local health probe. Non-loopback and HTTPS targets
/// remain disconnected until a TLS-capable, authenticated client is integrated.
pub fn probe(endpoint: &str) -> DesktopState {
    let connected = loopback_address(endpoint)
        .and_then(|addr| {
            let mut stream = TcpStream::connect_timeout(&addr, PROBE_TIMEOUT).ok()?;
            stream.set_read_timeout(Some(PROBE_TIMEOUT)).ok()?;
            stream.set_write_timeout(Some(PROBE_TIMEOUT)).ok()?;
            stream
                .write_all(b"GET /health HTTP/1.1\r\nHost: localhost\r\nConnection: close\r\n\r\n")
                .ok()?;
            let mut status = [0u8; 13];
            stream.read_exact(&mut status).ok()?;
            Some(status == *b"HTTP/1.1 200 " || status == *b"HTTP/1.0 200 ")
        })
        .unwrap_or(false);
    DesktopState {
        connected,
        endpoint: endpoint.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::TcpListener;
    use std::thread;
    use std::time::Instant;

    #[test]
    fn permits_only_explicit_loopback_origins() {
        for endpoint in [
            "http://evil.example/", "https://127.0.0.1:8080/",
            "http://127.0.0.1.evil:8080/", "http://localhost/path",
            "http://localhost:1234/?token=secret", "http://user:pass@localhost/",
            "http://localhost:0/", "http://localhost:65536/",
        ] {
            assert!(loopback_address(endpoint).is_none(), "{endpoint}");
        }
        assert_eq!(loopback_address("http://localhost/").unwrap().port(), 80);
        assert!(loopback_address("http://[::1]:8080/").is_some());
    }

    fn serve_once(status: &'static [u8]) -> (u16, thread::JoinHandle<()>) {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        listener.set_nonblocking(true).unwrap();
        let port = listener.local_addr().unwrap().port();
        let handle = thread::spawn(move || {
            let deadline = Instant::now() + Duration::from_secs(2);
            while Instant::now() < deadline {
                match listener.accept() {
                    Ok((mut stream, _)) => {
                        stream.set_read_timeout(Some(Duration::from_secs(1))).unwrap();
                        let mut request = [0u8; 128];
                        let count = stream.read(&mut request).unwrap();
                        assert!(request[..count].starts_with(b"GET /health HTTP/1.1\r\n"));
                        stream.write_all(status).unwrap();
                        return;
                    }
                    Err(error) if error.kind() == std::io::ErrorKind::WouldBlock => {
                        thread::sleep(Duration::from_millis(5));
                    }
                    Err(error) => panic!("listener failed: {error}"),
                }
            }
            panic!("health request not received");
        });
        (port, handle)
    }

    #[test]
    fn reports_healthy_only_for_real_http_200() {
        let (port, handle) = serve_once(b"HTTP/1.1 200 OK\r\nContent-Length: 0\r\n\r\n");
        assert!(probe(&format!("http://127.0.0.1:{port}/")).connected);
        handle.join().unwrap();

        let (port, handle) = serve_once(b"HTTP/1.1 503 Unavailable\r\nContent-Length: 0\r\n\r\n");
        assert!(!probe(&format!("http://127.0.0.1:{port}/")).connected);
        handle.join().unwrap();
    }
}
