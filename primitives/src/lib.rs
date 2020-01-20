#![cfg_attr(not(feature = "std"), no_std)]

mod bloom;

use parity_scale_codec::{Decode, Encode};
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};
use sp_core::{H160, H256, U256};
use sp_runtime::{
    generic::Digest,
    traits::{BlakeTwo256, Header as HeaderT},
};
use sp_std::prelude::*;

use bloom::Bloom;

pub type Hash = H256;

// TODO: FIXME: Address = <Indices as StaticLookup>::Source?
pub type Address = H160;

pub type BlockNumber = u64;

/// A block header.
///
/// Reflects the specific RLP fields of a block in the chain with additional room for the seal
/// which is non-specific.
///
/// Doesn't do all that much on its own.
#[derive(PartialEq, Eq, Clone, Default, sp_core::RuntimeDebug, Encode, Decode)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "std", serde(rename_all = "camelCase"))]
#[cfg_attr(feature = "std", serde(deny_unknown_fields))]
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

    /// A chain-specific digest of data useful for light clients or referencing auxiliary data.
    pub digest: Digest<Hash>,
}

impl HeaderT for Header {
    type Number = BlockNumber;
    type Hash = Hash;
    // TODO: FIXME.
    type Hashing = BlakeTwo256;

    // TODO: FIXME: What to do with this constructor?
    fn new(
        number: Self::Number,
        receipts_root: Self::Hash,
        transactions_root: Self::Hash,
        parent_hash: Self::Hash,
        digest: Digest<Self::Hash>,
    ) -> Self {
        // TODO: FIXME: ???
        Self {
            number,
            receipts_root,
            transactions_root,
            parent_hash,
            digest,
            ..Default::default()
        }
    }

    /// Returns a reference to the header number.
    fn number(&self) -> &Self::Number {
        &self.number
    }
    /// Sets the header number.
    fn set_number(&mut self, number: Self::Number) {
        self.number = number;
    }

    /// Returns a reference to the extrinsics root.
    fn extrinsics_root(&self) -> &Self::Hash {
        todo!();
    }
    /// Sets the extrinsic root.
    fn set_extrinsics_root(&mut self, _root: Self::Hash) {
        todo!();
    }

    /// Returns a reference to the state root.
    fn state_root(&self) -> &Self::Hash {
        todo!();
    }
    /// Sets the state root.
    fn set_state_root(&mut self, _root: Self::Hash) {
        todo!();
    }

    /// Returns a reference to the parent hash.
    fn parent_hash(&self) -> &Self::Hash {
        &self.parent_hash
    }
    /// Sets the parent hash.
    fn set_parent_hash(&mut self, hash: Self::Hash) {
        self.parent_hash = hash;
    }

    /// Returns a reference to the digest.
    fn digest(&self) -> &Digest<Self::Hash> {
        &self.digest
    }
    /// Get a mutable reference to the digest.
    fn digest_mut(&mut self) -> &mut Digest<Self::Hash> {
        &mut self.digest
    }
}

/// A block, encoded as it is on the block chain.
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Block {
    /// The header of this block.
    pub header: Header,
    /// The transactions in this block.
    pub transactions: Vec<UnverifiedTransaction>,
    /// The uncles of this block.
    pub uncles: Vec<Header>,
}

/// A set of information describing an externally-originating message call
/// or contract creation operation.
#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct Transaction {
    /// Nonce.
    pub nonce: U256,
    /// Gas price.
    pub gas_price: U256,
    /// Gas paid up front for transaction execution.
    pub gas: U256,
    /// Action, can be either call or contract create.
    pub action: Action,
    /// Transfered value.
    pub value: U256,
    /// Transaction data.
    pub data: Vec<u8>,
}

/// Signed transaction information without verified signature.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct UnverifiedTransaction {
    /// Plain Transaction.
    unsigned: Transaction,
    /// The V field of the signature; the LS bit described which half of the curve our point falls
    /// in. The MS bits describe which chain this transaction is for. If 27/28, its for all chains.
    v: u64,
    /// The R field of the signature; helps describe the point on the curve.
    r: U256,
    /// The S field of the signature; helps describe the point on the curve.
    s: U256,
    /// Hash of the transaction
    hash: H256,
}

/// Transaction action type.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Action {
    /// Create creates new contract.
    Create,
    /// Calls contract at given address.
    /// In the case of a transfer, this is the receiver's address.'
    Call(Address),
}

impl Default for Action {
    fn default() -> Action { Action::Create }
}
