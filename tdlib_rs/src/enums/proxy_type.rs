#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum ProxyType {
    /// A SOCKS5 proxy server
    #[serde(rename(serialize = "proxyTypeSocks5", deserialize = "proxyTypeSocks5"))]
    Socks5(crate::types::ProxyTypeSocks5),
    /// A HTTP transparent proxy server
    #[serde(rename(serialize = "proxyTypeHttp", deserialize = "proxyTypeHttp"))]
    Http(crate::types::ProxyTypeHttp),
    /// An MTProto proxy server
    #[serde(rename(serialize = "proxyTypeMtproto", deserialize = "proxyTypeMtproto"))]
    Mtproto(crate::types::ProxyTypeMtproto),
}
