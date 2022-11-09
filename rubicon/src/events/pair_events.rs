use ethers::contract::EthEvent;
use ethers::core::types::{Address, U256};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, EthEvent, Deserialize, Serialize)]
pub struct LogStrategistTrade {
    #[ethevent(name = "strategistTradeID")]
    strategist_trade_id: U256,
    #[ethevent(name = "askId")]
    ask_id: [u8; 32], // [u8;N] is tokenizable
    #[ethevent(name = "bidId")]
    bid_id: [u8; 32],
    #[ethevent(name = "askAsset")]
    ask_asset: Address,
    #[ethevent(name = "bidAsset")]
    bid_asset: Address,
    timestamp: U256,
    strategist: Address,
}

#[derive(Clone, Debug, EthEvent, Deserialize, Serialize)]
pub struct LogScrubbedStratTrade {
    #[ethevent(name = "strategistIDScrubbed")]
    strategist_id_scrubbed: U256,
    #[ethevent(name = "assetFill")]
    asset_fill: U256,
    #[ethevent(name = "assetAddress")]
    asset_address: Address,
    #[ethevent(name = "bathAssetAddress")]
    bath_asset_address: Address,
    #[ethevent(name = "quoteFill")]
    quote_fill: U256,
    #[ethevent(name = "quoteAddress")]
    quote_address: Address,
    #[ethevent(name = "bathQuoteAddress")]
    both_quote_address: Address,
}

#[derive(Clone, Debug, EthEvent, Deserialize, Serialize)]
pub struct LogStrategistRewardClaim {
    strategist: Address,
    asset: Address,
    #[ethevent(name = "amountOfReward")]
    amount_of_reward: U256,
    timestamp: U256,
}

#[derive(Clone, Debug, EthEvent, Deserialize, Serialize)]
pub struct LogBatchRequoteOffers {
    strategist: Address,
    trades: Vec<U256>, // presumably, these are IDs?
}

#[derive(Clone, Debug, EthEvent, Deserialize, Serialize)]
pub struct LogTailOffv0 {
    strategist: Address,
    #[ethevent(name = "targetPool")]
    target_pool: Address,
    #[ethevent(name = "tokenToHandle")]
    token_to_handle: Address,
    #[ethevent(name = "targetToken")]
    target_token: Address,
    #[ethevent(name = "poolFee")]
    pool_fee: U256,
    #[ethevent(name = "inputAmount")]
    input_amount: U256,
}

#[derive(Clone, Debug, EthEvent, Deserialize, Serialize)]
pub struct LogTailOffMultiv0 {
    strategist: Address,
    #[ethevent(name = "targetPool")]
    target_pool: Address,
    assets: Vec<Address>,
    fees: Vec<u32>, // ?????
    #[ethevent(name = "inputAmount")]
    input_amount: U256,
}

#[derive(Clone, Debug, EthEvent, Deserialize, Serialize)]
pub struct LogRebalancePair {
    strategist: Address,
    #[ethevent(name = "underlyingAsset")]
    underlying_asset: Address,
    #[ethevent(name = "quoteAddress")]
    quote_address: Address,
    #[ethevent(name = "assetRebalAmt")]
    asset_rebal_amt: U256,
    #[ethevent(name = "quoteRebalAmt")]
    quote_rebal_amt: U256,
    #[ethevent(name = "strategistReward")]
    strategist_reward: U256,
}
