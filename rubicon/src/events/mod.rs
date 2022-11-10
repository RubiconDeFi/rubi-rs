mod market_events;
mod pair_events;

pub use market_events::*;
pub use pair_events::*;

use ethers::abi::RawLog;
use ethers::contract::EthEvent;
use ethers::core::types::{Log, TransactionReceipt};


#[allow(dead_code)]
pub(crate) fn extract_events<E: EthEvent>(receipt: &TransactionReceipt) -> Vec<E> {
    receipt
        .logs
        .iter()
        .map(|x| as_raw(x))
        .filter_map(|x| E::decode_log(&x).ok())
        .collect()
}

#[allow(dead_code)]
fn as_raw(log: &Log) -> RawLog {
    RawLog {
        topics: log.topics.clone(),
        data: log.data.to_vec(),
    }
}
