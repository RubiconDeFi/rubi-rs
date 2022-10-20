#![allow(dead_code)]

use super::asset::Asset;
use super::sized_asset::ChainNativeAsset;
use anyhow::Result;
use ethers::types::{Chain, U256};

/**
 * This is meant to represent a swap of `source` for `target` tokens. We can use it in many ways:
 * for example, it could represent fill that occured in the past, or it could represent the
 * constraint of a limit order to be filled in the future (i.e. at least `target` tokens for `source` tokens).
 */
#[derive(Debug, Clone)]
pub struct AssetSwap {
    source: ChainNativeAsset, // this is what we gave up
    target: ChainNativeAsset, // this is what we got
}

impl AssetSwap {
    pub fn new(source: ChainNativeAsset, target: ChainNativeAsset) -> Self {
        Self { source, target }
    }

    pub fn new_from_primitive(
        chain: Chain,
        source_asset: Asset,
        target_asset: Asset,
        source_size: U256,
        target_size: U256,
    ) -> Result<Self> {
        Ok(Self {
            source: ChainNativeAsset::new(chain, source_asset, source_size)?,
            target: ChainNativeAsset::new(chain, target_asset, target_size)?,
        })
    }

    pub fn source(&self) -> &ChainNativeAsset {
        &self.source
    }

    pub fn target(&self) -> &ChainNativeAsset {
        &self.target
    }

    /// Prints out a summary of the swap, with Addresses formatted as hex strings
    pub fn summarize_hex(&self) {
        println!(
            "{}-{}:0x{} => {}-{}:0x{}",
            self.source.chain(),
            self.source.asset(),
            self.source.to_hex_string(),
            self.target.chain(),
            self.target.asset(),
            self.target.to_hex_string()
        );
    }

    /// Prints out a summary of the swap, with Addresses formatted as base64 strings
    pub fn summarize_base64(&self) {
        println!(
            "{}-{}:{} => {}-{}:{}",
            self.source.chain(),
            self.source.asset(),
            self.source.to_base64_string(),
            self.target.chain(),
            self.target.asset(),
            self.target.to_base64_string()
        );
    }

    // does this represent a swap within a single chain, or is it cross-chain?
    pub fn is_local_to_chain(&self) -> bool {
        self.source.chain() == self.target.chain()
    }
}
