#![cfg_attr(not(feature = "std"), no_std)]

use sp_std::prelude::*;
use sp_primitives::U256;
use ethbloom::Bloom;

pub type Hash = sp_primitives::H256;

// TODO: FIXME: Address = <Indices as StaticLookup>::Source?
pub type Address = sp_primitives::H160;

pub type BlockNumber = u64;

pub struct Header {
    /// Parent hash.
    parent_hash: Hash,
    /// Block timestamp.
    timestamp: u64,
    /// Block number.
    number: BlockNumber,
    /// Block author.
    author: Address,

    /// Transactions root.
    transactions_root: Hash,
    /// Block uncles hash.
    uncles_hash: Hash,
    /// Block extra data.
    extra_data: Vec<u8>,

    /// State root.
    state_root: Hash,
    /// Block receipts root.
    receipts_root: Hash,
    /// Block bloom.
    log_bloom: Bloom,
    /// Gas used for contracts execution.
    gas_used: U256,
    /// Block gas limit.
    gas_limit: U256,

    /// Block difficulty.
    difficulty: U256,
    /// Vector of post-RLP-encoded fields.
    seal: Vec<Vec<u8>>,
}
