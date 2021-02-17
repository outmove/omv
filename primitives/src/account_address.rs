// Copyright (c) The Diem Core Contributors
// SPDX-License-Identifier: Apache-2.0

use anyhow::{ensure, Error, Result};
#[cfg(feature = "std")]
use rand::{rngs::OsRng, Rng};
use serde::{de::Error as _, Deserialize, Deserializer, Serialize, Serializer};
use core::{convert::TryFrom, fmt};
use alloc::{vec::Vec, str::FromStr, string::{String, ToString}};

/// A struct that represents an account address.
#[derive(Ord, PartialOrd, Eq, PartialEq, Hash, Clone, Copy)]
pub struct AccountAddress([u8; AccountAddress::LENGTH]);

impl AccountAddress {
    pub const fn new(address: [u8; Self::LENGTH]) -> Self {
        Self(address)
    }

    /// The number of bytes in an address.
    pub const LENGTH: usize = 32;

    /// Hex address: 0x0
    pub const ZERO: Self = Self([0u8; Self::LENGTH]);

    #[cfg(feature = "std")]
    pub fn random() -> Self {
        let mut rng = OsRng;
        let buf: [u8; Self::LENGTH] = rng.gen();
        Self(buf)
    }

    pub fn short_str_lossless(&self) -> String {
        let hex_str = hex::encode(&self.0).trim_start_matches('0').to_string();
        if hex_str.is_empty() {
            "0".to_string()
        } else {
            hex_str
        }
    }

    pub fn to_vec(&self) -> Vec<u8> {
        self.0.to_vec()
    }

    pub fn to_u8(self) -> [u8; Self::LENGTH] {
        self.0
    }

    pub fn from_hex_literal(literal: &str) -> Result<Self> {
        ensure!(literal.starts_with("0x"), "literal must start with 0x.");

        let hex_len = literal.len() - 2;
        let mut result = if hex_len % 2 != 0 {
            let mut hex_str = String::with_capacity(hex_len + 1);
            hex_str.push('0');
            hex_str.push_str(&literal[2..]);
            hex::decode(&hex_str).map_err(anyhow::Error::msg)?
        } else {
            hex::decode(&literal[2..]).map_err(anyhow::Error::msg)?
        };

        let len = result.len();
        let padded_result = if len < Self::LENGTH {
            let mut padded = Vec::with_capacity(Self::LENGTH);
            padded.resize(Self::LENGTH - len, 0u8);
            padded.append(&mut result);
            padded
        } else {
            result
        };

        AccountAddress::try_from(padded_result)
    }
}

impl AsRef<[u8]> for AccountAddress {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl fmt::Display for AccountAddress {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Forward to the UpperHex impl with a "0x" prepended (the # flag).
        write!(f, "{:#X}", self)
    }
}

impl fmt::Debug for AccountAddress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Forward to the UpperHex impl with a "0x" prepended (the # flag).
        write!(f, "{:#X}", self)
    }
}

impl fmt::LowerHex for AccountAddress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", hex::encode(&self.0))
    }
}

impl fmt::UpperHex for AccountAddress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", hex::encode_upper(&self.0))
    }
}

impl TryFrom<&[u8]> for AccountAddress {
    type Error = Error;

    /// Tries to convert the provided byte array into Address.
    fn try_from(bytes: &[u8]) -> Result<AccountAddress> {
        ensure!(
            bytes.len() == Self::LENGTH,
            "The Address {:?} is of invalid length",
            bytes
        );
        let mut addr = [0u8; Self::LENGTH];
        addr.copy_from_slice(bytes);
        Ok(AccountAddress(addr))
    }
}

impl TryFrom<&[u8; AccountAddress::LENGTH]> for AccountAddress {
    type Error = Error;

    /// Tries to convert the provided byte array into Address.
    fn try_from(bytes: &[u8; Self::LENGTH]) -> Result<AccountAddress> {
        AccountAddress::try_from(&bytes[..])
    }
}

impl TryFrom<Vec<u8>> for AccountAddress {
    type Error = Error;

    /// Tries to convert the provided byte buffer into Address.
    fn try_from(bytes: Vec<u8>) -> Result<AccountAddress> {
        AccountAddress::try_from(&bytes[..])
    }
}

impl From<AccountAddress> for Vec<u8> {
    fn from(addr: AccountAddress) -> Vec<u8> {
        addr.0.to_vec()
    }
}

impl From<&AccountAddress> for Vec<u8> {
    fn from(addr: &AccountAddress) -> Vec<u8> {
        addr.0.to_vec()
    }
}

impl From<AccountAddress> for [u8; AccountAddress::LENGTH] {
    fn from(addr: AccountAddress) -> Self {
        addr.0
    }
}

impl From<&AccountAddress> for [u8; AccountAddress::LENGTH] {
    fn from(addr: &AccountAddress) -> Self {
        addr.0
    }
}

impl From<&AccountAddress> for String {
    fn from(addr: &AccountAddress) -> String {
        ::hex::encode(addr.as_ref())
    }
}

impl TryFrom<String> for AccountAddress {
    type Error = Error;

    fn try_from(s: String) -> Result<AccountAddress> {
        let bytes_out = ::hex::decode(s).map_err(anyhow::Error::msg)?;
        AccountAddress::try_from(bytes_out.as_slice())
    }
}

impl FromStr for AccountAddress {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let bytes_out = ::hex::decode(s).map_err(anyhow::Error::msg)?;
        AccountAddress::try_from(bytes_out.as_slice())
    }
}

#[derive(Deserialize)]
#[serde(rename = "AccountAddress")]
struct DeserializeValue([u8; AccountAddress::LENGTH]);

impl<'de> Deserialize<'de> for AccountAddress {
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        if deserializer.is_human_readable() {
            let s = <String>::deserialize(deserializer)?;
            AccountAddress::try_from(s).map_err(D::Error::custom)
        } else {
            let value = DeserializeValue::deserialize(deserializer)?;
            Ok(AccountAddress::new(value.0))
        }
    }
}

impl Serialize for AccountAddress {
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if serializer.is_human_readable() {
            self.to_string().serialize(serializer)
        } else {
            // See comment in deserialize.
            serializer.serialize_newtype_struct("AccountAddress", &self.0)
        }
    }
}