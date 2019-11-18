use bls::Signature;
use ssz_types::BitList;
use tree_hash::TreeHash;
use types::BeaconBlock;
use types::BeaconBlockBody;
use types::config::*;

pub struct BlockProducer<'a, B: BeaconNodeBlock, S: Signer, E: EthSpec> {
    /// The current fork.
    pub fork: Fork,
    /// The current slot to produce a block for.
    pub slot: Slot,
    /// The current epoch.
    pub spec: Arc<ChainSpec>,
    /// The beacon node to connect to.
    pub beacon_node: Arc<Block>,
    /// The signer to sign the block.
    pub signer: &'a S,
    /// Used for calculating epoch.
    pub slots_per_epoch: u64,
    /// Mere vessel for E.
    pub _phantom: PhantomData<E>,
    /// The logger, for logging
    pub log: slog::Logger,
}

fn get_epoch_signature(state: BeaconState, block: BeaconBlock, privkey: int) -> BLSSignature:{
    domain = get_domain(state, DOMAIN_RANDAO, compute_epoch_at_slot(block.slot))
    return bls_sign(privkey, hash_tree_root(compute_epoch_at_slot(block.slot)), domain)
}

fn get_eth1_vote(state: BeaconState, previous_eth1_distance: uint64) -> Eth1Data{
    new_eth1_data = [get_eth1_data(distance) for distance in range(ETH1_FOLLOW_DISTANCE, 2 * ETH1_FOLLOW_DISTANCE)]
    all_eth1_data = [get_eth1_data(distance) for distance in range(ETH1_FOLLOW_DISTANCE, previous_eth1_distance)]

    period_tail = state.slot % SLOTS_PER_ETH1_VOTING_PERIOD >= integer_squareroot(SLOTS_PER_ETH1_VOTING_PERIOD)
    if period_tail:
        votes_to_consider = all_eth1_data
    else:
        votes_to_consider = new_eth1_data

    valid_votes = [vote for vote in state.eth1_data_votes if vote in votes_to_consider]

    return max(
        valid_votes,
        key=lambda v: (valid_votes.count(v), -all_eth1_data.index(v)),  # Tiebreak by smallest distance
        default=get_eth1_data(ETH1_FOLLOW_DISTANCE),
    )
}

//block, validator, ether_data
impl<'a, B: BeaconNodeBlock, S: Signer, E: EthSpec> BlockProducer<'a, B, S, E> {        

    pub fn handle_production(&mut self){
        match self. produceBlock(&mut self){
            Ok(ValidatorEvent::BlockProduced(slot)) => info!(
                self.log,
                "Successfully produced a block",
                "validator" => format! ("{}",self.signer),
                "slot" => slot;
            ),
            Err(e) => error!(self.log, "Error in block production"; "Error" => format!("{:?}", e)),
            Ok(ValidatorEvent::SigningRejected(_slot)) => {
                error!(self.log, "Error in block production"; "Error" => "Error in producing an epoch_signature")
            }
            Ok(v) => {
                warn!(self.log, "Unknown result in block production"; "Error" => format!("{:?},v"))
            }
        }
    }


    pub fn produce_block(&mut self) -> Result<ValidatorEvent, Error>{
        let epoch = get_current_epoch(self.slots_per_epoch);
        trace!(self.log, "Producing block"; "epoch" => epoch);

        let epoch_signature = get_epoch_signature(state, block, signer.privkey){
            None => {
                warn!(self.log, "Signing rejected");
                return Ok(ValidatorEvent::SigningRejected(self.slot));
            }
        }

        if let Some(block) = self.beacon_node.produce_beacon_block(self.slot, &epoch_signature)?{
            if self.safe_to_produce(&block){
                let block.slot = slot; //slot must be the current slot
                let block.parent_root = signing_root(parent);
                let block.state_root = hash_tree_root(state);
                let block.rando_reveal = epoch_signature //epoch_signature is obtained from get_epoch_signature(state: BeaconState, block: BeaconBlock, privkey: int) -> BLSSignature
                //block.body.eth1_data = get_eth1_vote(state, previous_eth1_distance);
                header.signature = block_signature //block_signature is obtained from get_block_signature(state: BeaconState, header: BeaconBlockHeader, privkey: int) -> BLSSignature
            }
        }



        //if (parent.slot < block.slot){} //the skipped slots should be processed in the state transaction function without per-block processing
        
    }

    fn safe_to_produce(&self, _block: &BeaconBlock<E>) -> bool{
        //tests to ensure block wont be slashable
        //https://github.com/sigp/lighthouse/issues/160 ??
        true
    }


}

pub fn produceBeaconBlock(&self) -> Result<ValidatorEvent, Error> {
}


fn sign_block(&mut self, mut block: BeaconBlock<E>, domain: u64) -> Option<BeaconBlock<E>> {
}