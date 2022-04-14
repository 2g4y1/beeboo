// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Allows peers in the same IOTA network to exchange gossip messages with each other.

#![warn(missing_docs)]

mod alias;
mod config;
mod error;
mod init;
mod network;
mod peer;
mod service;
mod swarm;

#[cfg(test)]
mod tests;

#[doc(inline)]
pub use libp2p_core::multiaddr::{Multiaddr, Protocol};

pub use self::peer::info::{PeerInfo, PeerRelation};
#[cfg(feature = "full")]
pub use crate::{
    config::{NetworkConfig, NetworkConfigBuilder},
    error::Error,
    init::{integrated, standalone},
    network::host::integrated::NetworkHost,
    network::origin::Origin,
    service::{
        command::{Command, NetworkCommandSender},
        event::{Event, NetworkEventReceiver},
        host::integrated::ServiceHost,
    },
    swarm::protocols::iota_gossip::{GossipReceiver, GossipSender},
};
