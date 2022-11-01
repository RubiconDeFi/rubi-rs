#![allow(dead_code)]

use anyhow::Result;

use ethers::{
    contract::Contract,
    core::types::{Address, Chain, U256},
    prelude::builders::ContractCall,
    providers::Middleware,
};
use std::sync::Arc;
use tracing::{event, instrument, Level};

/**
 * This represents an ERC-20 token living on chain. It implements the IERC-20 specification from [OpenZeppelin](https://docs.openzeppelin.com/contracts/2.x/api/token/erc20#IERC20)
 */
pub struct Token<M: Middleware + 'static> {
    chain: Chain,
    coin: Contract<M>,
}

impl<M: Middleware + 'static> Token<M> {
    pub fn new(client: Arc<M>, chain: Chain, address: Address) -> Self {
        Self {
            chain: chain,
            coin: crate::contracts::ierc_20::build_contract(address, client).unwrap(),
        }
    }

    pub fn new_from_string<T: ToString>(client: Arc<M>, chain: Chain, address: T) -> Result<Self> {
        Ok(Self {
            chain: chain,
            coin: crate::contracts::ierc_20::build_contract_string(address, client)?,
        })
    }

    /// Returns a reference to the Chain that this Token is on.
    pub fn chain(&self) -> &Chain {
        &self.chain
    }

    /// Returns a reference to the underlying ERC-20 contract
    pub fn contract(&self) -> &Contract<M> {
        &self.coin
    }

    /// This is the address of the underlying ERC-20 contract
    pub fn address(&self) -> Address {
        self.coin.address()
    }

    /// Are we on a legacy chain (pre EIP-1559)? If so, we have to use legacy TX calls...
    pub fn is_legacy(&self) -> bool {
        self.chain().is_legacy()
    }

    // IERC-20 function implementations

    /// Returns the amount of tokens in existence. This function is a View function.
    #[instrument(level = "debug", skip_all)]
    pub async fn total_supply(&self) -> Result<U256> {
        Ok(self
            .contract()
            .method::<_, U256>("totalSupply", ())?
            .call()
            .await?)
    }

    /// Returns the amount of tokens owned by `account`. This function is a View function.
    #[instrument(level = "debug", skip(self))]
    pub async fn balance_of(&self, account: Address) -> Result<U256> {
        Ok(self
            .contract()
            .method::<_, U256>("balanceOf", (account,))?
            .call()
            .await?)
    }

    /// Returns a contract call that transfers `amount` of tokens from the caller's account to the `receiver`'s account. It is the user's responsibility to execute the contract call with either the methods provided in ethers-rs.
    #[instrument(level = "debug", skip(self))]
    pub fn transfer(&self, receiver: Address, amount: U256) -> Result<ContractCall<M, bool>> {
        let mut tx = self
            .contract()
            .method::<_, bool>("transfer", (receiver, amount))?;
        if self.is_legacy() {
            tx = tx.legacy();
        }
        Ok(tx)
    }

    /// Returns the amount of `owner`'s tokens that `spender` is allowed to spend. This function is a View function.
    #[instrument(level = "debug", skip(self))]
    pub async fn allowance(&self, owner: Address, spender: Address) -> Result<U256> {
        Ok(self
            .contract()
            .method::<_, U256>("allowance", (owner, spender))?
            .call()
            .await?)
    }

    /// Returns a contract call that sets `amount` as the allowance of `spender` of the caller's tokens. It is the user's responsibility to execute the contract call with either the methods provided in ethers-rs.
    #[instrument(level = "debug", skip(self))]
    pub fn approve(&self, spender: Address, amount: U256) -> Result<ContractCall<M, bool>> {
        let mut tx = self.coin.method("approve", (spender, amount))?;
        if self.is_legacy() {
            tx = tx.legacy();
        }
        Ok(tx)
    }

    /// Returns a contract call that transfers `amount` of tokens from `sender` to `receiver` using the allowance mechanism. It is the user's responsibility to execute the contract call with either the methods provided in ethers-rs.
    #[instrument(level = "debug", skip(self))]
    pub fn transfer_from(
        &self,
        from: Address,
        to: Address,
        amount: U256,
    ) -> Result<ContractCall<M, bool>> {
        let mut tx = self.coin.method("transferFrom", (from, to, amount))?;
        if self.is_legacy() {
            tx = tx.legacy();
        }
        Ok(tx)
    }
}
