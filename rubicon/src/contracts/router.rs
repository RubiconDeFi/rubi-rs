#![allow(dead_code)]
use anyhow::Result;
use ethers::{abi::Abi, contract::Contract, core::types::Address, providers::Middleware};
use std::sync::Arc;

pub fn build_default<M: Middleware>(client: impl Into<Arc<M>>) -> Result<Contract<M>> {
    let address = "7Af14ADc8Aea70f063c7eA3B2C1AD0D7A59C4bFf";
    build_contract_string(address, client)
}

pub fn build_kovan<M: Middleware>(client: impl Into<Arc<M>>) -> Result<Contract<M>> {
    let address = "8316F9a27e6F632a54D1E1F21aF347D4E0Cc38Dd";
    build_contract_string(address, client)
}

pub fn build_goerli<M: Middleware>(client: impl Into<Arc<M>>) -> Result<Contract<M>> {
    let address = "6aaEd1985a0e011ca82BB5Df8ebd92063134fd7c";
    build_contract_string(address, client)
}

fn build_contract_string<M: Middleware, T: ToString>(
    address: T,
    client: impl Into<Arc<M>>,
) -> Result<Contract<M>> {
    let hx = hex::decode(address.to_string()).unwrap();
    let addr = Address::from_slice(hx.as_slice());
    build_contract(addr, client)
}

fn build_contract<M: Middleware>(
    address: Address,
    client: impl Into<Arc<M>>,
) -> Result<Contract<M>> {
    let abi: Abi = serde_json::from_str(ABI)?;
    Ok(Contract::new(address, abi, client))
}

