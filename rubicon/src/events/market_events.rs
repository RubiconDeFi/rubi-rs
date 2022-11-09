use ethers::contract::EthEvent;
use ethers::core::types::{Address, U256};

// first, we do the matching events

#[derive(Clone, Debug, EthEvent)]
pub struct LogBuyEnabled {
    #[ethevent(name = "isEnabled")]
    is_enabled: bool,
}

#[derive(Clone, Debug, EthEvent)]
pub struct LogMinSell {
    pay_gem: Address,
    min_amount: U256,
}

#[derive(Clone, Debug, EthEvent)]
pub struct LogMatchingEnabled {
    #[ethevent(name = "isEnabled")]
    is_enabled: bool,
}

#[derive(Clone, Debug, EthEvent)]
pub struct LogUnsortedOffer {
    id: U256,
}

#[derive(Clone, Debug, EthEvent)]
pub struct LogSortedOffer {
    id: U256,
}

#[derive(Clone, Debug, EthEvent)]
pub struct LogInsert {
    keeper: Address,
    id: U256,
}

#[derive(Clone, Debug, EthEvent)]
pub struct LogDelete {
    keeper: Address,
    id: U256,
}

#[derive(Clone, Debug, EthEvent)]
pub struct LogMatch {
    id: U256,
    amount: U256,
}

// now, we go do the eventful market events
#[derive(Clone, Debug, EthEvent)]
pub struct LogItemUpdate {
    id: U256,
}

#[derive(Clone, Debug, EthEvent)]
pub struct LogTrade {
    pay_amt: U256,
    #[ethevent(indexed)]
    pay_gem: Address,
    buy_amt: U256,
    #[ethevent(indexed)]
    buy_gem: Address,
}

#[derive(Clone, Debug, EthEvent)]
pub struct LogMake {
    #[ethevent(indexed)]
    id: [u8; 32],
    #[ethevent(indexed)]
    pair: [u8; 32],
    #[ethevent(indexed)]
    maker: [u8; 32],
    pay_gem: Address,
    buy_gem: Address,
    pay_amt: u128,
    buy_amt: u128,
    timestamp: u64,
}

impl LogMake {
    pub fn id(&self) -> U256 {
        U256::from_big_endian(&self.id)
    }

    pub fn maker_address(&self) -> Address {
        Address::from_slice(&self.maker[12..])
    }

    pub fn pair() {}
}

#[derive(Clone, Debug, EthEvent)]
pub struct LogBump {
    #[ethevent(indexed)]
    id: [u8; 32],
    #[ethevent(indexed)]
    pair: [u8; 32],
    #[ethevent(indexed)]
    maker: [u8; 32],
    pay_gem: Address,
    buy_gem: Address,
    pay_amt: u128,
    buy_amt: u128,
    timestamp: u64,
}

#[derive(Clone, Debug, EthEvent)]
pub struct LogTake {
    #[ethevent(indexed)]
    id: [u8; 32],
    #[ethevent(indexed)]
    pair: [u8; 32],
    #[ethevent(indexed)]
    maker: [u8; 32],
    pay_gem: Address,
    buy_gem: Address,
    #[ethevent(indexed)]
    taker: Address,
    take_amt: u128,
    give_amt: u128,
    timestamp: u64,
}

#[derive(Clone, Debug, EthEvent)]
pub struct LogKill {
    #[ethevent(indexed)]
    id: [u8; 32],
    #[ethevent(indexed)]
    pair: [u8; 32],
    #[ethevent(indexed)]
    maker: [u8; 32],
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

#[derive(Clone, Debug, EthEvent)]
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

#[derive(Clone, Debug, EthEvent)]
pub struct OfferDeleted {
    #[ethevent(indexed)]
    id: [u8; 32],
}
