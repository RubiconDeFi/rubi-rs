use ethers::prelude::*;
use ethers::providers::Middleware;
use ethers::providers::Provider;
use ethers::contract::Contract; //this is the import for a contract
use ethers::core::types::Address; //address type
use ethers::providers::Http;

// Base class representation of a contract that defines the structure of a contract and provides 
// methods that can be used by Rubicon contracts 
// Current contracts are erc20, market, and router.
// Aid support is not available 

//in v1 - M is Middleware + Clone + 'static...why?
// clone trait used to create multiple independant copies of a value - not adding rn

pub struct BaseRubiContract <M: Middleware> {
    w3: Provider<Http>,
    contract: Contract<M>,
    wallet: Option<Address>,
    key: Option<String>
}

impl<M: Middleware> BaseRubiContract<M> {
    pub fn new(
        w3: Provider<M>,
        contract: Contract<M>,
        wallet: Option<Address>,
        key: Option<String>,
    ) -> Self {
        if wallet.is_some() != key.is_some() {
            eprintln!("Both a wallet and a key are required to sign transactions. Provide both or provide none.");
        }
        BaseRubiContract { w3: w3, contract: contract, wallet: wallet, key: key}
    }

    //adding this as a local var in `new` or `from_address_and_abi` prob wouldn't let me access it
    pub fn _get_signing_permission(&self) -> bool {
        return self.wallet.is_some() && self.key.is_some();
    }

    pub fn _get_chain_id(&self) -> U256 {
        return self.w3.get_chainid()
    }

    pub async fn from_address_and_abi(
        w3: Provider<Http>,
        address: Address,
        contract_abi: ethers::abi::Abi,
        wallet: Option<Address>,
        key: Option<String>
    ) -> Result<Self, String> {
        if wallet.is_some() != key.is_some() {
            return Err(String::from("Both a wallet and a key are required sign txns. Provide both or provide none")) //"Both a wallet and a key are required sign txns. Provide both or provide none".to_owned()
        }
        let contract: Contract<bool> = Contract::new(address, contract_abi, w3);
    }
}
