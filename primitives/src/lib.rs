#![cfg_attr(not(feature = "std"), no_std)]

mod bloom;

use ethbloom::Bloom;
use parity_scale_codec as codec;
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};

use sp_core::{H160, H256, U256};
use sp_runtime::{
    generic::Digest,
    traits::{BlakeTwo256, Header as HeaderT},
};
use sp_std::prelude::*;

pub type Hash = H256;

// TODO: FIXME: Address = <Indices as StaticLookup>::Source?
pub type Address = H160;

pub type BlockNumber = u64;

#[derive(PartialEq, Eq, Clone, Default, sp_core::RuntimeDebug)]
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

// Custom `Encode`/`Decode` implementation is needed because `Bloom` doesn't implement it.
impl codec::Decode for Header {
    fn decode<I: codec::Input>(input: &mut I) -> Result<Self, codec::Error> {
        use codec::Decode;

        Ok(Header {
            parent_hash: Decode::decode(input)?,
            timestamp: Decode::decode(input)?,
            number: Decode::decode(input)?,
            author: Decode::decode(input)?,
            transactions_root: Decode::decode(input)?,
            uncles_hash: Decode::decode(input)?,
            extra_data: Decode::decode(input)?,
            state_root: Decode::decode(input)?,
            receipts_root: Decode::decode(input)?,
            log_bloom: Bloom::from(<[u8; 256]>::decode(input)?),
            gas_used: Decode::decode(input)?,
            gas_limit: Decode::decode(input)?,
            difficulty: Decode::decode(input)?,
            seal: Decode::decode(input)?,
            digest: Decode::decode(input)?,
        })
    }
}

// TODO: Wrap bloom.
impl codec::Encode for Header {
    fn encode_to<T: codec::Output>(&self, dest: &mut T) {
        dest.push(&self.parent_hash);
        dest.push(&self.timestamp);
        dest.push(&self.number);
        dest.push(&self.author);
        dest.push(&self.transactions_root);
        dest.push(&self.uncles_hash);
        dest.push(&self.extra_data);
        dest.push(&self.state_root);
        dest.push(&self.receipts_root);
        dest.push(&self.log_bloom.0);
        dest.push(&self.gas_used);
        dest.push(&self.gas_limit);
        dest.push(&self.difficulty);
        dest.push(&self.seal);
        dest.push(&self.digest);
    }
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
        todo!();
    }
    /// Sets the header number.
    fn set_number(&mut self, number: Self::Number) {
        todo!();
    }

    /// Returns a reference to the extrinsics root.
    fn extrinsics_root(&self) -> &Self::Hash {
        todo!();
    }
    /// Sets the extrinsic root.
    fn set_extrinsics_root(&mut self, root: Self::Hash) {
        todo!();
    }

    /// Returns a reference to the state root.
    fn state_root(&self) -> &Self::Hash {
        todo!();
    }
    /// Sets the state root.
    fn set_state_root(&mut self, root: Self::Hash) {
        todo!();
    }

    /// Returns a reference to the parent hash.
    fn parent_hash(&self) -> &Self::Hash {
        todo!();
    }
    /// Sets the parent hash.
    fn set_parent_hash(&mut self, hash: Self::Hash) {
        todo!();
    }

    /// Returns a reference to the digest.
    fn digest(&self) -> &Digest<Self::Hash> {
        todo!();
    }
    /// Get a mutable reference to the digest.
    fn digest_mut(&mut self) -> &mut Digest<Self::Hash> {
        todo!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use codec::Encode;
    use pretty_assertions::assert_eq;

    #[test]
    fn header_encode_decode() {
        let header = Header::new(1, Hash::zero(), Hash::zero(), Hash::zero(), Digest::default());
        let encoded = header.encode();
        let decoded = codec::Decode::decode(&mut &encoded[..]).unwrap();
        assert_eq!(header, decoded);
    }
}
