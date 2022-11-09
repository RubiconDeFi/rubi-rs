mod market_events;
mod pair_events;

pub use market_events::*;
pub use pair_events::*;

use ethers::contract::EthEvent;
use ethers::abi::RawLog;
use ethers::core::types::{TransactionReceipt, Log};
pub(crate) fn extract_events<E: EthEvent>(receipt: &TransactionReceipt) -> Vec<E> {
    receipt.logs.iter().map(|x| as_raw(x)).filter_map(|x| E::decode_log(&x).ok()).collect()
}

fn as_raw(log: &Log) -> RawLog {
    RawLog {
        topics: log.topics.clone(),
        data: log.data.to_vec(),
    }
}