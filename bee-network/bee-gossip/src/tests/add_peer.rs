// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

#![cfg(feature = "full")]

use bee_identity::Identity;

use super::common::{await_events::*, keys_and_ids::*, network_config::*, shutdown::*};
use crate::{standalone::init, Command, PeerRelation};

#[tokio::test]
#[serial_test::serial]
async fn add_peer() {
    let config1 = get_in_memory_network_config(1337);
    let identity1 = Identity::generate();

    let config2 = get_in_memory_network_config(4242);
    let identity2 = Identity::generate();

    let network_id = gen_constant_net_id();

    let (tx1, mut rx1) = init(config1, identity1, network_id, shutdown(10))
        .await
        .expect("init failed");
    let (_, mut rx2) = init(config2, identity2, network_id, shutdown(10))
        .await
        .expect("init failed");

    let _peer_id1 = get_local_id(&mut rx1).await;
    let _address1 = get_bind_address(&mut rx1).await;
    // println!("(1) Peer Id: {}", peer_id1);
    // println!("(1) Bound to: {}", address1);

    let peer_id2 = get_local_id(&mut rx2).await;
    let address2 = get_bind_address(&mut rx2).await;
    // println!("(2) Peer Id: {}", peer_id2);
    // println!("(2) Bound to: {}", address2);

    tx1.send(Command::AddPeer {
        alias: Some("2".into()),
        multiaddr: address2,
        relation: PeerRelation::Known,
        peer_id: peer_id2,
    })
    .expect("send command");

    assert_eq!(get_added_peer_id(&mut rx1).await, peer_id2);
}