const ABI: &str = r#"[{
    "anonymous": false,
    "inputs": [
      {
        "indexed": false,
        "internalType": "string",
        "name": "",
        "type": "string"
      },
      {
        "indexed": false,
        "internalType": "uint256",
        "name": "",
        "type": "uint256"
      }
    ],
    "name": "LogNote",
    "type": "event"
  },
  {
    "anonymous": false,
    "inputs": [
      {
        "indexed": false,
        "internalType": "uint256",
        "name": "inputAmount",
        "type": "uint256"
      },
      {
        "indexed": false,
        "internalType": "address",
        "name": "inputERC20",
        "type": "address"
      },
      {
        "indexed": false,
        "internalType": "uint256",
        "name": "hurdleBuyAmtMin",
        "type": "uint256"
      },
      {
        "indexed": false,
        "internalType": "address",
        "name": "targetERC20",
        "type": "address"
      },
      {
        "indexed": true,
        "internalType": "bytes32",
        "name": "pair",
        "type": "bytes32"
      },
      {
        "indexed": false,
        "internalType": "uint256",
        "name": "realizedFill",
        "type": "uint256"
      },
      {
        "indexed": false,
        "internalType": "address",
        "name": "recipient",
        "type": "address"
      }
    ],
    "name": "LogSwap",
    "type": "event"
  },
  {
    "stateMutability": "payable",
    "type": "fallback"
  },
  {
    "inputs": [],
    "name": "RubiconMarketAddress",
    "outputs": [
      {
        "internalType": "address",
        "name": "",
        "type": "address"
      }
    ],
    "stateMutability": "view",
    "type": "function"
  },
  {
    "inputs": [
      {
        "internalType": "uint256",
        "name": "buy_amt",
        "type": "uint256"
      },
      {
        "internalType": "contract ERC20",
        "name": "pay_gem",
        "type": "address"
      },
      {
        "internalType": "uint256",
        "name": "max_fill_amount",
        "type": "uint256"
      }
    ],
    "name": "buyAllAmountForETH",
    "outputs": [
      {
        "internalType": "uint256",
        "name": "fill",
        "type": "uint256"
      }
    ],
    "stateMutability": "nonpayable",
    "type": "function"
  },
  {
    "inputs": [
      {
        "internalType": "contract ERC20",
        "name": "buy_gem",
        "type": "address"
      },
      {
        "internalType": "uint256",
        "name": "buy_amt",
        "type": "uint256"
      },
      {
        "internalType": "uint256",
        "name": "max_fill_amount",
        "type": "uint256"
      },
      {
        "internalType": "uint256",
        "name": "expectedMarketFeeBPS",
        "type": "uint256"
      }
    ],
    "name": "buyAllAmountWithETH",
    "outputs": [
      {
        "internalType": "uint256",
        "name": "fill",
        "type": "uint256"
      }
    ],
    "stateMutability": "payable",
    "type": "function"
  },
  {
    "inputs": [
      {
        "internalType": "uint256",
        "name": "id",
        "type": "uint256"
      }
    ],
    "name": "cancelForETH",
    "outputs": [
      {
        "internalType": "bool",
        "name": "outcome",
        "type": "bool"
      }
    ],
    "stateMutability": "nonpayable",
    "type": "function"
  },
  {
    "inputs": [
      {
        "internalType": "address",
        "name": "user",
        "type": "address"
      },
      {
        "internalType": "address[]",
        "name": "targetBathTokens",
        "type": "address[]"
      },
      {
        "internalType": "address",
        "name": "token",
        "type": "address"
      }
    ],
    "name": "checkClaimAllUserBonusTokens",
    "outputs": [
      {
        "internalType": "uint256",
        "name": "earnedAcrossPools",
        "type": "uint256"
      }
    ],
    "stateMutability": "view",
    "type": "function"
  },
  {
    "inputs": [
      {
        "internalType": "uint256",
        "name": "amount",
        "type": "uint256"
      },
      {
        "internalType": "address",
        "name": "targetPool",
        "type": "address"
      }
    ],
    "name": "depositWithETH",
    "outputs": [
      {
        "internalType": "uint256",
        "name": "newShares",
        "type": "uint256"
      }
    ],
    "stateMutability": "payable",
    "type": "function"
  },
  {
    "inputs": [
      {
        "internalType": "address",
        "name": "asset",
        "type": "address"
      },
      {
        "internalType": "address",
        "name": "quote",
        "type": "address"
      }
    ],
    "name": "getBestOfferAndInfo",
    "outputs": [
      {
        "internalType": "uint256",
        "name": "",
        "type": "uint256"
      },
      {
        "internalType": "uint256",
        "name": "",
        "type": "uint256"
      },
      {
        "internalType": "contract ERC20",
        "name": "",
        "type": "address"
      },
      {
        "internalType": "uint256",
        "name": "",
        "type": "uint256"
      },
      {
        "internalType": "contract ERC20",
        "name": "",
        "type": "address"
      }
    ],
    "stateMutability": "view",
    "type": "function"
  },
  {
    "inputs": [
      {
        "internalType": "contract ERC20",
        "name": "asset",
        "type": "address"
      },
      {
        "internalType": "contract ERC20",
        "name": "quote",
        "type": "address"
      },
      {
        "internalType": "uint256",
        "name": "topNOrders",
        "type": "uint256"
      }
    ],
    "name": "getBookFromPair",
    "outputs": [
      {
        "internalType": "uint256[3][]",
        "name": "",
        "type": "uint256[3][]"
      },
      {
        "internalType": "uint256[3][]",
        "name": "",
        "type": "uint256[3][]"
      },
      {
        "internalType": "uint256",
        "name": "",
        "type": "uint256"
      }
    ],
    "stateMutability": "view",
    "type": "function"
  },
  {
    "inputs": [
      {
        "internalType": "uint256",
        "name": "pay_amt",
        "type": "uint256"
      },
      {
        "internalType": "uint256",
        "name": "buy_amt_min",
        "type": "uint256"
      },
      {
        "internalType": "address[]",
        "name": "route",
        "type": "address[]"
      },
      {
        "internalType": "uint256",
        "name": "expectedMarketFeeBPS",
        "type": "uint256"
      }
    ],
    "name": "getExpectedSwapFill",
    "outputs": [
      {
        "internalType": "uint256",
        "name": "fill_amt",
        "type": "uint256"
      }
    ],
    "stateMutability": "view",
    "type": "function"
  },
  {
    "inputs": [
      {
        "internalType": "contract ERC20",
        "name": "buy_gem",
        "type": "address"
      },
      {
        "internalType": "contract ERC20",
        "name": "pay_gem",
        "type": "address"
      },
      {
        "internalType": "uint256",
        "name": "max_fill_amount",
        "type": "uint256"
      }
    ],
    "name": "maxBuyAllAmount",
    "outputs": [
      {
        "internalType": "uint256",
        "name": "fill",
        "type": "uint256"
      }
    ],
    "stateMutability": "nonpayable",
    "type": "function"
  },
  {
    "inputs": [
      {
        "internalType": "contract ERC20",
        "name": "pay_gem",
        "type": "address"
      },
      {
        "internalType": "contract ERC20",
        "name": "buy_gem",
        "type": "address"
      },
      {
        "internalType": "uint256",
        "name": "min_fill_amount",
        "type": "uint256"
      }
    ],
    "name": "maxSellAllAmount",
    "outputs": [
      {
        "internalType": "uint256",
        "name": "fill",
        "type": "uint256"
      }
    ],
    "stateMutability": "nonpayable",
    "type": "function"
  },
  {
    "inputs": [
      {
        "internalType": "uint256",
        "name": "pay_amt",
        "type": "uint256"
      },
      {
        "internalType": "contract ERC20",
        "name": "pay_gem",
        "type": "address"
      },
      {
        "internalType": "uint256",
        "name": "buy_amt",
        "type": "uint256"
      },
      {
        "internalType": "uint256",
        "name": "pos",
        "type": "uint256"
      }
    ],
    "name": "offerForETH",
    "outputs": [
      {
        "internalType": "uint256",
        "name": "",
        "type": "uint256"
      }
    ],
    "stateMutability": "nonpayable",
    "type": "function"
  },
  {
    "inputs": [
      {
        "internalType": "uint256",
        "name": "pay_amt",
        "type": "uint256"
      },
      {
        "internalType": "uint256",
        "name": "buy_amt",
        "type": "uint256"
      },
      {
        "internalType": "contract ERC20",
        "name": "buy_gem",
        "type": "address"
      },
      {
        "internalType": "uint256",
        "name": "pos",
        "type": "uint256"
      }
    ],
    "name": "offerWithETH",
    "outputs": [
      {
        "internalType": "uint256",
        "name": "",
        "type": "uint256"
      }
    ],
    "stateMutability": "payable",
    "type": "function"
  },
  {
    "inputs": [
      {
        "internalType": "address",
        "name": "_theTrap",
        "type": "address"
      },
      {
        "internalType": "address payable",
        "name": "_weth",
        "type": "address"
      }
    ],
    "name": "startErUp",
    "outputs": [],
    "stateMutability": "nonpayable",
    "type": "function"
  },
  {
    "inputs": [],
    "name": "started",
    "outputs": [
      {
        "internalType": "bool",
        "name": "",
        "type": "bool"
      }
    ],
    "stateMutability": "view",
    "type": "function"
  },
  {
    "inputs": [
      {
        "internalType": "uint256",
        "name": "pay_amt",
        "type": "uint256"
      },
      {
        "internalType": "uint256",
        "name": "buy_amt_min",
        "type": "uint256"
      },
      {
        "internalType": "address[]",
        "name": "route",
        "type": "address[]"
      },
      {
        "internalType": "uint256",
        "name": "expectedMarketFeeBPS",
        "type": "uint256"
      }
    ],
    "name": "swap",
    "outputs": [
      {
        "internalType": "uint256",
        "name": "",
        "type": "uint256"
      }
    ],
    "stateMutability": "nonpayable",
    "type": "function"
  },
  {
    "inputs": [
      {
        "internalType": "uint256",
        "name": "pay_amt",
        "type": "uint256"
      },
      {
        "internalType": "uint256",
        "name": "buy_amt_min",
        "type": "uint256"
      },
      {
        "internalType": "address[]",
        "name": "route",
        "type": "address[]"
      },
      {
        "internalType": "uint256",
        "name": "expectedMarketFeeBPS",
        "type": "uint256"
      }
    ],
    "name": "swapForETH",
    "outputs": [
      {
        "internalType": "uint256",
        "name": "fill",
        "type": "uint256"
      }
    ],
    "stateMutability": "payable",
    "type": "function"
  },
  {
    "inputs": [
      {
        "internalType": "uint256",
        "name": "pay_amt",
        "type": "uint256"
      },
      {
        "internalType": "uint256",
        "name": "buy_amt_min",
        "type": "uint256"
      },
      {
        "internalType": "address[]",
        "name": "route",
        "type": "address[]"
      },
      {
        "internalType": "uint256",
        "name": "expectedMarketFeeBPS",
        "type": "uint256"
      }
    ],
    "name": "swapWithETH",
    "outputs": [
      {
        "internalType": "uint256",
        "name": "",
        "type": "uint256"
      }
    ],
    "stateMutability": "payable",
    "type": "function"
  },
  {
    "inputs": [
      {
        "internalType": "address",
        "name": "",
        "type": "address"
      },
      {
        "internalType": "uint256",
        "name": "",
        "type": "uint256"
      }
    ],
    "name": "userNativeAssetOrders",
    "outputs": [
      {
        "internalType": "uint256",
        "name": "",
        "type": "uint256"
      }
    ],
    "stateMutability": "view",
    "type": "function"
  },
  {
    "inputs": [],
    "name": "wethAddress",
    "outputs": [
      {
        "internalType": "address payable",
        "name": "",
        "type": "address"
      }
    ],
    "stateMutability": "view",
    "type": "function"
  },
  {
    "inputs": [
      {
        "internalType": "uint256",
        "name": "shares",
        "type": "uint256"
      },
      {
        "internalType": "address",
        "name": "targetPool",
        "type": "address"
      }
    ],
    "name": "withdrawForETH",
    "outputs": [
      {
        "internalType": "uint256",
        "name": "withdrawnWETH",
        "type": "uint256"
      }
    ],
    "stateMutability": "payable",
    "type": "function"
  },
  {
    "stateMutability": "payable",
    "type": "receive"
  }
]"#;
