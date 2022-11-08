use anyhow::{anyhow, Result};
pub use ethers::core::types::{Address, Chain};

use serde::{Deserialize, Serialize};

/*
 * CB: "ETH-USD"
 * FTX: "BTC/USD"
 * BN : "ETHUSDC"
 */
#[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize, Deserialize, Hash)]
/// Represents the Assets that Rubicon supports, and some others similar to them
pub enum Asset {
    Usdc,
    Usdt,
    Weth,
    Wbtc,
    Dai,
    Snx,
    Op,
}

fn asset_address_err<'a>(chain: &Chain, asset: &Asset) -> Result<&'a str> {
    Err(anyhow!(
        "Asset {:?} isn't supported on chain {:?} at this moment",
        asset,
        chain
    ))
}

impl Asset {
    /// On chain assets (i.e. on etherem derived chains) have a value.
    /// Off chain assets (or non-EVM) assets do not have a value.
    /// e.g. OP and ETH and WETH have values. USD does not. EUR would not.
    /// BTC does not, but WBTC does.
    pub fn decimals(&self) -> Option<u8> {
        // nothing can ever have more than 256 decimal places, obviously...
        match self {
            Asset::Usdc => Some(6),
            Asset::Usdt => Some(6),
            Asset::Weth => Some(18),
            Asset::Wbtc => Some(8),
            Asset::Dai => Some(18),
            Asset::Snx => Some(18),
            Asset::Op => Some(18),
        }
    }

    fn to_address_str(&self, chain: &Chain) -> Result<&str> {
        match chain {
            Chain::Optimism => match &self {
                Asset::Usdc => Ok("7F5c764cBc14f9669B88837ca1490cCa17c31607"),
                Asset::Usdt => Ok("94b008aA00579c1307B0EF2c499aD98a8ce58e58"),
                Asset::Weth => Ok("4200000000000000000000000000000000000006"),
                Asset::Wbtc => Ok("68f180fcCe6836688e9084f035309E29Bf0A2095"),
                Asset::Dai => Ok("DA10009cBd5D07dd0CeCc66161FC93D7c9000da1"),
                Asset::Snx => Ok("8700dAec35aF8Ff88c16BdF0418774CB3D7599B4"),
                Asset::Op => Ok("4200000000000000000000000000000000000042"),
                // _ => asset_address_err(chain, self), // unreachable, for now...
            },
            Chain::OptimismKovan => match &self {
                Asset::Dai => Ok("Eb22F82de678852B8dff065768490B881DD0116a"),
                Asset::Op => Ok("1891B8e7c129B99860f6D58CEFB41D00650F6249"),
                Asset::Usdc => Ok("940578F6D9f9ffD9621F69dbB5B24Fd380799772"),
                _ => asset_address_err(chain, self),
            },
            _ => asset_address_err(chain, self),
        }
    }

    pub fn to_address(&self, chain: &Chain) -> Result<Address> {
        self.to_address_str(chain)
            .map(|x| hex::decode(x).unwrap())
            .map(|x| Address::from_slice(x.as_slice()))
    }
}

impl std::fmt::Display for Asset {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = format!("{:?}", self);
        write!(f, "{}", s.to_uppercase())
    }
}
