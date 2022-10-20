#![allow(dead_code)]
use anyhow::Result;
use ethers::{abi::Abi, contract::Contract, core::types::Address, providers::Middleware};
use std::sync::Arc;

pub fn build_default<M: Middleware>(client: impl Into<Arc<M>>) -> Result<Contract<M>> {
    let address = "7a512d3609211e719737E82c7bb7271eC05Da70d";
    build_contract_string(address, client)
}

pub fn build_kovan<M: Middleware>(client: impl Into<Arc<M>>) -> Result<Contract<M>> {
    let address = "45Da67AAAAa0164cC2f8789CE1B9EAcEABE70c51";
    build_contract_string(address, client)
}

pub fn build_goerli<M: Middleware>(client: impl Into<Arc<M>>) -> Result<Contract<M>> {
    let address = "6cD8666aBB003073e45D69E5b3aa0b0Fe9CDBF91";
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

const ABI: &str = r#"[
    {
      "anonymous": false,
      "inputs": [
        {
          "indexed": true,
          "internalType": "bytes32",
          "name": "id",
          "type": "bytes32"
        },
        {
          "indexed": true,
          "internalType": "bytes32",
          "name": "pair",
          "type": "bytes32"
        },
        {
          "indexed": false,
          "internalType": "contract ERC20",
          "name": "asset",
          "type": "address"
        },
        {
          "indexed": true,
          "internalType": "address",
          "name": "taker",
          "type": "address"
        },
        {
          "indexed": false,
          "internalType": "address",
          "name": "feeTo",
          "type": "address"
        },
        {
          "indexed": false,
          "internalType": "uint256",
          "name": "feeAmt",
          "type": "uint256"
        },
        {
          "indexed": false,
          "internalType": "uint64",
          "name": "timestamp",
          "type": "uint64"
        }
      ],
      "name": "FeeTake",
      "type": "event"
    },
    {
      "anonymous": false,
      "inputs": [
        {
          "indexed": true,
          "internalType": "bytes32",
          "name": "id",
          "type": "bytes32"
        },
        {
          "indexed": true,
          "internalType": "bytes32",
          "name": "pair",
          "type": "bytes32"
        },
        {
          "indexed": true,
          "internalType": "address",
          "name": "maker",
          "type": "address"
        },
        {
          "indexed": false,
          "internalType": "contract ERC20",
          "name": "pay_gem",
          "type": "address"
        },
        {
          "indexed": false,
          "internalType": "contract ERC20",
          "name": "buy_gem",
          "type": "address"
        },
        {
          "indexed": false,
          "internalType": "uint128",
          "name": "pay_amt",
          "type": "uint128"
        },
        {
          "indexed": false,
          "internalType": "uint128",
          "name": "buy_amt",
          "type": "uint128"
        },
        {
          "indexed": false,
          "internalType": "uint64",
          "name": "timestamp",
          "type": "uint64"
        }
      ],
      "name": "LogBump",
      "type": "event"
    },
    {
      "anonymous": false,
      "inputs": [
        {
          "indexed": false,
          "internalType": "bool",
          "name": "isEnabled",
          "type": "bool"
        }
      ],
      "name": "LogBuyEnabled",
      "type": "event"
    },
    {
      "anonymous": false,
      "inputs": [
        {
          "indexed": false,
          "internalType": "address",
          "name": "keeper",
          "type": "address"
        },
        {
          "indexed": false,
          "internalType": "uint256",
          "name": "id",
          "type": "uint256"
        }
      ],
      "name": "LogDelete",
      "type": "event"
    },
    {
      "anonymous": false,
      "inputs": [
        {
          "indexed": false,
          "internalType": "address",
          "name": "keeper",
          "type": "address"
        },
        {
          "indexed": false,
          "internalType": "uint256",
          "name": "id",
          "type": "uint256"
        }
      ],
      "name": "LogInsert",
      "type": "event"
    },
    {
      "anonymous": false,
      "inputs": [
        {
          "indexed": false,
          "internalType": "string",
          "name": "lol",
          "type": "string"
        },
        {
          "indexed": false,
          "internalType": "uint256",
          "name": "input",
          "type": "uint256"
        }
      ],
      "name": "LogInt",
      "type": "event"
    },
    {
      "anonymous": false,
      "inputs": [
        {
          "indexed": false,
          "internalType": "uint256",
          "name": "id",
          "type": "uint256"
        }
      ],
      "name": "LogItemUpdate",
      "type": "event"
    },
    {
      "anonymous": false,
      "inputs": [
        {
          "indexed": true,
          "internalType": "bytes32",
          "name": "id",
          "type": "bytes32"
        },
        {
          "indexed": true,
          "internalType": "bytes32",
          "name": "pair",
          "type": "bytes32"
        },
        {
          "indexed": true,
          "internalType": "address",
          "name": "maker",
          "type": "address"
        },
        {
          "indexed": false,
          "internalType": "contract ERC20",
          "name": "pay_gem",
          "type": "address"
        },
        {
          "indexed": false,
          "internalType": "contract ERC20",
          "name": "buy_gem",
          "type": "address"
        },
        {
          "indexed": false,
          "internalType": "uint128",
          "name": "pay_amt",
          "type": "uint128"
        },
        {
          "indexed": false,
          "internalType": "uint128",
          "name": "buy_amt",
          "type": "uint128"
        },
        {
          "indexed": false,
          "internalType": "uint64",
          "name": "timestamp",
          "type": "uint64"
        }
      ],
      "name": "LogKill",
      "type": "event"
    },
    {
      "anonymous": false,
      "inputs": [
        {
          "indexed": true,
          "internalType": "bytes32",
          "name": "id",
          "type": "bytes32"
        },
        {
          "indexed": true,
          "internalType": "bytes32",
          "name": "pair",
          "type": "bytes32"
        },
        {
          "indexed": true,
          "internalType": "address",
          "name": "maker",
          "type": "address"
        },
        {
          "indexed": false,
          "internalType": "contract ERC20",
          "name": "pay_gem",
          "type": "address"
        },
        {
          "indexed": false,
          "internalType": "contract ERC20",
          "name": "buy_gem",
          "type": "address"
        },
        {
          "indexed": false,
          "internalType": "uint128",
          "name": "pay_amt",
          "type": "uint128"
        },
        {
          "indexed": false,
          "internalType": "uint128",
          "name": "buy_amt",
          "type": "uint128"
        },
        {
          "indexed": false,
          "internalType": "uint64",
          "name": "timestamp",
          "type": "uint64"
        }
      ],
      "name": "LogMake",
      "type": "event"
    },
    {
      "anonymous": false,
      "inputs": [
        {
          "indexed": false,
          "internalType": "uint256",
          "name": "id",
          "type": "uint256"
        },
        {
          "indexed": false,
          "internalType": "uint256",
          "name": "amount",
          "type": "uint256"
        }
      ],
      "name": "LogMatch",
      "type": "event"
    },
    {
      "anonymous": false,
      "inputs": [
        {
          "indexed": false,
          "internalType": "bool",
          "name": "isEnabled",
          "type": "bool"
        }
      ],
      "name": "LogMatchingEnabled",
      "type": "event"
    },
    {
      "anonymous": false,
      "inputs": [
        {
          "indexed": false,
          "internalType": "address",
          "name": "pay_gem",
          "type": "address"
        },
        {
          "indexed": false,
          "internalType": "uint256",
          "name": "min_amount",
          "type": "uint256"
        }
      ],
      "name": "LogMinSell",
      "type": "event"
    },
    {
      "anonymous": true,
      "inputs": [
        {
          "indexed": true,
          "internalType": "bytes4",
          "name": "sig",
          "type": "bytes4"
        },
        {
          "indexed": true,
          "internalType": "address",
          "name": "guy",
          "type": "address"
        },
        {
          "indexed": true,
          "internalType": "bytes32",
          "name": "foo",
          "type": "bytes32"
        },
        {
          "indexed": true,
          "internalType": "bytes32",
          "name": "bar",
          "type": "bytes32"
        },
        {
          "indexed": false,
          "internalType": "uint256",
          "name": "wad",
          "type": "uint256"
        },
        {
          "indexed": false,
          "internalType": "bytes",
          "name": "fax",
          "type": "bytes"
        }
      ],
      "name": "LogNote",
      "type": "event"
    },
    {
      "anonymous": false,
      "inputs": [
        {
          "indexed": true,
          "internalType": "address",
          "name": "authority",
          "type": "address"
        }
      ],
      "name": "LogSetAuthority",
      "type": "event"
    },
    {
      "anonymous": false,
      "inputs": [
        {
          "indexed": true,
          "internalType": "address",
          "name": "owner",
          "type": "address"
        }
      ],
      "name": "LogSetOwner",
      "type": "event"
    },
    {
      "anonymous": false,
      "inputs": [
        {
          "indexed": false,
          "internalType": "uint256",
          "name": "id",
          "type": "uint256"
        }
      ],
      "name": "LogSortedOffer",
      "type": "event"
    },
    {
      "anonymous": false,
      "inputs": [
        {
          "indexed": false,
          "internalType": "bytes32",
          "name": "id",
          "type": "bytes32"
        },
        {
          "indexed": true,
          "internalType": "bytes32",
          "name": "pair",
          "type": "bytes32"
        },
        {
          "indexed": true,
          "internalType": "address",
          "name": "maker",
          "type": "address"
        },
        {
          "indexed": false,
          "internalType": "contract ERC20",
          "name": "pay_gem",
          "type": "address"
        },
        {
          "indexed": false,
          "internalType": "contract ERC20",
          "name": "buy_gem",
          "type": "address"
        },
        {
          "indexed": true,
          "internalType": "address",
          "name": "taker",
          "type": "address"
        },
        {
          "indexed": false,
          "internalType": "uint128",
          "name": "take_amt",
          "type": "uint128"
        },
        {
          "indexed": false,
          "internalType": "uint128",
          "name": "give_amt",
          "type": "uint128"
        },
        {
          "indexed": false,
          "internalType": "uint64",
          "name": "timestamp",
          "type": "uint64"
        }
      ],
      "name": "LogTake",
      "type": "event"
    },
    {
      "anonymous": false,
      "inputs": [
        {
          "indexed": false,
          "internalType": "uint256",
          "name": "pay_amt",
          "type": "uint256"
        },
        {
          "indexed": true,
          "internalType": "address",
          "name": "pay_gem",
          "type": "address"
        },
        {
          "indexed": false,
          "internalType": "uint256",
          "name": "buy_amt",
          "type": "uint256"
        },
        {
          "indexed": true,
          "internalType": "address",
          "name": "buy_gem",
          "type": "address"
        }
      ],
      "name": "LogTrade",
      "type": "event"
    },
    {
      "anonymous": false,
      "inputs": [
        {
          "indexed": false,
          "internalType": "uint256",
          "name": "id",
          "type": "uint256"
        }
      ],
      "name": "LogUnsortedOffer",
      "type": "event"
    },
    {
      "anonymous": false,
      "inputs": [
        {
          "indexed": true,
          "internalType": "bytes32",
          "name": "id",
          "type": "bytes32"
        }
      ],
      "name": "OfferDeleted",
      "type": "event"
    },
    {
      "inputs": [],
      "name": "AqueductAddress",
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
      "name": "AqueductDistributionLive",
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
        }
      ],
      "name": "_best",
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
        }
      ],
      "name": "_dust",
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
      "name": "_head",
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
      "name": "_near",
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
      "name": "_rank",
      "outputs": [
        {
          "internalType": "uint256",
          "name": "next",
          "type": "uint256"
        },
        {
          "internalType": "uint256",
          "name": "prev",
          "type": "uint256"
        },
        {
          "internalType": "uint256",
          "name": "delb",
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
      "name": "_span",
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
          "internalType": "bytes32",
          "name": "id_",
          "type": "bytes32"
        }
      ],
      "name": "bump",
      "outputs": [],
      "stateMutability": "nonpayable",
      "type": "function"
    },
    {
      "inputs": [],
      "name": "buyEnabled",
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
      "inputs": [],
      "name": "dustId",
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
      "name": "getFeeBPS",
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
          "name": "id",
          "type": "uint256"
        }
      ],
      "name": "getOffer",
      "outputs": [
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
          "internalType": "uint256",
          "name": "id",
          "type": "uint256"
        }
      ],
      "name": "getOwner",
      "outputs": [
        {
          "internalType": "address",
          "name": "owner",
          "type": "address"
        }
      ],
      "stateMutability": "view",
      "type": "function"
    },
    {
      "inputs": [],
      "name": "getTime",
      "outputs": [
        {
          "internalType": "uint64",
          "name": "",
          "type": "uint64"
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
          "internalType": "uint256",
          "name": "id",
          "type": "uint256"
        }
      ],
      "name": "isActive",
      "outputs": [
        {
          "internalType": "bool",
          "name": "active",
          "type": "bool"
        }
      ],
      "stateMutability": "view",
      "type": "function"
    },
    {
      "inputs": [],
      "name": "isClosed",
      "outputs": [
        {
          "internalType": "bool",
          "name": "closed",
          "type": "bool"
        }
      ],
      "stateMutability": "pure",
      "type": "function"
    },
    {
      "inputs": [],
      "name": "last_offer_id",
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
      "name": "matchingEnabled",
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
          "name": "",
          "type": "uint256"
        }
      ],
      "name": "offers",
      "outputs": [
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
          "internalType": "contract ERC20",
          "name": "buy_gem",
          "type": "address"
        },
        {
          "internalType": "address",
          "name": "owner",
          "type": "address"
        },
        {
          "internalType": "uint64",
          "name": "timestamp",
          "type": "uint64"
        }
      ],
      "stateMutability": "view",
      "type": "function"
    },
    {
      "inputs": [],
      "name": "owner",
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
          "internalType": "address",
          "name": "owner_",
          "type": "address"
        }
      ],
      "name": "setOwner",
      "outputs": [],
      "stateMutability": "nonpayable",
      "type": "function"
    },
    {
      "inputs": [],
      "name": "stop",
      "outputs": [],
      "stateMutability": "nonpayable",
      "type": "function"
    },
    {
      "inputs": [],
      "name": "stopped",
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
          "internalType": "bool",
          "name": "_live",
          "type": "bool"
        },
        {
          "internalType": "address",
          "name": "_feeTo",
          "type": "address"
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
          "internalType": "uint128",
          "name": "pay_amt",
          "type": "uint128"
        },
        {
          "internalType": "uint128",
          "name": "buy_amt",
          "type": "uint128"
        }
      ],
      "name": "make",
      "outputs": [
        {
          "internalType": "bytes32",
          "name": "",
          "type": "bytes32"
        }
      ],
      "stateMutability": "nonpayable",
      "type": "function"
    },
    {
      "inputs": [
        {
          "internalType": "bytes32",
          "name": "id",
          "type": "bytes32"
        },
        {
          "internalType": "uint128",
          "name": "maxTakeAmount",
          "type": "uint128"
        }
      ],
      "name": "take",
      "outputs": [],
      "stateMutability": "nonpayable",
      "type": "function"
    },
    {
      "inputs": [
        {
          "internalType": "bytes32",
          "name": "id",
          "type": "bytes32"
        }
      ],
      "name": "kill",
      "outputs": [],
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
      "name": "offer",
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
          "internalType": "contract ERC20",
          "name": "buy_gem",
          "type": "address"
        },
        {
          "internalType": "uint256",
          "name": "pos",
          "type": "uint256"
        },
        {
          "internalType": "bool",
          "name": "matching",
          "type": "bool"
        }
      ],
      "name": "offer",
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
          "internalType": "contract ERC20",
          "name": "buy_gem",
          "type": "address"
        }
      ],
      "name": "offer",
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
          "internalType": "uint256",
          "name": "id",
          "type": "uint256"
        },
        {
          "internalType": "uint256",
          "name": "amount",
          "type": "uint256"
        }
      ],
      "name": "buy",
      "outputs": [
        {
          "internalType": "bool",
          "name": "",
          "type": "bool"
        }
      ],
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
      "name": "cancel",
      "outputs": [
        {
          "internalType": "bool",
          "name": "success",
          "type": "bool"
        }
      ],
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
      "name": "del_rank",
      "outputs": [
        {
          "internalType": "bool",
          "name": "",
          "type": "bool"
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
          "internalType": "uint256",
          "name": "dust",
          "type": "uint256"
        }
      ],
      "name": "setMinSell",
      "outputs": [
        {
          "internalType": "bool",
          "name": "",
          "type": "bool"
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
        }
      ],
      "name": "getMinSell",
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
          "internalType": "bool",
          "name": "buyEnabled_",
          "type": "bool"
        }
      ],
      "name": "setBuyEnabled",
      "outputs": [
        {
          "internalType": "bool",
          "name": "",
          "type": "bool"
        }
      ],
      "stateMutability": "nonpayable",
      "type": "function"
    },
    {
      "inputs": [
        {
          "internalType": "contract ERC20",
          "name": "sell_gem",
          "type": "address"
        },
        {
          "internalType": "contract ERC20",
          "name": "buy_gem",
          "type": "address"
        }
      ],
      "name": "getBestOffer",
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
          "name": "id",
          "type": "uint256"
        }
      ],
      "name": "getWorseOffer",
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
          "name": "id",
          "type": "uint256"
        }
      ],
      "name": "getBetterOffer",
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
          "internalType": "contract ERC20",
          "name": "sell_gem",
          "type": "address"
        },
        {
          "internalType": "contract ERC20",
          "name": "buy_gem",
          "type": "address"
        }
      ],
      "name": "getOfferCount",
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
      "name": "getFirstUnsortedOffer",
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
          "name": "id",
          "type": "uint256"
        }
      ],
      "name": "getNextUnsortedOffer",
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
          "name": "id",
          "type": "uint256"
        }
      ],
      "name": "isOfferSorted",
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
          "internalType": "contract ERC20",
          "name": "pay_gem",
          "type": "address"
        },
        {
          "internalType": "uint256",
          "name": "pay_amt",
          "type": "uint256"
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
      "name": "sellAllAmount",
      "outputs": [
        {
          "internalType": "uint256",
          "name": "fill_amt",
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
      "name": "buyAllAmount",
      "outputs": [
        {
          "internalType": "uint256",
          "name": "fill_amt",
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
          "internalType": "contract ERC20",
          "name": "pay_gem",
          "type": "address"
        },
        {
          "internalType": "uint256",
          "name": "pay_amt",
          "type": "uint256"
        }
      ],
      "name": "getBuyAmount",
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
          "name": "buy_amt",
          "type": "uint256"
        }
      ],
      "name": "getPayAmount",
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
          "internalType": "uint256",
          "name": "_newFeeBPS",
          "type": "uint256"
        }
      ],
      "name": "setFeeBPS",
      "outputs": [
        {
          "internalType": "bool",
          "name": "",
          "type": "bool"
        }
      ],
      "stateMutability": "nonpayable",
      "type": "function"
    },
    {
      "inputs": [
        {
          "internalType": "bool",
          "name": "live",
          "type": "bool"
        }
      ],
      "name": "setAqueductDistributionLive",
      "outputs": [
        {
          "internalType": "bool",
          "name": "",
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
          "name": "_Aqueduct",
          "type": "address"
        }
      ],
      "name": "setAqueductAddress",
      "outputs": [
        {
          "internalType": "bool",
          "name": "",
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
          "name": "newFeeTo",
          "type": "address"
        }
      ],
      "name": "setFeeTo",
      "outputs": [
        {
          "internalType": "bool",
          "name": "",
          "type": "bool"
        }
      ],
      "stateMutability": "nonpayable",
      "type": "function"
    },
    {
      "inputs": [],
      "name": "getFeeTo",
      "outputs": [
        {
          "internalType": "address",
          "name": "",
          "type": "address"
        }
      ],
      "stateMutability": "view",
      "type": "function"
    }
  ]
  "#;
