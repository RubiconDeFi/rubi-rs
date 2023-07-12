use ethers::providers::Middleware;
use ethers::signers::Signer;
// import libraries
use ethers::{abi::Address, signers::Signer::address};
use ethers::prelude::*;
use ethers::contract::Contract; //this is the import for a contract
use ethers::core::types::Address; //address type
use ethers::core::types::Chain; //rust has built in chain IDs

// Base class representation of a contract that defines the structure of a contract and provides 
// methods that can be used by Rubicon contracts 
// Current contracts are erc20, market, and router.
// Aid support is not available 
pub struct BaseContract {
    w3: Provider, //not sure if this is supposed to be middleware. also might need <Http> type? idk
    contract: Contract, //contract instance 
    wallet: Option<Address>, //optional signer wallet address
    key: Option<str> //pk of signer 
}

// implementation of base contract
impl BaseContract {
    pub fn new(
        w3: Provider,
        contract: Contract,
        wallet: Option<Address>,
        key: Option<str>,
    ) -> Result<Self, String> {
            if wallet.is_some() != key.is_some() {
                return Err(String::from("Both a wallet and a key are required sign txns. Provide both or provide none")) //"Both a wallet and a key are required sign txns. Provide both or provide none".to_owned()
            }
            let chain_id = w3.get_chainid().await.map_err(|e| e.to_string())?;
            Ok(BaseContract{
                w3,
                contract,
                wallet,
                key
            })
        }
}