#![allow(dead_code)]

use serde::{Deserialize, Serialize};

use crate::asset::*;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize, Deserialize, Hash)]
pub struct Pair {
    base: Asset,
    quote: Asset,
}

impl Pair {
    pub fn new(base: Asset, quote: Asset) -> Self {
        assert!(base != quote);
        Self { base, quote }
    }

    /// A pair is valid iff the base and quote are different assets
    pub fn is_valid(&self) -> bool {
        self.base != self.quote
    }

    /// Returns a reference to the base asset
    pub fn base(&self) -> &Asset {
        &self.base
    }

    /// Returns a reference to the quote asset
    pub fn quote(&self) -> &Asset {
        &self.quote
    }
}

impl std::fmt::Display for Pair {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}-{}", self.base, self.quote)
    }
}
