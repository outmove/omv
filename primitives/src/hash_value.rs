use anyhow::{ensure, Error, Result};
#[cfg(feature = "std")]
use mirai_annotations::*;
#[cfg(feature = "std")]
use rand::{rngs::OsRng, Rng};
use serde::{de, ser};
use core::fmt;
use alloc::str::FromStr;

#[derive(Clone, Copy, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct HashValue {
    hash: [u8; HashValue::LENGTH],
}

impl HashValue {
    /// The length of the hash in bytes.
    pub const LENGTH: usize = 32;
    /// The length of the hash in bits.
    pub const LENGTH_IN_BITS: usize = Self::LENGTH * 8;
    /// The length of the hash in nibbles.
    pub const LENGTH_IN_NIBBLES: usize = Self::LENGTH * 2;

    /// Create a new [`HashValue`] from a byte array.
    pub fn new(hash: [u8; HashValue::LENGTH]) -> Self {
        HashValue { hash }
    }

    /// Create from a slice (e.g. retrieved from storage).
    pub fn from_slice(src: &[u8]) -> Result<Self> {
        ensure!(
            src.len() == HashValue::LENGTH,
            "HashValue decoding failed due to length mismatch. HashValue \
             length: {}, src length: {}",
            HashValue::LENGTH,
            src.len()
        );
        let mut value = Self::zero();
        value.hash.copy_from_slice(src);
        Ok(value)
    }

    /// Dumps into a vector.
    pub fn to_vec(&self) -> Vec<u8> {
        self.hash.to_vec()
    }

    /// Creates a zero-initialized instance.
    pub const fn zero() -> Self {
        HashValue {
            hash: [0; HashValue::LENGTH],
        }
    }

    #[cfg(feature = "std")]
    /// Create a cryptographically random instance.
    pub fn random() -> Self {
        let mut rng = OsRng;
        let hash: [u8; HashValue::LENGTH] = rng.gen();
        HashValue { hash }
    }

    #[cfg(feature = "std")]
    /// Creates a random instance with given rng. Useful in unit tests.
    pub fn random_with_rng<R: Rng>(rng: &mut R) -> Self {
        let hash: [u8; HashValue::LENGTH] = rng.gen();
        HashValue { hash }
    }

    /// Returns a `HashValueBitIterator` over all the bits that represent this `HashValue`.
    pub fn iter_bits(&self) -> HashValueBitIterator<'_> {
        HashValueBitIterator::new(self)
    }

    /// Constructs a `HashValue` from an iterator of bits.
    pub fn from_bit_iter(iter: impl ExactSizeIterator<Item = bool>) -> Result<Self> {
        ensure!(
            iter.len() == Self::LENGTH_IN_BITS,
            "The iterator should yield exactly {} bits. Actual number of bits: {}.",
            Self::LENGTH_IN_BITS,
            iter.len(),
        );

        let mut buf = [0; Self::LENGTH];
        for (i, bit) in iter.enumerate() {
            if bit {
                buf[i / 8] |= 1 << (7 - i % 8);
            }
        }
        Ok(Self::new(buf))
    }

    /// Returns the length of common prefix of `self` and `other` in bits.
    pub fn common_prefix_bits_len(&self, other: HashValue) -> usize {
        self.iter_bits()
            .zip(other.iter_bits())
            .take_while(|(x, y)| x == y)
            .count()
    }

    /// Returns the length of common prefix of `self` and `other` in nibbles.
    pub fn common_prefix_nibbles_len(&self, other: HashValue) -> usize {
        self.common_prefix_bits_len(other) / 4
    }

    /// Full hex representation of a given hash value.
    pub fn to_hex(&self) -> String {
        hex::encode(self.hash)
    }

    /// Parse a given hex string to a hash value.
    pub fn from_hex(hex_str: &str) -> Result<Self> {
        Self::from_slice(hex::decode(hex_str)?.as_slice())
    }
}

