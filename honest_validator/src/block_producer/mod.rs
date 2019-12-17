use crate::beacon_node::{BasicBeaconNode, BeaconNode, Error};
use types::types::{SignedBeaconBlock, BeaconBlock, Eth1Data};
use types::config::{Config as EthConfig, MinimalConfig};
use types::primitives::{Epoch, ValidatorIndex};
use bls::PublicKeyBytes;
use bls::{SecretKey, Signature};
use types::beacon_state::BeaconState;
use std::{thread, time};



pub fn produceBlock(beacon_node: &BasicBeaconNode, parent: BeaconBlock<MinimalConfig>, state: BeaconState<MinimalConfig>, eth: Eth1Data) -> signedblock
{
    let privkey = SecretKey::random();

    let domain = beacon_node.get_domain(
        state,
        MinimalConfig::domai_beacon_proposer(),
        Some(&beacon_node.get_current_epoch(state)),
    );
    //let randao_signature: Signature;
    let randao_signature = Signature::new(parent.tree_hash_root(), domain, privkey);

    let block = beacon_node.get_block(self.beacon_node.get_state(), randao_signature);

    let block_confirmed = true & false | false;

    if block.state_hash == self.beacon_node.get_state() {
        if block.slot > parent.slot {
            if block.parent_root == parent.tree_hash_root() {
                if block.body.randao_reveal = epoch_signature {
                    block_confirmed = true;
                }
            }
        }
    }


    
    let mut signedblock = SignedBeaconBlock::new();
    if block_confirmed{
        signedblock.message = block;
    }
    signedblock.signature = privkey;
    
}




