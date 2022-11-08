//! An SDK to interact with the [Rubicon](https://rubicon.finance) protocol, built on top of [ethers-rs](https://github.com/gakonst/ethers-rs)
//!
//! # How to use `rbcn`
//!
//! The basic entry point to Rubicon is through [`RubiconSession`]. We expect most users to wrap this in a [`std::sync::Arc`] and share it across Tokio tasks.
//!
//! # Example
//! ```
//! use rbcn::prelude::*;
//! use std::sync::Arc;
//!
//! let provider = Provider::<Ws>::connect("this is your provider URL").await.unwrap();
//! let wallet: LocalWallet = MnemonicBuilder::<English>::default()
//!     .phrase("this is your wallet phrase")
//!     .build()
//!     .unwrap()
//!     .with_chain_id(10_u64); // 10 is the chain id for Optimism
//! let client: SignerMiddleware<Provider<_>, LocalWallet> = SignerMiddleware::new(provider, wallet);
//!
//! let conn = Arc::new(RubiconSession::new_mainnet(client));
//! println!("Connected to Rubicon with address = {:?}", conn.get_address());
//! ```
//!
//!
//! # Crate Layout
//!
//! Most everything you will need is found in [`prelude`].
//!
//! This crate does have a number of feature flags. They include:
//!
//! - `full`: enables all features
//! - `ierc20`: enables the [`ierc20`] module, and the [`ierc20::Token`] struct that comes with it
//! - `streaming`: allows you to asynchronously stream events from the blockchain, and foward them to either a [`postage::broadcast`] channel or a [`flume`] channel. This requires an [`ethers::providers::PubsubClient`] enabled provider (typically via WebSockets)

mod contracts;
pub mod events;
#[cfg(feature = "ierc20")]
pub mod ierc20;
pub mod session;
pub use session::*;

pub mod prelude {
    pub use super::events::*;
    #[cfg(feature = "ierc20")]
    pub use super::ierc20::*;
    pub use super::session::*;
    pub use numeraire::prelude::*;
}
