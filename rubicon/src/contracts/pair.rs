#![allow(dead_code)]
use anyhow::Result;
use ethers::{abi::Abi, contract::Contract, core::types::Address, providers::Middleware};
use std::sync::Arc;

pub fn build_default<M: Middleware>(client: impl Into<Arc<M>>) -> Result<Contract<M>> {
    let address = "F8780E00Ce8ed2e79aeC10908a169900eD1D4AFe";
    build_contract_string(address, client)
}

pub fn build_kovan<M: Middleware>(client: impl Into<Arc<M>>) -> Result<Contract<M>> {
    let address = "45Da67AAAAa0164cC2f8789CE1B9EAcEABE70c51";
    build_contract_string(address, client)
}

pub fn build_goerli<M: Middleware>(client: impl Into<Arc<M>>) -> Result<Contract<M>> {
    let address = "9dBf17d518f722B5Aae5573D808B94024b635529";
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
    Ok(Contract::new(address, abi, client.into()))
}

const ABI: &str = r#"[
    {
      "anonymous": false,
      "inputs": [
        {
          "indexed": false,
          "internalType": "address",
          "name": "strategist",
          "type": "address"
        },
        {
          "indexed": false,
          "internalType": "uint256[]",
          "name": "trades",
          "type": "uint256[]"
        }
      ],
      "name": "LogBatchMarketMakingTrades",
      "type": "event"
    },
    {
      "anonymous": false,
      "inputs": [
        {
          "indexed": false,
          "internalType": "address",
          "name": "strategist",
          "type": "address"
        },
        {
          "indexed": false,
          "internalType": "uint256[]",
          "name": "scrubbedOfferIDs",
          "type": "uint256[]"
        }
      ],
      "name": "LogBatchRequoteOffers",
      "type": "event"
    },
    {
      "anonymous": false,
      "inputs": [
        {
          "indexed": false,
          "internalType": "address",
          "name": "strategist",
          "type": "address"
        },
        {
          "indexed": false,
          "internalType": "address",
          "name": "underlyingAsset",
          "type": "address"
        },
        {
          "indexed": false,
          "internalType": "address",
          "name": "quoteAddress",
          "type": "address"
        },
        {
          "indexed": false,
          "internalType": "uint256",
          "name": "assetRebalAmt",
          "type": "uint256"
        },
        {
          "indexed": false,
          "internalType": "uint256",
          "name": "quoteRebalAmt",
          "type": "uint256"
        },
        {
          "indexed": false,
          "internalType": "uint256",
          "name": "strategistReward",
          "type": "uint256"
        }
      ],
      "name": "LogRebalancePair",
      "type": "event"
    },
    {
      "anonymous": false,
      "inputs": [
        {
          "indexed": false,
          "internalType": "address",
          "name": "strategist",
          "type": "address"
        },
        {
          "indexed": false,
          "internalType": "uint256",
          "name": "scrubbedOfferID",
          "type": "uint256"
        },
        {
          "indexed": false,
          "internalType": "uint256",
          "name": "newOfferID",
          "type": "uint256"
        }
      ],
      "name": "LogRequote",
      "type": "event"
    },
    {
      "anonymous": false,
      "inputs": [
        {
          "indexed": false,
          "internalType": "uint256",
          "name": "strategistIDScrubbed",
          "type": "uint256"
        },
        {
          "indexed": false,
          "internalType": "uint256",
          "name": "assetFill",
          "type": "uint256"
        },
        {
          "indexed": false,
          "internalType": "address",
          "name": "assetAddress",
          "type": "address"
        },
        {
          "indexed": false,
          "internalType": "address",
          "name": "bathAssetAddress",
          "type": "address"
        },
        {
          "indexed": false,
          "internalType": "uint256",
          "name": "quoteFill",
          "type": "uint256"
        },
        {
          "indexed": false,
          "internalType": "address",
          "name": "quoteAddress",
          "type": "address"
        },
        {
          "indexed": false,
          "internalType": "address",
          "name": "bathQuoteAddress",
          "type": "address"
        }
      ],
      "name": "LogScrubbedStratTrade",
      "type": "event"
    },
    {
      "anonymous": false,
      "inputs": [
        {
          "indexed": false,
          "internalType": "address",
          "name": "strategist",
          "type": "address"
        },
        {
          "indexed": false,
          "internalType": "address",
          "name": "asset",
          "type": "address"
        },
        {
          "indexed": false,
          "internalType": "uint256",
          "name": "amountOfReward",
          "type": "uint256"
        },
        {
          "indexed": false,
          "internalType": "uint256",
          "name": "timestamp",
          "type": "uint256"
        }
      ],
      "name": "LogStrategistRewardClaim",
      "type": "event"
    },
    {
      "anonymous": false,
      "inputs": [
        {
          "indexed": false,
          "internalType": "uint256",
          "name": "strategistTradeID",
          "type": "uint256"
        },
        {
          "indexed": false,
          "internalType": "bytes32",
          "name": "askId",
          "type": "bytes32"
        },
        {
          "indexed": false,
          "internalType": "bytes32",
          "name": "bidId",
          "type": "bytes32"
        },
        {
          "indexed": false,
          "internalType": "address",
          "name": "askAsset",
          "type": "address"
        },
        {
          "indexed": false,
          "internalType": "address",
          "name": "bidAsset",
          "type": "address"
        },
        {
          "indexed": false,
          "internalType": "uint256",
          "name": "timestamp",
          "type": "uint256"
        },
        {
          "indexed": false,
          "internalType": "address",
          "name": "strategist",
          "type": "address"
        }
      ],
      "name": "LogStrategistTrade",
      "type": "event"
    },
    {
      "anonymous": false,
      "inputs": [
        {
          "indexed": false,
          "internalType": "address",
          "name": "strategist",
          "type": "address"
        },
        {
          "indexed": false,
          "internalType": "address",
          "name": "targetPool",
          "type": "address"
        },
        {
          "indexed": false,
          "internalType": "address[]",
          "name": "assets",
          "type": "address[]"
        },
        {
          "indexed": false,
          "internalType": "uint24[]",
          "name": "fees",
          "type": "uint24[]"
        },
        {
          "indexed": false,
          "internalType": "uint256",
          "name": "inputAmount",
          "type": "uint256"
        }
      ],
      "name": "LogTailOffMultiv0",
      "type": "event"
    },
    {
      "anonymous": false,
      "inputs": [
        {
          "indexed": false,
          "internalType": "address",
          "name": "strategist",
          "type": "address"
        },
        {
          "indexed": false,
          "internalType": "address",
          "name": "targetPool",
          "type": "address"
        },
        {
          "indexed": false,
          "internalType": "address",
          "name": "tokenToHandle",
          "type": "address"
        },
        {
          "indexed": false,
          "internalType": "address",
          "name": "targetToken",
          "type": "address"
        },
        {
          "indexed": false,
          "internalType": "uint256",
          "name": "poolFee",
          "type": "uint256"
        },
        {
          "indexed": false,
          "internalType": "uint256",
          "name": "inputAmount",
          "type": "uint256"
        }
      ],
      "name": "LogTailOffv0",
      "type": "event"
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
      "inputs": [],
      "name": "bathHouse",
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
      "inputs": [],
      "name": "deprecatedStorageVarKept4Proxy",
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
      "name": "initialized",
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
          "internalType": "address",
          "name": "",
          "type": "address"
        },
        {
          "internalType": "address",
          "name": "",
          "type": "address"
        },
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
      "name": "outOffersByStrategist",
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
      "inputs": [
        {
          "internalType": "address",
          "name": "",
          "type": "address"
        },
        {
          "internalType": "address",
          "name": "",
          "type": "address"
        }
      ],
      "name": "strategist2Fills",
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
      "inputs": [
        {
          "internalType": "uint256",
          "name": "",
          "type": "uint256"
        }
      ],
      "name": "strategistTrades",
      "outputs": [
        {
          "internalType": "uint256",
          "name": "askId",
          "type": "uint256"
        },
        {
          "internalType": "uint256",
          "name": "askPayAmt",
          "type": "uint256"
        },
        {
          "internalType": "address",
          "name": "askAsset",
          "type": "address"
        },
        {
          "internalType": "uint256",
          "name": "bidId",
          "type": "uint256"
        },
        {
          "internalType": "uint256",
          "name": "bidPayAmt",
          "type": "uint256"
        },
        {
          "internalType": "address",
          "name": "bidAsset",
          "type": "address"
        },
        {
          "internalType": "address",
          "name": "strategist",
          "type": "address"
        },
        {
          "internalType": "uint256",
          "name": "timestamp",
          "type": "uint256"
        }
      ],
      "stateMutability": "view",
      "type": "function"
    },
    {
      "inputs": [
        {
          "internalType": "address",
          "name": "",
          "type": "address"
        }
      ],
      "name": "totalFillsPerAsset",
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
      "inputs": [
        {
          "internalType": "uint256",
          "name": "_maxOrderSizeBPS",
          "type": "uint256"
        },
        {
          "internalType": "int128",
          "name": "_shapeCoefNum",
          "type": "int128"
        }
      ],
      "name": "initialize",
      "outputs": [],
      "stateMutability": "nonpayable",
      "type": "function"
    },
    {
      "inputs": [
        {
          "internalType": "address[2]",
          "name": "tokenPair",
          "type": "address[2]"
        },
        {
          "internalType": "uint256",
          "name": "askNumerator",
          "type": "uint256"
        },
        {
          "internalType": "uint256",
          "name": "askDenominator",
          "type": "uint256"
        },
        {
          "internalType": "uint256",
          "name": "bidNumerator",
          "type": "uint256"
        },
        {
          "internalType": "uint256",
          "name": "bidDenominator",
          "type": "uint256"
        }
      ],
      "name": "placeMarketMakingTrades",
      "outputs": [
        {
          "internalType": "uint256",
          "name": "id",
          "type": "uint256"
        }
      ],
      "stateMutability": "nonpayable",
      "type": "function"
    },
    {
      "inputs": [
        {
          "internalType": "address[2]",
          "name": "tokenPair",
          "type": "address[2]"
        },
        {
          "internalType": "uint256[]",
          "name": "askNumerators",
          "type": "uint256[]"
        },
        {
          "internalType": "uint256[]",
          "name": "askDenominators",
          "type": "uint256[]"
        },
        {
          "internalType": "uint256[]",
          "name": "bidNumerators",
          "type": "uint256[]"
        },
        {
          "internalType": "uint256[]",
          "name": "bidDenominators",
          "type": "uint256[]"
        }
      ],
      "name": "batchMarketMakingTrades",
      "outputs": [],
      "stateMutability": "nonpayable",
      "type": "function"
    },
    {
      "inputs": [
        {
          "internalType": "uint256",
          "name": "id",
          "type": "uint256"
        },
        {
          "internalType": "address[2]",
          "name": "tokenPair",
          "type": "address[2]"
        },
        {
          "internalType": "uint256",
          "name": "askNumerator",
          "type": "uint256"
        },
        {
          "internalType": "uint256",
          "name": "askDenominator",
          "type": "uint256"
        },
        {
          "internalType": "uint256",
          "name": "bidNumerator",
          "type": "uint256"
        },
        {
          "internalType": "uint256",
          "name": "bidDenominator",
          "type": "uint256"
        }
      ],
      "name": "requote",
      "outputs": [],
      "stateMutability": "nonpayable",
      "type": "function"
    },
    {
      "inputs": [
        {
          "internalType": "uint256[]",
          "name": "ids",
          "type": "uint256[]"
        },
        {
          "internalType": "address[2]",
          "name": "tokenPair",
          "type": "address[2]"
        },
        {
          "internalType": "uint256[]",
          "name": "askNumerators",
          "type": "uint256[]"
        },
        {
          "internalType": "uint256[]",
          "name": "askDenominators",
          "type": "uint256[]"
        },
        {
          "internalType": "uint256[]",
          "name": "bidNumerators",
          "type": "uint256[]"
        },
        {
          "internalType": "uint256[]",
          "name": "bidDenominators",
          "type": "uint256[]"
        }
      ],
      "name": "batchRequoteOffers",
      "outputs": [],
      "stateMutability": "nonpayable",
      "type": "function"
    },
    {
      "inputs": [
        {
          "internalType": "uint256",
          "name": "assetRebalAmt",
          "type": "uint256"
        },
        {
          "internalType": "uint256",
          "name": "quoteRebalAmt",
          "type": "uint256"
        },
        {
          "internalType": "address",
          "name": "_underlyingAsset",
          "type": "address"
        },
        {
          "internalType": "address",
          "name": "_underlyingQuote",
          "type": "address"
        }
      ],
      "name": "rebalancePair",
      "outputs": [],
      "stateMutability": "nonpayable",
      "type": "function"
    },
    {
      "inputs": [
        {
          "internalType": "address",
          "name": "targetPool",
          "type": "address"
        },
        {
          "internalType": "address",
          "name": "tokenToHandle",
          "type": "address"
        },
        {
          "internalType": "address",
          "name": "targetToken",
          "type": "address"
        },
        {
          "internalType": "address",
          "name": "_stratUtil",
          "type": "address"
        },
        {
          "internalType": "uint256",
          "name": "amount",
          "type": "uint256"
        },
        {
          "internalType": "uint256",
          "name": "hurdle",
          "type": "uint256"
        },
        {
          "internalType": "uint24",
          "name": "_poolFee",
          "type": "uint24"
        }
      ],
      "name": "tailOff",
      "outputs": [],
      "stateMutability": "nonpayable",
      "type": "function"
    },
    {
      "inputs": [
        {
          "internalType": "address",
          "name": "targetPool",
          "type": "address"
        },
        {
          "internalType": "uint256",
          "name": "amount",
          "type": "uint256"
        },
        {
          "internalType": "address[]",
          "name": "assets",
          "type": "address[]"
        },
        {
          "internalType": "uint24[]",
          "name": "fees",
          "type": "uint24[]"
        },
        {
          "internalType": "uint256",
          "name": "hurdle",
          "type": "uint256"
        },
        {
          "internalType": "address",
          "name": "_stratUtil",
          "type": "address"
        }
      ],
      "name": "tailOffMulti",
      "outputs": [],
      "stateMutability": "nonpayable",
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
      "name": "scrubStrategistTrade",
      "outputs": [],
      "stateMutability": "nonpayable",
      "type": "function"
    },
    {
      "inputs": [
        {
          "internalType": "uint256[]",
          "name": "ids",
          "type": "uint256[]"
        }
      ],
      "name": "scrubStrategistTrades",
      "outputs": [],
      "stateMutability": "nonpayable",
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
      "name": "strategistBootyClaim",
      "outputs": [],
      "stateMutability": "nonpayable",
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
        },
        {
          "internalType": "address",
          "name": "strategist",
          "type": "address"
        },
        {
          "internalType": "uint256[]",
          "name": "ids",
          "type": "uint256[]"
        }
      ],
      "name": "flushDeprecatedStratOrders",
      "outputs": [],
      "stateMutability": "nonpayable",
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
        },
        {
          "internalType": "address",
          "name": "strategist",
          "type": "address"
        }
      ],
      "name": "getOutstandingStrategistTrades",
      "outputs": [
        {
          "internalType": "uint256[]",
          "name": "",
          "type": "uint256[]"
        }
      ],
      "stateMutability": "view",
      "type": "function"
    }
  ]
  "#;
