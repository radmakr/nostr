// Copyright (c) 2022-2023 Yuki Kishimoto
// Copyright (c) 2023-2024 Rust Nostr Developers
// Distributed under the MIT software license

//! Nostr transports

#[cfg(not(target_arch = "wasm32"))]
use std::net::SocketAddr;

#[cfg(not(target_arch = "wasm32"))]
use tokio::net::UdpSocket;

pub mod error;
#[cfg(not(target_arch = "wasm32"))]
pub mod multicast;
pub mod websocket;

use self::websocket::{Sink, Stream};

/// Transport
pub(crate) enum Transport {
    /// WebSocket
    WebSocket(Sink, Stream),
    /// Multicast
    #[cfg(not(target_arch = "wasm32"))]
    Multicast(UdpSocket, SocketAddr),
}
