#![allow(dead_code)]
use anyhow::Result;
use ethers::{abi::Abi, contract::Contract, core::types::Address, providers::Middleware};
use std::sync::Arc;

/// Builds the default BathHouse contract on OP Mainnet
pub fn build_default<M: Middleware>(client: impl Into<Arc<M>>) -> Result<Contract<M>> {
    let address = "203328C161D23dCEee3E439DeEB25cA19e2c4984";
    let hx = hex::decode(address).unwrap();
    let addr = Address::from_slice(hx.as_slice());
    build_contract(addr, client)
}

pub fn build_kovan<M: Middleware>(client: impl Into<Arc<M>>) -> Result<Contract<M>> {
    let address = "68B5fBd7CEFEE3076e4101920b13C9Cc1A6cbF0e";
    build_contract_string(address, client)
}

pub fn build_goerli<M: Middleware>(client: impl Into<Arc<M>>) -> Result<Contract<M>> {
    let address = "1229036F63679B61910CB1463e5BB57f68D19bb2";
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
          "indexed": false,
          "internalType": "address",
          "name": "underlyingToken",
          "type": "address"
        },
        {
          "indexed": false,
          "internalType": "address",
          "name": "bathTokenAddress",
          "type": "address"
        },
        {
          "indexed": false,
          "internalType": "address",
          "name": "bathTokenFeeAdmin",
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
          "name": "bathTokenCreator",
          "type": "address"
        }
      ],
      "name": "LogNewBathToken",
      "type": "event"
    },
    {
      "anonymous": false,
      "inputs": [
        {
          "indexed": false,
          "internalType": "contract IERC20",
          "name": "newERC20Underlying",
          "type": "address"
        },
        {
          "indexed": false,
          "internalType": "address",
          "name": "spawnedBathToken",
          "type": "address"
        },
        {
          "indexed": false,
          "internalType": "uint256",
          "name": "initialNewBathTokenDeposit",
          "type": "uint256"
        },
        {
          "indexed": false,
          "internalType": "contract IERC20",
          "name": "pairedExistingAsset",
          "type": "address"
        },
        {
          "indexed": false,
          "internalType": "address",
          "name": "pairedExistingBathToken",
          "type": "address"
        },
        {
          "indexed": false,
          "internalType": "uint256",
          "name": "pairedBathTokenDeposit",
          "type": "uint256"
        },
        {
          "indexed": false,
          "internalType": "address",
          "name": "signaler",
          "type": "address"
        }
      ],
      "name": "LogOpenCreationSignal",
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
      "name": "admin",
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
      "name": "approvedPairContract",
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
          "name": "",
          "type": "address"
        }
      ],
      "name": "approvedStrategists",
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
      "name": "bpsToStrategists",
      "outputs": [
        {
          "internalType": "uint8",
          "name": "",
          "type": "uint8"
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
      "inputs": [],
      "name": "name",
      "outputs": [
        {
          "internalType": "string",
          "name": "",
          "type": "string"
        }
      ],
      "stateMutability": "view",
      "type": "function"
    },
    {
      "inputs": [],
      "name": "newBathTokenImplementation",
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
      "name": "permissionedStrategists",
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
      "name": "proxyManager",
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
      "name": "reserveRatio",
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
      "name": "timeDelay",
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
      "name": "tokenToBathToken",
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
          "name": "market",
          "type": "address"
        },
        {
          "internalType": "uint256",
          "name": "_reserveRatio",
          "type": "uint256"
        },
        {
          "internalType": "uint256",
          "name": "_timeDelay",
          "type": "uint256"
        },
        {
          "internalType": "address",
          "name": "_newBathTokenImplementation",
          "type": "address"
        },
        {
          "internalType": "address",
          "name": "_proxyAdmin",
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
          "internalType": "contract IERC20",
          "name": "newBathTokenUnderlying",
          "type": "address"
        },
        {
          "internalType": "uint256",
          "name": "initialLiquidityNew",
          "type": "uint256"
        },
        {
          "internalType": "contract IERC20",
          "name": "desiredPairedAsset",
          "type": "address"
        },
        {
          "internalType": "uint256",
          "name": "initialLiquidityExistingBathToken",
          "type": "uint256"
        }
      ],
      "name": "openBathTokenSpawnAndSignal",
      "outputs": [
        {
          "internalType": "address",
          "name": "newBathToken",
          "type": "address"
        }
      ],
      "stateMutability": "nonpayable",
      "type": "function"
    },
    {
      "inputs": [
        {
          "internalType": "contract IERC20",
          "name": "underlyingERC20",
          "type": "address"
        },
        {
          "internalType": "address",
          "name": "_feeAdmin",
          "type": "address"
        }
      ],
      "name": "createBathToken",
      "outputs": [
        {
          "internalType": "address",
          "name": "newBathTokenAddress",
          "type": "address"
        }
      ],
      "stateMutability": "nonpayable",
      "type": "function"
    },
    {
      "inputs": [
        {
          "internalType": "contract IERC20",
          "name": "overwriteERC20",
          "type": "address"
        },
        {
          "internalType": "address",
          "name": "newBathToken",
          "type": "address"
        }
      ],
      "name": "adminWriteBathToken",
      "outputs": [],
      "stateMutability": "nonpayable",
      "type": "function"
    },
    {
      "inputs": [
        {
          "internalType": "address",
          "name": "_bathPairAddress",
          "type": "address"
        },
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
      "name": "initBathPair",
      "outputs": [
        {
          "internalType": "address",
          "name": "newPair",
          "type": "address"
        }
      ],
      "stateMutability": "nonpayable",
      "type": "function"
    },
    {
      "inputs": [
        {
          "internalType": "address",
          "name": "newAdmin",
          "type": "address"
        }
      ],
      "name": "setBathHouseAdmin",
      "outputs": [],
      "stateMutability": "nonpayable",
      "type": "function"
    },
    {
      "inputs": [
        {
          "internalType": "address",
          "name": "newImplementation",
          "type": "address"
        }
      ],
      "name": "setNewBathTokenImplementation",
      "outputs": [],
      "stateMutability": "nonpayable",
      "type": "function"
    },
    {
      "inputs": [
        {
          "internalType": "address",
          "name": "strategist",
          "type": "address"
        }
      ],
      "name": "approveStrategist",
      "outputs": [],
      "stateMutability": "nonpayable",
      "type": "function"
    },
    {
      "inputs": [
        {
          "internalType": "address",
          "name": "strategist",
          "type": "address"
        }
      ],
      "name": "removeStrategist",
      "outputs": [],
      "stateMutability": "nonpayable",
      "type": "function"
    },
    {
      "inputs": [
        {
          "internalType": "bool",
          "name": "_new",
          "type": "bool"
        }
      ],
      "name": "setPermissionedStrategists",
      "outputs": [],
      "stateMutability": "nonpayable",
      "type": "function"
    },
    {
      "inputs": [
        {
          "internalType": "uint256",
          "name": "value",
          "type": "uint256"
        }
      ],
      "name": "setCancelTimeDelay",
      "outputs": [],
      "stateMutability": "nonpayable",
      "type": "function"
    },
    {
      "inputs": [
        {
          "internalType": "uint256",
          "name": "rr",
          "type": "uint256"
        }
      ],
      "name": "setReserveRatio",
      "outputs": [],
      "stateMutability": "nonpayable",
      "type": "function"
    },
    {
      "inputs": [
        {
          "internalType": "address",
          "name": "bathToken",
          "type": "address"
        },
        {
          "internalType": "address",
          "name": "newMarket",
          "type": "address"
        }
      ],
      "name": "setBathTokenMarket",
      "outputs": [],
      "stateMutability": "nonpayable",
      "type": "function"
    },
    {
      "inputs": [
        {
          "internalType": "address",
          "name": "bathToken",
          "type": "address"
        },
        {
          "internalType": "address",
          "name": "newBonusToken",
          "type": "address"
        }
      ],
      "name": "setBonusToken",
      "outputs": [],
      "stateMutability": "nonpayable",
      "type": "function"
    },
    {
      "inputs": [
        {
          "internalType": "address",
          "name": "bathToken",
          "type": "address"
        },
        {
          "internalType": "address",
          "name": "newBathBuddy",
          "type": "address"
        }
      ],
      "name": "setBathTokenBathBuddy",
      "outputs": [],
      "stateMutability": "nonpayable",
      "type": "function"
    },
    {
      "inputs": [
        {
          "internalType": "address",
          "name": "bathToken",
          "type": "address"
        },
        {
          "internalType": "address",
          "name": "newAdmin",
          "type": "address"
        }
      ],
      "name": "setBathTokenBathHouse",
      "outputs": [],
      "stateMutability": "nonpayable",
      "type": "function"
    },
    {
      "inputs": [
        {
          "internalType": "address",
          "name": "bathToken",
          "type": "address"
        },
        {
          "internalType": "uint256",
          "name": "newBPS",
          "type": "uint256"
        }
      ],
      "name": "setBathTokenFeeBPS",
      "outputs": [],
      "stateMutability": "nonpayable",
      "type": "function"
    },
    {
      "inputs": [
        {
          "internalType": "address",
          "name": "targetBathToken",
          "type": "address"
        }
      ],
      "name": "bathTokenApproveSetMarket",
      "outputs": [],
      "stateMutability": "nonpayable",
      "type": "function"
    },
    {
      "inputs": [
        {
          "internalType": "address",
          "name": "bathToken",
          "type": "address"
        },
        {
          "internalType": "address",
          "name": "feeTo",
          "type": "address"
        }
      ],
      "name": "setBathTokenFeeTo",
      "outputs": [],
      "stateMutability": "nonpayable",
      "type": "function"
    },
    {
      "inputs": [
        {
          "internalType": "address",
          "name": "newMarket",
          "type": "address"
        }
      ],
      "name": "setMarket",
      "outputs": [],
      "stateMutability": "nonpayable",
      "type": "function"
    },
    {
      "inputs": [
        {
          "internalType": "uint8",
          "name": "newBPS",
          "type": "uint8"
        }
      ],
      "name": "setBPSToStrategists",
      "outputs": [],
      "stateMutability": "nonpayable",
      "type": "function"
    },
    {
      "inputs": [],
      "name": "getMarket",
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
      "name": "getReserveRatio",
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
      "name": "getCancelTimeDelay",
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
          "internalType": "contract IERC20",
          "name": "asset",
          "type": "address"
        }
      ],
      "name": "getBathTokenfromAsset",
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
      "name": "getBPSToStrats",
      "outputs": [
        {
          "internalType": "uint8",
          "name": "",
          "type": "uint8"
        }
      ],
      "stateMutability": "view",
      "type": "function"
    },
    {
      "inputs": [
        {
          "internalType": "address",
          "name": "wouldBeStrategist",
          "type": "address"
        }
      ],
      "name": "isApprovedStrategist",
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
          "name": "pair",
          "type": "address"
        }
      ],
      "name": "isApprovedPair",
      "outputs": [
        {
          "internalType": "bool",
          "name": "outcome",
          "type": "bool"
        }
      ],
      "stateMutability": "view",
      "type": "function"
    }
  ]
  "#;
