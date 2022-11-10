use ethers::contract::EthEvent;
use ethers::core::types::{Address, U256};
use serde::{Deserialize, Serialize};
// first, we do the matching events

#[derive(Clone, Debug, EthEvent, Deserialize, Serialize)]
pub struct LogBuyEnabled {
    #[ethevent(name = "isEnabled")]
    is_enabled: bool,
}

#[derive(Clone, Debug, EthEvent, Deserialize, Serialize)]
pub struct LogMinSell {
    pay_gem: Address,
    min_amount: U256,
}

#[derive(Clone, Debug, EthEvent, Deserialize, Serialize)]
pub struct LogMatchingEnabled {
    #[ethevent(name = "isEnabled")]
    is_enabled: bool,
}

#[derive(Clone, Debug, EthEvent, Deserialize, Serialize)]
pub struct LogUnsortedOffer {
    id: U256,
}

// VALIDATED
#[derive(Clone, Debug, EthEvent, Deserialize, Serialize)]
pub struct LogSortedOffer {
    id: U256,
}

#[derive(Clone, Debug, EthEvent, Deserialize, Serialize)]
pub struct LogInsert {
    keeper: Address,
    id: U256,
}

#[derive(Clone, Debug, EthEvent, Deserialize, Serialize)]
pub struct LogDelete {
    keeper: Address,
    id: U256,
}

#[derive(Clone, Debug, EthEvent, Deserialize, Serialize)]
pub struct LogMatch {
    id: U256,
    amount: U256,
}

// now, we go do the eventful market events
// VALIDATED
#[derive(Clone, Debug, EthEvent, Deserialize, Serialize)]
pub struct LogItemUpdate {
    id: U256,
}

// VALIDATED
#[derive(Clone, Debug, EthEvent, Deserialize, Serialize)]
pub struct LogTrade {
    pay_amt: U256,
    #[ethevent(indexed)]
    pay_gem: Address,
    buy_amt: U256,
    #[ethevent(indexed)]
    buy_gem: Address,
}

// sig needs to be 0x773ff502687307abfa024ac9f62f9752a0d210dac2ffd9a29e38e12e2ea82c82
#[derive(Clone, Debug, EthEvent, Deserialize, Serialize)]
#[ethevent(signature = "773ff502687307abfa024ac9f62f9752a0d210dac2ffd9a29e38e12e2ea82c82")]
//#[ethevent(abi="LogMake(bytes32,bytes32,address,address,address,uint128,uint128,uint64)")]
pub struct LogMake {
    #[ethevent(indexed)]
    id: U256,
    #[ethevent(indexed)]
    pair: [u8; 32],
    #[ethevent(indexed)]
    maker: Address,
    pay_gem: Address,
    buy_gem: Address,
    pay_amt: u128,
    buy_amt: u128,
    timestamp: u64,
}

#[derive(Clone, Debug, EthEvent, Deserialize, Serialize)]
#[ethevent(abi = "LogBump(bytes32,bytes32,address,address,address,uint128,uint128,uint64)")]
pub struct LogBump {
    #[ethevent(indexed)]
    id: [u8; 32],
    #[ethevent(indexed)]
    pair: [u8; 32],
    #[ethevent(indexed)]
    maker: Address,
    pay_gem: Address,
    buy_gem: Address,
    pay_amt: u128,
    buy_amt: u128,
    timestamp: u64,
}

#[derive(Clone, Debug, EthEvent, Deserialize, Serialize)]
#[ethevent(signature = "3383e3357c77fd2e3a4b30deea81179bc70a795d053d14d5b7f2f01d0fd4596f")]
pub struct LogTake {
    #[ethevent(indexed)]
    id: [u8; 32],
    #[ethevent(indexed)]
    pair: [u8; 32],
    #[ethevent(indexed)]
    maker: Address,
    pay_gem: Address,
    buy_gem: Address,
    #[ethevent(indexed)]
    taker: Address,
    take_amt: u128,
    give_amt: u128,
    timestamp: u64,
}

#[derive(Clone, Debug, EthEvent, Deserialize, Serialize)]
pub struct LogKill {
    #[ethevent(indexed)]
    id: [u8; 32],
    #[ethevent(indexed)]
    pair: [u8; 32],
    #[ethevent(indexed)]
    maker: Address,
    pay_gem: Address,
    buy_gem: Address,
    pay_amt: u128,
    buy_amt: u128,
    timestamp: u64,
}

// I don't know how strings work here - I'll deal with it later
// #[derive(Clone, Debug, EthEvent)]
// pub struct LogInt{
//     string lol,
//     uint256 input
// };

#[derive(Clone, Debug, EthEvent, Deserialize, Serialize)]
pub struct FeeTake {
    #[ethevent(indexed)]
    id: [u8; 32],
    #[ethevent(indexed)]
    pair: [u8; 32],
    asset: Address,
    #[ethevent(indexed)]
    taker: Address,
    #[ethevent(name = "feeTo")]
    fee_to: Address,
    #[ethevent(name = "feeAmt")]
    fee_amt: U256,
    timestamp: u64,
}

#[derive(Clone, Debug, EthEvent, Deserialize, Serialize)]
pub struct OfferDeleted {
    #[ethevent(indexed)]
    id: [u8; 32],
}
