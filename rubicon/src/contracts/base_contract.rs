use std::any::Any;
use ethers::providers::Middleware;
use ethers::providers::Provider;
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
        w3: Provider<Http>,
        contract: Contract,
        wallet: Option<Address>,
        key: Option<str>,
    ) -> Result<Self, String> {
            if wallet.is_some() != key.is_some() {
                return Err(String::from("Both a wallet and a key are required sign txns. Provide both or provide none")) //"Both a wallet and a key are required sign txns. Provide both or provide none".to_owned()
            }
            
            // I added getter functions for these so they can be used
            // let chain_id: U256 = w3.get_chainid().await.map_err(|e| e.to_string())?;
            // let signing_permissions: bool = wallet.is_some() && key.is_some();

            Ok(BaseContract{
                w3,
                contract,
                wallet,
                key,
            })
        }
    pub async fn from_address_and_abi(
        w3: Provider<Http>,
        address: Address,
        contract_abi: ethers::contract::Abigen,
        wallet: Option<Address>,
        key: Option<str>
    ) -> Result<Self, String> {
            if wallet.is_some() != key.is_some() {
                return Err(String::from("Both a wallet and a key are required sign txns. Provide both or provide none")) //"Both a wallet and a key are required sign txns. Provide both or provide none".to_owned()
            }
            
            let contract: Contract<Provider<Http>> = Contract::new(address, contract_abi, w3.clone()); //unkown provider need to fix that across 
            let base_contract = Self::new(w3, contract, wallet, key);
            let signing_permissions: bool = wallet.is_some() && key.is_some();
            
            Ok(BaseContract(
                ..base_contract //struct update syntax allowing to copy remaining fields from an exsiting struct (BaseContract) into a new instance also BaseContract
                //what about signing permissions
            )) 
    }

    //adding this as a local var in `new` or `from_address_and_abi` prob wouldn't let me access it
    fn _get_signing_permission(&self) -> bool {
        return self.wallet.is_some() && self.key.is_some();
    }

    fn _get_chain_id(&self) -> U256 {
        return self.w3.get_chainid();
    }
}

