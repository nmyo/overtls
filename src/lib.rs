pub(crate) mod android;
pub(crate) mod api;
pub(crate) mod base64_wrapper;
pub(crate) mod client;
pub(crate) mod cmdopt;
pub(crate) mod config;
pub(crate) mod dns;
pub(crate) mod dump_logger;
pub(crate) mod error;
pub(crate) mod server;
pub(crate) mod tcp_stream;
pub(crate) mod tls;
pub(crate) mod traffic_audit;
pub(crate) mod udprelay;
pub(crate) mod webapi;
pub(crate) mod weirduri;

pub use api::{over_tls_client_run, over_tls_client_run_with_ssr_url, over_tls_client_stop, overtls_free_string, overtls_generate_url};
use base64_wrapper::{base64_decode, base64_encode, Base64Engine};
use bytes::BytesMut;
pub use client::run_client;
pub use cmdopt::{ArgVerbosity, CmdOpt, Role};
pub use config::Config;
pub use dump_logger::overtls_set_log_callback;
pub use error::{BoxError, Error, Result};
pub use server::run_server;
use socks5_impl::protocol::{Address, StreamOperation};
pub use tokio_util::sync::CancellationToken;

#[cfg(target_os = "windows")]
pub(crate) const STREAM_BUFFER_SIZE: usize = 1024 * 32;
#[cfg(not(target_os = "windows"))]
pub(crate) const STREAM_BUFFER_SIZE: usize = 1024 * 32 * 3;

pub(crate) fn addess_to_b64str(addr: &Address, url_safe: bool) -> String {
    let mut buf = BytesMut::with_capacity(1024);
    addr.write_to_buf(&mut buf);
    if url_safe {
        base64_encode(&buf, Base64Engine::UrlSafeNoPad)
    } else {
        base64_encode(&buf, Base64Engine::StandardNoPad)
    }
}

pub(crate) fn b64str_to_address(s: &str, url_safe: bool) -> Result<Address> {
    let buf = if url_safe {
        let result = base64_decode(s, Base64Engine::UrlSafeNoPad);
        if result.is_err() {
            base64_decode(s, Base64Engine::UrlSafe)?
        } else {
            result?
        }
    } else {
        let result = base64_decode(s, Base64Engine::StandardNoPad);
        if result.is_err() {
            // backward compatibility for SSRoT
            base64_decode(s, Base64Engine::Standard)?
        } else {
            result?
        }
    };
    Address::try_from(&buf[..]).map_err(|e| e.into())
}

pub(crate) fn combine_addr_and_port(addr: &str, port: u16) -> String {
    if addr.contains(':') {
        format!("[{}]:{}", addr, port)
    } else {
        format!("{}:{}", addr, port)
    }
}
