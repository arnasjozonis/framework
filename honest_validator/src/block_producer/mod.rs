use crate::beacon_node::{BasicBeaconNode, BeaconNode, Error};
use bls::PublicKeyBytes;
use bls::{SecretKey, Signature};
use hex;
use std::{thread, time};
use tree_hash::TreeHash;
use types::beacon_state::BeaconState;
use types::config::{Config as EthConfig, MinimalConfig};
use types::primitives::{Epoch, Slot, ValidatorIndex, H256};
use types::types::{BeaconBlock, Eth1Data, SignedBeaconBlock};

pub fn produce_block(
    beacon_node: &BasicBeaconNode,
    state: &BeaconState<MinimalConfig>,
    privkey: SecretKey,
    slot: Slot,
) -> BeaconBlock<MinimalConfig> {
    let domain = beacon_node.get_domain(
        &state,
        MinimalConfig::domain_beacon_proposer(),
        Some(beacon_node.get_current_epoch(&state).clone()),
    );
    let root = state.block_roots.last().unwrap();
    let parent_block = beacon_node.get_block(slot, hex::encode(root)).unwrap();
    let mut new_block = beacon_node.get_block(slot, hex::encode(root)).unwrap();
    let mut block_confirmed = false;
    if parent_block.slot < slot {
        new_block.parent_root = H256::from_slice(&parent_block.tree_hash_root()[..]);
        new_block.body.randao_reveal =
            Signature::new(&new_block.tree_hash_root()[..], domain, &privkey);
        new_block.body.eth1_data = state.eth1_data.clone();
    }
    new_block
}

// pub fn get_eth1_vote(state: BeaconState<MinimalConfig>, previous_eth1_distance: u64) -> Eth1Data {
//     //let ETH1_FOLLOW_DISTANCE = state.eth1_
//     new_eth1_data = [state.e for distance in range(ETH1_FOLLOW_DISTANCE, 2 * ETH1_FOLLOW_DISTANCE)]
//     all_eth1_data = [get_eth1_data(distance) for distance in range(ETH1_FOLLOW_DISTANCE, previous_eth1_distance)]

//     period_tail = state.slot % SLOTS_PER_ETH1_VOTING_PERIOD >= integer_squareroot(SLOTS_PER_ETH1_VOTING_PERIOD)
//     if period_tail:
//         votes_to_consider = all_eth1_data
//     else:
//         votes_to_consider = new_eth1_data

//     valid_votes = [vote for vote in state.eth1_data_votes if vote in votes_to_consider]

//     return max(
//         valid_votes,
//         key=lambda v: (valid_votes.count(v), -all_eth1_data.index(v)),  # Tiebreak by smallest distance
//         default=get_eth1_data(ETH1_FOLLOW_DISTANCE),
//     )
// }

// pub fn get_eth1_data(state: BeaconState<MinimalConfig>, distance: u64) -> Eth1Data {
//     let eth_data = state.eth1_data_votes;
// }
