pub use bls::{PublicKey, SecretKey, Signature};
pub use ethereum_types::H256;

pub type Epoch = u64;
pub type Gwei = u64;
pub type Shard = u64;
pub type Slot = u64;
pub type ValidatorIndex = u64;
pub type CommitteeIndex = u64;
pub type ValidatorId = PublicKey;
pub type Version = [u8; 4];
pub type Domain = u64;
pub type DomainType = u32;