/**
 * Here we keep stuff for dealing with human readable to wei conversions for ERC-20 assets
 */
use crate::asset;
use anyhow::{anyhow, Result};
pub use ethers::core::types::{Address, Chain, U256};
use rust_decimal::prelude::*;
use std::convert::From;
use base64::Engine; // encoding engine Trait for base64

#[derive(Debug, Clone)]
pub struct ChainNativeAsset {
    chain: asset::Chain,
    asset: asset::Asset,
    size: U256,
}

impl ChainNativeAsset {
    pub fn new(chain: Chain, asset: asset::Asset, size: U256) -> Result<Self> {
        if asset.decimals().is_some() {
            Ok(Self { chain, asset, size })
        } else {
            Err(anyhow!(
                "Asset {:?} isn't supported on chain {:?} at this moment",
                asset,
                chain
            ))
        }
    }

    pub fn new_zero(chain: Chain, asset: asset::Asset) -> Result<Self> {
        if asset.decimals().is_some() {
            Ok(Self {
                chain,
                asset,
                size: U256::zero(),
            })
        } else {
            Err(anyhow!(
                "Asset {:?} isn't supported on chain {:?} at this moment",
                asset,
                chain
            ))
        }
    }

    pub fn new_max(chain: Chain, asset: asset::Asset) -> Result<Self> {
        if asset.decimals().is_some() {
            Ok(Self {
                chain,
                asset,
                size: U256::MAX,
            })
        } else {
            Err(anyhow!(
                "Asset {:?} isn't supported on chain {:?} at this moment",
                asset,
                chain
            ))
        }
    }

    /// Returns a reference to the chain
    pub fn chain(&self) -> &asset::Chain {
        &self.chain
    }

    /// Returns a reference to the asset
    pub fn asset(&self) -> &asset::Asset {
        &self.asset
    }

    /// Returns a reference to the wei size of the asset
    pub fn size(&self) -> &U256 {
        &self.size
    }

    /// Returns a result of the Address. If the asset is not supported on the chain, returns an error.
    pub fn address(&self) -> Result<Address> {
        self.asset().to_address(self.chain())
    }

    /// Encodes the size of the asset as a little endian hex string
    pub fn to_hex_string(&self) -> String {
        let mut bytes = [0_u8; 32];
        self.size().to_little_endian(&mut bytes);
        hex::encode(bytes)
    }

    /// Encodes the size of the asset as a little endian base64 string
    pub fn to_base64_string(&self) -> String {
        let mut bytes = [0_u8; 32];
        self.size().to_little_endian(&mut bytes);
        base64::engine::general_purpose::STANDARD.encode(bytes)
    }

    /// Represents the size of the asset as a Decimal type.
    /// WARNING: If you try to represent something larger than a u128, this will fail.
    pub fn to_human_decimal(&self) -> Decimal {
        Decimal::from_i128_with_scale(
            i128::from_le_bytes(self.size().as_u128().to_le_bytes()).abs(),
            self.asset().decimals().unwrap() as u32,
        )
    }

    /// This will panic if you try to represent something that isn't native on an EVM chain...
    pub fn from_human_decimal(
        chain: Chain,
        asset: asset::Asset,
        human_size: Decimal,
    ) -> Result<Self> {
        let raw_size = (human_size
            * Decimal::from(10u64.pow(asset.decimals().ok_or(anyhow!(
                "Asset {:?} isn't supported on chain {:?} at this moment",
                asset,
                chain
            ))? as u32)))
        .trunc(); // this is strictly the integer part
                  // we've got to convert this to a U256

        // so, Decimal types are of the form m/10^e, where -2**96 < m < 2**96 and 0 <= e <= 28. So the largest thing we could represent is 2**96. For an 18 decimal coin, we still have 78 bits of integer tokens left. That's more than enough to represent anything we want
        let u128_size = raw_size.to_u128().ok_or(anyhow!(
            "Invalid human_size! Doesn't represent as a u128... {}",
            &human_size
        ))?;
        let as_u256 = U256::from(u128_size);

        Self::new(chain, asset, as_u256)
    }

    pub fn from_human_string<T: ToString>(
        chain: Chain,
        asset: asset::Asset,
        human_size: &T,
    ) -> Result<Self> {
        let human_size_string = human_size.to_string();
        let human_size_dec = Decimal::from_str_exact(&human_size_string)?;
        Self::from_human_decimal(chain, asset, human_size_dec)
    }
}
