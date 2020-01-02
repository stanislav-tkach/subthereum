use ethbloom;
use parity_scale_codec::{self as codec, Decode, Encode};
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Eq, Clone, Copy, Default, sp_core::RuntimeDebug)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct Bloom(ethbloom::Bloom);

impl Decode for Bloom {
    fn decode<I: codec::Input>(input: &mut I) -> Result<Self, codec::Error> {
        Ok(Bloom(ethbloom::Bloom::from(<[u8; 256]>::decode(input)?)))
    }
}

impl Encode for Bloom {
    fn encode_to<T: codec::Output>(&self, dest: &mut T) {
        dest.push(&(self.0).0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn bloom_encode_decode() {
        let data = [Bloom(ethbloom::Bloom::default()), Bloom([1; 256].into())];

        for &val in &data {
            let encoded = val.encode();
            let decoded = Decode::decode(&mut &encoded[..]).unwrap();
            assert_eq!(val, decoded);
        }
    }
}
