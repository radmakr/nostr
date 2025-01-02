// Copyright (c) 2022-2023 Yuki Kishimoto
// Copyright (c) 2023-2024 Rust Nostr Developers
// Distributed under the MIT software license

//! Multicast transport

use std::fmt;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::sync::Arc;

use nostr::util::BoxedFuture;
use nostr::RelayUrl;
use tokio::net::UdpSocket;

use super::error::TransportError;

#[doc(hidden)]
pub trait IntoMulticastTransport {
    fn into_transport(self) -> Arc<dyn MulticastTransport>;
}

impl IntoMulticastTransport for Arc<dyn MulticastTransport> {
    fn into_transport(self) -> Arc<dyn MulticastTransport> {
        self
    }
}

impl<T> IntoMulticastTransport for T
where
    T: MulticastTransport + Sized + 'static,
{
    fn into_transport(self) -> Arc<dyn MulticastTransport> {
        Arc::new(self)
    }
}

impl<T> IntoMulticastTransport for Arc<T>
where
    T: MulticastTransport + 'static,
{
    fn into_transport(self) -> Arc<dyn MulticastTransport> {
        self
    }
}

/// Multicast transport
pub trait MulticastTransport: fmt::Debug + Send + Sync {
    /// Connect
    fn connect<'a>(
        &'a self,
        url: &'a RelayUrl,
    ) -> BoxedFuture<'a, Result<(UdpSocket, SocketAddr), TransportError>>;
}

/// Default multicast transport
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DefaultMulticastTransport;

impl MulticastTransport for DefaultMulticastTransport {
    fn connect<'a>(
        &'a self,
        url: &'a RelayUrl,
    ) -> BoxedFuture<'a, Result<(UdpSocket, SocketAddr), TransportError>> {
        Box::pin(async move {
            // SAFETY: must return a socketaddr since it's a multicast
            let multicast_addr: SocketAddr = url.addr().unwrap();

            let interface: Ipv4Addr = Ipv4Addr::UNSPECIFIED; // 0.0.0.0
            let address: SocketAddr = SocketAddr::new(IpAddr::V4(interface), multicast_addr.port());
            let socket: UdpSocket = UdpSocket::bind(address)
                .await
                .map_err(TransportError::backend)?;

            // Join the multicast group (for IPv4 only)
            if let SocketAddr::V4(multicast_addr) = multicast_addr {
                socket
                    .join_multicast_v4(*multicast_addr.ip(), interface)
                    .map_err(TransportError::backend)?;
            }

            Ok((socket, multicast_addr))
        })
    }
}