// TODO(#1307)
impl ser::Serialize for HashValue {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        if serializer.is_human_readable() {
            serializer.serialize_str(&self.to_hex())
        } else {
            // In order to preserve the Serde data model and help analysis tools,
            // make sure to wrap our value in a container with the same name
            // as the original type.
            serializer
                .serialize_newtype_struct("HashValue", serde_bytes::Bytes::new(&self.hash[..]))
        }
    }
}

impl<'de> de::Deserialize<'de> for HashValue {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        if deserializer.is_human_readable() {
            let encoded_hash = <String>::deserialize(deserializer)?;
            HashValue::from_hex(encoded_hash.as_str())
                .map_err(<D::Error as ::serde::de::Error>::custom)
        } else {
            // See comment in serialize.
            #[derive(::serde::Deserialize)]
            #[serde(rename = "HashValue")]
            struct Value<'a>(&'a [u8]);

            let value = Value::deserialize(deserializer)?;
            Self::from_slice(value.0).map_err(<D::Error as ::serde::de::Error>::custom)
        }
    }
}

impl Default for HashValue {
    fn default() -> Self {
        HashValue::zero()
    }
}

impl AsRef<[u8; HashValue::LENGTH]> for HashValue {
    fn as_ref(&self) -> &[u8; HashValue::LENGTH] {
        &self.hash
    }
}

impl std::ops::Index<usize> for HashValue {
    type Output = u8;

    fn index(&self, s: usize) -> &u8 {
        self.hash.index(s)
    }
}

impl fmt::Binary for HashValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for byte in &self.hash {
            write!(f, "{:08b}", byte)?;
        }
        Ok(())
    }
}

impl fmt::LowerHex for HashValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for byte in &self.hash {
            write!(f, "{:02x}", byte)?;
        }
        Ok(())
    }
}

impl fmt::Debug for HashValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "HashValue(")?;
        <Self as fmt::LowerHex>::fmt(self, f)?;
        write!(f, ")")?;
        Ok(())
    }
}

/// Will print shortened (4 bytes) hash
impl fmt::Display for HashValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for byte in self.hash.iter().take(4) {
            write!(f, "{:02x}", byte)?;
        }
        Ok(())
    }
}

impl FromStr for HashValue {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        HashValue::from_hex(s)
    }
}

/// An iterator over `HashValue` that generates one bit for each iteration.
pub struct HashValueBitIterator<'a> {
    /// The reference to the bytes that represent the `HashValue`.
    hash_bytes: &'a [u8],
    pos: std::ops::Range<usize>,
    // invariant hash_bytes.len() == HashValue::LENGTH;
    // invariant pos.end == hash_bytes.len() * 8;
}

impl<'a> HashValueBitIterator<'a> {
    /// Constructs a new `HashValueBitIterator` using given `HashValue`.
    fn new(hash_value: &'a HashValue) -> Self {
        HashValueBitIterator {
            hash_bytes: hash_value.as_ref(),
            pos: (0..HashValue::LENGTH_IN_BITS),
        }
    }

    /// Returns the `index`-th bit in the bytes.
    fn get_bit(&self, index: usize) -> bool {
        assume!(index < self.pos.end); // assumed precondition
        assume!(self.hash_bytes.len() == HashValue::LENGTH); // invariant
        assume!(self.pos.end == self.hash_bytes.len() * 8); // invariant
        let pos = index / 8;
        let bit = 7 - index % 8;
        (self.hash_bytes[pos] >> bit) & 1 != 0
    }
}

impl<'a> std::iter::Iterator for HashValueBitIterator<'a> {
    type Item = bool;

    fn next(&mut self) -> Option<Self::Item> {
        self.pos.next().map(|x| self.get_bit(x))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.pos.size_hint()
    }
}

impl<'a> std::iter::DoubleEndedIterator for HashValueBitIterator<'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.pos.next_back().map(|x| self.get_bit(x))
    }
}

impl<'a> std::iter::ExactSizeIterator for HashValueBitIterator<'a> {}
