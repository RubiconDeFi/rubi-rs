use super::events;

use anyhow::{anyhow, Result};

use ethers::{
    abi::Detokenize,
    contract::Contract,
    core::types::{Address, BlockNumber, Chain, TransactionReceipt, U256},
    prelude::{builders::ContractCall, EthEvent},
    providers::Middleware,
};
use futures::Future;
use numeraire::prelude::*;

use std::convert::Into;

use std::sync::Arc;
use tracing::{event, instrument, Level};

#[cfg(feature = "streaming")]
mod streaming;

/*
 * TRACING METHODOLOGY:
 *      low level functions that interact mutably with the blockchain use DEBUG
 *      high level functions that interact mutably with the blockchain use INFO
 *      WARN is used for errors that won't likely result in the death of the program
 *      ERROR is used for errors that will likely result in the death of the program
 */

pub type TxResult = Result<Option<TransactionReceipt>>;
trait FutTxResult: Future<Output = TxResult> {}

pub struct RubiconSession<M: Middleware + Clone + 'static> {
    chain: Chain,
    rbcn_market: Contract<M>,
    bath_house: Contract<M>,
    bath_pair: Contract<M>,
    market_aid: Option<Contract<M>>,
    router: Contract<M>,
    _internal_middleware: Arc<M>, // we just keep this around to clone if we build new contracts
}

#[allow(dead_code)]
impl<M: Middleware + Clone + 'static> RubiconSession<M> {
    pub fn new_mainnet(client: M) -> Self {
        let arc_client = Arc::new(client);
        Self {
            chain: Chain::Optimism,
            rbcn_market: crate::contracts::market::build_default(arc_client.clone()).unwrap(),
            bath_house: crate::contracts::house::build_default(arc_client.clone()).unwrap(),
            bath_pair: crate::contracts::pair::build_default(arc_client.clone()).unwrap(),
            router: crate::contracts::router::build_default(arc_client.clone()).unwrap(),
            market_aid: Some(
                crate::contracts::market_aid::build_default(arc_client.clone()).unwrap(),
            ),
            _internal_middleware: arc_client,
        }
    }

    pub fn new_kovan(client: M) -> Self {
        let arc_client = Arc::new(client);
        Self {
            chain: Chain::OptimismKovan,
            rbcn_market: crate::contracts::market::build_kovan(arc_client.clone()).unwrap(),
            bath_house: crate::contracts::house::build_kovan(arc_client.clone()).unwrap(),
            bath_pair: crate::contracts::pair::build_kovan(arc_client.clone()).unwrap(),
            router: crate::contracts::router::build_kovan(arc_client.clone()).unwrap(),
            market_aid: None,
            _internal_middleware: arc_client,
        }
    }

    pub fn new_goerli(client: M) -> Self {
        let arc_client = Arc::new(client);
        Self {
            chain: Chain::OptimismKovan,
            rbcn_market: crate::contracts::market::build_goerli(arc_client.clone()).unwrap(),
            bath_house: crate::contracts::house::build_goerli(arc_client.clone()).unwrap(),
            bath_pair: crate::contracts::pair::build_goerli(arc_client.clone()).unwrap(),
            router: crate::contracts::router::build_goerli(arc_client.clone()).unwrap(),
            market_aid: None,
            _internal_middleware: arc_client,
        }
    }

    #[instrument(level = "debug", skip_all)]
    fn change_provider(&mut self, provider: M) {
        let a = Arc::new(provider);

        self.rbcn_market = self.rbcn_market.connect(a.clone());
        self.bath_house = self.bath_house.connect(a.clone());
        self.bath_pair = self.bath_pair.connect(a.clone());
        self.router = self.router.connect(a.clone());
        self._internal_middleware = a;
    }

    // Getter functions, and some small helper functions
    pub fn market(&self) -> &Contract<M> {
        &self.rbcn_market
    }

    pub fn bath_house(&self) -> &Contract<M> {
        &self.bath_house
    }

    pub fn pair(&self) -> &Contract<M> {
        &self.bath_pair
    }

    pub fn router(&self) -> &Contract<M> {
        &self.router
    }

    pub fn chain(&self) -> &Chain {
        &self.chain
    }

    /// Market Aid isn't deployed on Kovan and Goerli - we can't always depend on it being there
    pub fn market_aid(&self) -> Option<&Contract<M>> {
        self.market_aid.as_ref()
    }

    /// Address associated with the current middleware, if it exists.
    pub fn get_address(&self) -> Option<Address> {
        self._internal_middleware.default_sender()
    }

    /// Are we on a legacy chain (pre EIP-1559)? If so, we have to use legacy TX calls...
    pub fn is_legacy(&self) -> bool {
        self.chain().is_legacy()
    }

    /**
     * This is a helper function that executes a transaction, and waits for a receipt.
     * By using the (ContractCall<M,D>, Future<...>) model for all of our transactions, we should retain the ability to
     * cancel stuff in flight, by calling the ContractCall part of the returned tuple again.
     */
    pub async fn handle_contract_call_v2<T: Detokenize>(
        &self,
        mut call: ContractCall<M, T>,
    ) -> TxResult {
        println!("entering handle_contract_call_v2");
        let gas_limit = Some(call.estimate_gas().await.unwrap());
        println!("gas_estimate: {}", gas_limit.as_ref().unwrap().to_string());
        call = if let Some(gl) = gas_limit {
            call.gas(gl)
        } else {
            call
        };

        let receipt = match call.send().await?.await {
            Ok(x) => x,
            Err(e) => {
                event!(
                    Level::WARN,
                    "[handle_contract_call]: failed to get receipt with error: {}",
                    e
                );
                return Err(e.into());
            }
        };

        Ok(receipt)
    }

    // RUBICON MARKET FUNCTIONS

    // first, we have the raw functions that interact with the contracts on chain

    /**
     * List of all Market functions:
     * - bump
     * - buy
     * - buyAllAmount //
     * - cancel //
     * - del_rank
     * - initialize
     * - kill
     * - make
     * - offer //
     * - offer //
     * - offer //
     * - sellAllAmount //
     * - setAqueductAddress
     * - setAqueductDistributionLive
     * - setBuyEnabled
     * - setFeeBPS
     * - setFeeTo
     * - setMinSell
     * - setOwner
     * - stop
     * - take
     */

    /// This is a market buy, where we spend no more than max_fill_amount to buy buy_amt
    /// the returned value is the fill amount
    #[instrument(level = "debug", skip(self))]
    pub fn buy_all_amount(
        &self,
        buy_gem: Address,
        buy_amt: U256,
        pay_gem: Address,
        max_fill_amount: U256,
    ) -> Result<ContractCall<M, U256>> {
        println!("entering buy_all_amount_v2");
        let tx = match self.is_legacy() {
            true => self
                .market()
                .method::<_, U256>("buyAllAmount", (buy_gem, buy_amt, pay_gem, max_fill_amount))?
                .legacy(),
            false => self
                .market()
                .method::<_, U256>("buyAllAmount", (buy_gem, buy_amt, pay_gem, max_fill_amount))?,
        };
        Ok(tx)
    }

    /// a market sell, where we spend pay_amt to buy as much as possible of buy_gem (and we get *at least* min_fill_amount)
    /// the returned value is the filled amount
    #[instrument(level = "debug", skip(self))]
    pub fn sell_all_amount(
        &self,
        pay_gem: Address,
        pay_amt: U256,
        buy_gem: Address,
        min_fill_amount: U256,
    ) -> Result<ContractCall<M, U256>> {
        let tx = match self.is_legacy() {
            true => self
                .market()
                .method::<_, U256>(
                    "sellAllAmount",
                    (pay_gem, pay_amt, buy_gem, min_fill_amount),
                )?
                .legacy(),
            false => self.market().method::<_, U256>(
                "sellAllAmount",
                (pay_gem, pay_amt, buy_gem, min_fill_amount),
            )?,
        };
        Ok(tx)
    }

    // what if we want to cancel the order before it hits the books tho...
    // offer to buy
    // should return the id of the order on the rubicon books
    #[instrument(level = "debug", skip(self))]
    pub fn offer(
        &self,
        pay_amt: U256,
        pay_gem: Address,
        buy_amt: U256,
        buy_gem: Address,
        pos: Option<U256>,
    ) -> Result<ContractCall<M, U256>> {
        let internal_position = pos.unwrap_or(U256::zero());

        let tx = if self.is_legacy() {
            self.market()
                .method::<_, U256>(
                    "offer",
                    (pay_amt, pay_gem, buy_amt, buy_gem, internal_position),
                )?
                .legacy()
        } else {
            self.market().method::<_, U256>(
                "offer",
                (pay_amt, pay_gem, buy_amt, buy_gem, internal_position),
            )?
        };

        Ok(tx)
    }

    /// Cancels an order that's already on the Rubicon book
    #[instrument(level = "debug", skip(self))]
    pub fn cancel(&self, order_id: U256) -> Result<ContractCall<M, U256>> {
        let tx = if self.is_legacy() {
            self.market()
                .method::<_, U256>("cancel", (order_id,))?
                .legacy()
        } else {
            self.market().method::<_, U256>("cancel", (order_id,))?
        };
        Ok(tx)
    }

    // second, we have wrapper functions around the raw functions, for the users to interact with
    /*
    /// executes a limit order using the constraints set out in the `AssetSwap` type
    #[instrument(level = "info", skip(self))]
    pub async fn limit_order_as(&self, swap: &AssetSwap) -> Result<U256> {
        self.limit_order_native(swap.source(), swap.target()).await
    }

    /// executes a a limit order
    #[instrument(level = "info", skip(self))]
    pub async fn limit_order_native(
        &self,
        source: &ChainNativeAsset,
        target: &ChainNativeAsset,
    ) -> Result<U256> {
        let (_, rst_opt_receipt) = self
            .offer(
                source.size().to_owned(),
                source.address().unwrap(),
                target.size().to_owned(),
                target.address().unwrap(),
                None,
            )?;
        match rst_opt_receipt.await {
            Ok(Some(receipt)) => Ok(deserialize_offer_receipt(&receipt)),
            Ok(None) => {
                event!(
                    Level::WARN,
                    "[limit_order_native]: receipt was an Ok(None) type"
                );
                Err(anyhow!(
                    "[limit_order_native]: receipt was an Ok(None) type"
                ))
            }
            Err(e) => {
                event!(
                    Level::WARN,
                    "[limit_order_native]: failed to get receipt with error: {}",
                    e
                );
                Err(e)
            }
        }
    }

    /// wrapper around `market_order`, but using the `AssetSwap` struct
    /// because of fees, this shouldn't return the identical asset swap that we put in
    /// but it should be close
    #[instrument(level = "info", skip(self))]
    pub async fn market_order_as(&self, swap: &AssetSwap, is_bounded: bool) -> Result<AssetSwap> {
        self.market_order_native(swap.source(), swap.target(), is_bounded, true)
            .await
    }

    // should return the amount that was filled, if successful
    #[instrument(level = "info", skip(self))]
    pub async fn market_order_native(
        &self,
        source: &ChainNativeAsset,
        target: &ChainNativeAsset,
        is_bounded: bool,
        is_buy: bool,
    ) -> Result<AssetSwap> {
        let opt_receipt = match (is_bounded, is_buy) {
            // `target` represents what and how much we want to buy, `source` represents what and the maximum amount we want to sell
            (true, true) => {
                self.buy_all_amount(
                    target.address()?,
                    *target.size(),
                    source.address()?,
                    *source.size(),
                )
                .await
            }
            (true, false) => {
                self.sell_all_amount(
                    source.address()?,
                    *source.size(),
                    target.address()?,
                    *target.size(),
                )
                .await
            }
            (false, true) => {
                self.buy_all_amount(
                    target.address()?,
                    *target.size(),
                    source.address()?,
                    U256::MAX,
                )
                .await
            }
            (false, false) => {
                self.sell_all_amount(
                    source.address()?,
                    *source.size(),
                    target.address()?,
                    U256::zero(),
                )
                .await
            }
        }?;

        if let Some(receipt) = opt_receipt {
            let (gained, lost) = deserialize_market_order_receipt(&receipt);
            let asset_swap = AssetSwap::new_from_primitive(
                *self.chain(),
                *source.asset(),
                *target.asset(),
                lost,
                gained,
            );
            asset_swap
        } else {
            Err(anyhow!("transaction receipt was an Ok(None) type!!!"))
        }
    }

    #[instrument(level = "info", skip(self))]
    pub async fn cancel_order(&self, order_id: U256) -> Result<TransactionReceipt> {
        match self.cancel(order_id).await {
            Ok(Some(receipt)) => Ok(receipt),
            Ok(None) => Err(anyhow!("transaction receipt was an Ok(None) type!")),
            Err(e) => Err(e),
        }
    }*/

    // RUBICON BATH HOUSE FUNCTIONS
    pub async fn self_is_approved_strategist(&self) -> Result<bool> {
        self.is_approved_strategist(
            self.get_address()
                .ok_or(anyhow!("ERROR: the Session doesn't have an address!"))?,
        )
        .await
    }

    pub async fn is_approved_strategist(&self, addr: Address) -> Result<bool> {
        let receipt = self
            .bath_house()
            .method::<_, bool>("isApprovedStrategist", (addr,))?
            .call()
            .await?;
        Ok(receipt)
    }

    // RUBICON BATH PAIR FUNCTIONS

    /*
     * what functions do we need?
     * - tail off (DONE)
     * - tail off multi (DONE)
     * - batch requote offers (DONE)
     * - requote a single pair of offers (DONE)
     * - place market making trades (DONE)
     * - batch market making trades (DONE except for deser of return...)
     * - rebalance pair (DONE)
     * - scrub strategist trade  (DONE)
     * - scrub strategist trades (DONE)
     */

    // these are the raw functions for interacting with the chain
    // they are not meant to be used directly, but rather through the
    // helper functions below
    // ask_num is ask_numerator, ask_dem -> ask_denominator
    // token_pair is of the form [base, quote]
    // returns a trade ID (U256)
    /// this emits a LogStrategistTrade event
    #[instrument(level = "debug", skip(self))]
    fn place_market_making_trades(
        &self,
        token_pair: [Address; 2],
        ask_num: U256,
        ask_dem: U256,
        bid_num: U256,
        bid_dem: U256,
    ) -> Result<ContractCall<M, ()>> {
        let tx = match self.is_legacy() {
            true => self
                .pair()
                .method::<_, ()>(
                    "placeMarketMakingTrades",
                    (token_pair, ask_num, ask_dem, bid_num, bid_dem),
                )?
                .legacy(),
            false => self.pair().method::<_, ()>(
                "placeMarketMakingTrades",
                (token_pair, ask_num, ask_dem, bid_num, bid_dem),
            )?,
        };
        Ok(tx)
    }

    // INCOMPLETE: shouldn't this return a vector of Order IDs?
    /// this emits a LogBatchMarketMakingTrades event
    #[instrument(level = "debug", skip(self))]
    fn batch_place_market_making_trades(
        &self,
        token_pair: [Address; 2],
        ask_nums: Vec<U256>,
        ask_dems: Vec<U256>,
        bid_nums: Vec<U256>,
        bid_dems: Vec<U256>,
    ) -> Result<ContractCall<M, ()>> {
        if !(ask_nums.len() == ask_dems.len() && bid_nums.len() == bid_dems.len()) {
            // there's some mismatch in the input values...
            // we should return an error and log it
            event!(
                Level::WARN,
                "[batch_place_market_making_trades]: mismatch in input vectors!"
            );
            Err(anyhow!(
                "[batch_place_market_making_trades]: mismatch in input vectors!"
            ))
        } else {
            let tx = match self.is_legacy() {
                true => self
                    .pair()
                    .method::<_, ()>(
                        "batchMarketMakingTrades",
                        (token_pair, ask_nums, ask_dems, bid_nums, bid_dems),
                    )?
                    .legacy(),
                false => self.pair().method::<_, ()>(
                    "batchMarketMakingTrades",
                    (token_pair, ask_nums, ask_dems, bid_nums, bid_dems),
                )?,
            };
            Ok(tx)
        }
    }

    // requotes a single pair of bid and ask
    // token_pair is of the form [base, quote]
    // this has no output
    #[instrument(level = "debug", skip(self))]
    fn requote_offers(
        &self,
        order_id: U256,
        token_pair: [Address; 2],
        ask_num: U256,
        ask_dem: U256,
        bid_num: U256,
        bid_dem: U256,
    ) -> Result<ContractCall<M, ()>> {
        let tx = match self.is_legacy() {
            true => self
                .pair()
                .method::<_, ()>(
                    "requote",
                    (order_id, token_pair, ask_num, ask_dem, bid_num, bid_dem),
                )?
                .legacy(),
            false => self.pair().method::<_, ()>(
                "requote",
                (order_id, token_pair, ask_num, ask_dem, bid_num, bid_dem),
            )?,
        };
        Ok(tx)
    }

    // requotes a series of paired bids and asks
    // this system operates in pairs - so that the first ask and the first bid share the same id, i think
    // token_pair is of the form [base, quote]
    // this has no output
    #[instrument(level = "debug", skip(self))]
    fn batch_requote_offers(
        &self,
        ids: Vec<U256>,
        token_pair: [Address; 2],
        ask_nums: Vec<U256>,
        ask_dems: Vec<U256>,
        bid_nums: Vec<U256>,
        bid_dems: Vec<U256>,
    ) -> Result<ContractCall<M, ()>> {
        assert!(
            !(ask_nums.len() == ask_dems.len()
                && bid_nums.len() == bid_dems.len()
                && ask_nums.len() == ids.len())
        );

        let tx = match self.is_legacy() {
            true => self
                .pair()
                .method::<_, ()>(
                    "batchRequoteOffers",
                    (ids, token_pair, ask_nums, ask_dems, bid_nums, bid_dems),
                )?
                .legacy(),
            false => self.pair().method::<_, ()>(
                "batchRequoteOffers",
                (ids, token_pair, ask_nums, ask_dems, bid_nums, bid_dems),
            )?,
        };
        Ok(tx)
    }

    // doesn't have any output
    #[instrument(level = "debug", skip(self))]
    fn scrub_strategist_trade(&self, trade_id: U256) -> Result<ContractCall<M, ()>> {
        let tx = match self.is_legacy() {
            true => self
                .pair()
                .method::<_, ()>("scrubStrategistTrade", trade_id)?
                .legacy(),
            false => self
                .pair()
                .method::<_, ()>("scrubStrategistTrade", trade_id)?,
        };
        Ok(tx)
    }

    // doesn't have any output
    #[instrument(level = "debug", skip(self))]
    fn scrub_strategist_trades(&self, trade_ids: Vec<U256>) -> Result<ContractCall<M, ()>> {
        let tx = match self.is_legacy() {
            true => self
                .pair()
                .method::<_, ()>("scrubStrategistTrades", trade_ids)?
                .legacy(),
            false => self
                .pair()
                .method::<_, ()>("scrubStrategistTrades", trade_ids)?,
        };
        Ok(tx)
    }

    // in an old ethers rust UV3 project, I used a u32 for the fee type......
    // let's fucking hope this works
    #[instrument(level = "debug", skip(self))]
    fn tailoff(
        &self,
        target_pool: Address,
        token_to_handle: Address,
        target_token: Address,
        strat_util: Address,
        amount: U256,
        hurdle: U256,
        pool_fee: u32,
    ) -> Result<ContractCall<M, ()>> {
        let tx = match self.is_legacy() {
            true => self
                .pair()
                .method::<_, ()>(
                    "tailOff",
                    (
                        target_pool,
                        token_to_handle,
                        target_token,
                        strat_util,
                        amount,
                        hurdle,
                        pool_fee,
                    ),
                )?
                .legacy(),
            false => self.pair().method::<_, ()>(
                "tailOff",
                (
                    target_pool,
                    token_to_handle,
                    target_token,
                    strat_util,
                    amount,
                    hurdle,
                    pool_fee,
                ),
            )?,
        };
        Ok(tx)
    }

    // doesn't have any output
    #[instrument(level = "debug", skip(self))]
    fn tailoff_multi(
        &self,
        target_pool: Address,
        amount: U256,
        assets: Vec<Address>,
        fees: Vec<u32>,
        hurdle: U256,
        strat_util: Address,
    ) -> Result<ContractCall<M, ()>> {
        let tx = match self.is_legacy() {
            true => self
                .pair()
                .method::<_, ()>(
                    "tailOffMulti",
                    (target_pool, amount, assets, fees, hurdle, strat_util),
                )?
                .legacy(),
            false => self.pair().method::<_, ()>(
                "tailOffMulti",
                (target_pool, amount, assets, fees, hurdle, strat_util),
            )?,
        };
        Ok(tx)
    }

    // this has no output
    #[instrument(level = "debug", skip(self))]
    fn rebalance_pair(
        &self,
        asset_rebal_amt: U256,
        quote_rebal_amt: U256,
        underlying_asset: Address,
        underlying_quote: Address,
    ) -> Result<ContractCall<M, ()>> {
        let tx = match self.is_legacy() {
            true => self
                .pair()
                .method::<_, ()>(
                    "rebalancePair",
                    (
                        asset_rebal_amt,
                        quote_rebal_amt,
                        underlying_asset,
                        underlying_quote,
                    ),
                )?
                .legacy(),
            false => self.pair().method::<_, ()>(
                "rebalancePair",
                (
                    asset_rebal_amt,
                    quote_rebal_amt,
                    underlying_asset,
                    underlying_quote,
                ),
            )?,
        };
        Ok(tx)
    }

    // now, we go and write some helper functions to make our lives easier
    // these are helper functions that take other helper types as arguments
    // they're public, not private, they're meant to be used by users
    /*
    #[instrument(level = "info", skip(self))]
    pub async fn s_place_paired_trade(&self, bid: AssetSwap, ask: AssetSwap) -> Result<()> {
        // we've got to go check that the bid and ask are local and conjugate
        self.local_and_conjugate_rst(vec![bid.clone()].as_slice(), vec![ask.clone()].as_slice())?;

        // now we can go and place the trade
        let token_pair = [bid.target().address()?, bid.source().address()?]; // [base, quote]
                                                                             // numerator is quote, denominator is base
        let _receipt = self
            .place_market_making_trades(
                token_pair,
                *ask.target().size(),
                *ask.source().size(),
                *bid.source().size(),
                *bid.target().size(),
            )
            .await;

        Ok(())
    }

    #[instrument(level = "info", skip(self))]
    pub async fn s_place_paired_trades(
        &self,
        bids: Vec<AssetSwap>,
        asks: Vec<AssetSwap>,
    ) -> Result<()> {
        // we've got to go check that the bid and ask are local and conjugate
        self.local_and_conjugate_rst(&bids, &asks)?;

        // now, we can go turn this into several vectors of bullshit
        // numerators are quote, denominators are base
        let ask_nums: Vec<U256> = asks.iter().map(|ask| *ask.target().size()).collect();
        let ask_dens: Vec<U256> = asks.iter().map(|ask| *ask.source().size()).collect();
        let bid_nums: Vec<U256> = bids.iter().map(|bid| *bid.source().size()).collect();
        let bid_dens: Vec<U256> = bids.iter().map(|bid| *bid.target().size()).collect();

        // token_pair is of the form [base, quote]
        let token_pair = [
            bids[0].target().asset().to_address(self.chain())?,
            bids[0].source().asset().to_address(self.chain())?,
        ];

        let _receipt = self
            .batch_place_market_making_trades(token_pair, ask_nums, ask_dens, bid_nums, bid_dens)
            .await;

        Ok(())
    }

    // #[instrument(level = "info", skip(self))]
    pub async fn s_requote_paired_trade(
        &self,
        order_id: U256,
        bid: AssetSwap,
        ask: AssetSwap,
    ) -> Result<ContractCall<M, ()>> {
        // the bid is quote seeking base
        // the ask is base seeking quote
        // we have to assert that the bid chain is the same as the ask chain is the same as the Session chain
        // we also have to assert that both the bid and the ask are local to a single chain

        if !bid.is_local_to_chain() {
            return Err(anyhow!(
                "[requote_paired_trade]: ERROR: bid is not local to a single chain!"
            ));
        }

        if !ask.is_local_to_chain() {
            return Err(anyhow!(
                "[requote_paired_trade]: ERROR: ask is not local to a single chain!"
            ));
        }

        if bid.source().chain() != ask.source().chain() {
            return Err(anyhow!(
                "[requote_paired_trade]: ERROR: bid and ask are not local to the same chain!"
            ));
        }

        // now, we need to assert that the bid is basically the inverse of the ask in terms of assets
        if bid.source().asset() != ask.target().asset() {
            return Err(anyhow!("[requote_paired_trade]: ERROR: bid source asset and ask target asset are not the same!"));
        }

        if bid.target().asset() != ask.source().asset() {
            return Err(anyhow!("[requote_paired_trade]: ERROR: bid target asset and ask source asset are not the same!"));
        }

        // this is all we will check - everything else (like, is ask.price > bid.price) needs to be up to the user to not fuck up
        // numerator is quote
        // denominator is base
        let bid_num = *bid.source().size();
        let bid_den = *bid.target().size();
        let ask_num = *ask.target().size();
        let ask_den = *ask.source().size();

        let chain = self.chain();
        let token_pair = [
            bid.target().asset().to_address(chain)?,
            bid.source().asset().to_address(chain)?,
        ]; // base, quote

        self.requote_offers(order_id, token_pair, bid_num, bid_den, ask_num, ask_den)
    }

    #[instrument(level = "info", skip(self))]
    pub async fn s_batch_requote_paired_trades(
        &self,
        order_ids: Vec<U256>,
        bids: Vec<AssetSwap>,
        asks: Vec<AssetSwap>,
    ) -> Result<TransactionReceipt> {
        // first we assert that the lengths of the vectors are all the same
        if !(order_ids.len() == bids.len() && bids.len() == asks.len()) {
            return Err(anyhow!("[batch_requote_paired_trades]: ERROR: order_ids, bids, and asks vectors are not all the same length!"));
        }

        // second, we assert that they're all greater than zero in length
        if !order_ids.len() > 0 {
            return Err(anyhow!("[batch_requote_paired_trades]: ERROR: order_ids, bids, and asks vectors are all zero length!"));
        }

        // the rest is checked by local and conjugate
        self.local_and_conjugate_rst(&bids, &asks)?;

        // now, we can go turn this into several vectors of bullshit
        // numerators are quote, denominators are base
        let ask_nums: Vec<U256> = asks.iter().map(|ask| *ask.target().size()).collect();
        let ask_dens: Vec<U256> = asks.iter().map(|ask| *ask.source().size()).collect();
        let bid_nums: Vec<U256> = bids.iter().map(|bid| *bid.source().size()).collect();
        let bid_dens: Vec<U256> = bids.iter().map(|bid| *bid.target().size()).collect();

        // token_pair is of the form [base, quote]
        let token_pair = [
            bids[0].target().asset().to_address(self.chain())?,
            bids[0].source().asset().to_address(self.chain())?,
        ];

        // now, we've got all we need to call the elementary function
        let rst_opt_tx = self
            .batch_requote_offers(
                order_ids, token_pair, ask_nums, ask_dens, bid_nums, bid_dens,
            )
            .await;

        match rst_opt_tx {
            Ok(Some(tx)) => Ok(tx),
            Ok(None) => Err(anyhow!(
                "[batch_requote_paired_trades]: ERROR: tx_receipt is None!"
            )),
            Err(e) => Err(e),
        }
    }

    #[instrument(level = "info", skip(self))]
    pub async fn s_clear_trade(&self, order_id: U256) -> Result<TransactionReceipt> {
        let tx_receipt = self.scrub_strategist_trade(order_id).await?;
        match tx_receipt {
            Some(tx) => Ok(tx),
            None => Err(anyhow!("[clear_strat_trade]: ERROR: tx_receipt is None!")),
        }
    }

    #[instrument(level = "info", skip(self))]
    pub async fn s_clear_trades(&self, order_ids: Vec<U256>) -> Result<TransactionReceipt> {
        if order_ids.is_empty() {
            return Err(anyhow!(
                "[clear_strat_trades]: ERROR: order_ids vector is zero length!"
            ));
        }
        let tx_receipt = self.scrub_strategist_trades(order_ids).await?;
        match tx_receipt {
            Some(tx) => Ok(tx),
            None => Err(anyhow!("[clear_strat_trades]: ERROR: tx_receipt is None!")),
        }
    }

    // this has no output
    #[instrument(level = "info", skip(self))]
    pub async fn s_rebalance_swap(&self, swap: AssetSwap) -> TxResult {
        // first, we've gotta check that the chain is the same for both assets
        if swap.source().chain() != swap.target().chain() {
            event!(
                Level::WARN,
                "[rebalance_swap]: WARNING: source and target chains are not the same!"
            );
            return Err(anyhow!(
                "[rebalance_swap]: WARNING: source and target chains are not the same!"
            ));
        }

        // now, we need to check that the chain is the same as the strategist's chain
        if swap.source().chain() != self.chain() {
            event!(Level::WARN, "[rebalance_swap]: WARNING: source/target chain and strategist chain are not the same!");
            return Err(anyhow!("[rebalance_swap]: WARNING: source and target chains are not the same as the session chain!"));
        }

        self.rebalance_pair(
            *swap.source().size(),
            *swap.target().size(),
            swap.source().address()?,
            swap.target().address()?,
        )
        .await
    }

    // MarketAid functions
    // first, the raw functions, later, the helper functions
    /// Returns a Result on a Vector of trade ID's
    async fn get_outstanding_strategist_trades(
        &self,
        asset: Address,
        quote: Address,
        strategist: Address,
    ) -> Result<Vec<U256>> {
        let ctr = self.market_aid();
        match ctr {
            Some(c) => Ok(c
                .method::<_, Vec<U256>>(
                    "getOutstandingStrategistTrades",
                    (asset, quote, strategist),
                )?
                .call()
                .await?),
            None => Err(anyhow!(
                "[get_outstanding_strategist_trades]: ERROR: market aid contract is None!"
            )),
        }
    }

    pub async fn s_get_outstanding_strat_trades(
        &self,
        pair: Pair,
        strategist: Address,
    ) -> Result<Vec<U256>> {
        let chain = self.chain();
        self.get_outstanding_strategist_trades(
            pair.base().to_address(chain)?,
            pair.quote().to_address(chain)?,
            strategist,
        )
        .await
    }

    pub async fn s_self_get_outstanding_strat_trades(&self, pair: Pair) -> Result<Vec<U256>> {
        let self_addr = self.get_address();
        self.s_get_outstanding_strat_trades(pair, self_addr.ok_or(anyhow!("[s_self_get_outstanding_strat_trades]: We don't have an address for this provider!"))?).await
    }*/

    // UTILITY FUNCTIONS AND WHATNOT
    /// This checks that the bids and asks are:
    /// 1. all local to the same chain
    /// 2. that that chain is the same as the session chain
    /// 3. that the bids and asks are all conjugate in the same way (i.e. all bids are QUOTE->BASE and all bids are BASE->QUOTE)
    fn local_and_conjugate_rst(&self, bids: &[AssetSwap], asks: &[AssetSwap]) -> Result<()> {
        // now, we go assert that all the bids and asks are local to a single chain
        if !bids.iter().map(|x| x.is_local_to_chain()).all(|x| x) {
            return Err(anyhow!(
                "[local_and_conjugate_rst]: ERROR: all bids are not local to a single chain!"
            ));
        }

        if !asks.iter().map(|x| x.is_local_to_chain()).all(|x| x) {
            return Err(anyhow!(
                "[local_and_conjugate_rst]: ERROR: all asks are not local to a single chain!"
            ));
        }

        // now, we go assert that all the bids and asks are local to the same chain and that that chain is the same as the session chain
        if !bids.iter().all(|x| x.source().chain() == self.chain()) {
            return Err(anyhow!("[local_and_conjugate_rst]: ERROR: all bids are not local to the same chain as the session chain!"));
        }

        if !asks.iter().all(|x| x.source().chain() == self.chain()) {
            return Err(anyhow!("[local_and_conjugate_rst]: ERROR: all asks are not local to the same chain as the session chain!"));
        }

        // we've also got to go make sure that the source and target types for all the assets are the the same or conjugate

        // we do this by making sure that they're the same in a row and then comparing the bids.src to the asks.target, etc
        if !bids
            .iter()
            .all(|x| x.source().asset() == bids[0].source().asset())
        {
            return Err(anyhow!(
                "[local_and_conjugate_rst]: ERROR: not all bids have the same source asset!"
            ));
        }

        if !bids
            .iter()
            .all(|x| x.target().asset() == bids[0].target().asset())
        {
            return Err(anyhow!(
                "[local_and_conjugate_rst]: ERROR: not all bids have the same target asset!"
            ));
        }

        if !asks
            .iter()
            .all(|x| x.source().asset() == asks[0].source().asset())
        {
            return Err(anyhow!(
                "[local_and_conjugate_rst]: ERROR: not all asks have the same source asset!"
            ));
        }

        if !asks
            .iter()
            .all(|x| x.target().asset() == asks[0].target().asset())
        {
            return Err(anyhow!(
                "[local_and_conjugate_rst]: ERROR: not all asks have the same target asset!"
            ));
        }

        // well, now we've got to go check the conjugate-ness of the assets
        if bids[0].source().asset() != asks[0].target().asset() {
            return Err(anyhow!("[local_and_conjugate_rst]: ERROR: bids source asset and asks target asset are not the same!"));
        }

        if bids[0].target().asset() != asks[0].source().asset() {
            return Err(anyhow!("[local_and_conjugate_rst]: ERROR: bids target asset and asks source asset are not the same!"));
        }

        Ok(())
    }

    // let's go work on events and whatnot
    // basically, for every event type, we want to be able to get historical events (bounded by a block range) and be able to get a stream of these new events as they happen
    // and maybe as well, we'd want an async fn to take that stream
    // and broadcast it over a Broadcast channel
    // and for all these functions, we'd like to be able to optinally apply a filter

    // let's handle LogBatchRequoteOffers first
    // to be perfectly frank, it's probably easier and faster to filter non-block number related stuff in memory after we receive a response
    // this might be wrong if we're dealing with huge numbers of events, where this might increase network load
    /// This pulls historical event data from the chain
    pub async fn pair_get_historical_batch_requotes(
        &self,
        oldest_block: Option<impl Into<BlockNumber>>,
        newest_block: Option<impl Into<BlockNumber>>,
    ) -> Result<Vec<events::LogBatchRequoteOffers>> {
        get_historical_events::<_, events::LogBatchRequoteOffers>(
            self.pair(),
            oldest_block,
            newest_block,
        )
        .await
    }
}

// some event helper functions
async fn get_historical_events<M: Middleware + 'static, E: EthEvent>(
    contract: &Contract<M>,
    start: Option<impl Into<BlockNumber>>,
    end: Option<impl Into<BlockNumber>>,
) -> Result<Vec<E>> {
    let event = contract.event::<E>();
    let query = match (start, end) {
        (Some(lower), Some(upper)) => event.from_block(lower).to_block(upper),
        (None, Some(upper)) => event.to_block(upper),
        (Some(lower), None) => event.from_block(lower),
        (None, None) => event,
    };
    Ok(query.query().await?)
}

/// we return the amount of `buy_gem` we gained, and the amount of `pay_gem` we spent, in that order
fn deserialize_market_order_receipt(receipt: &TransactionReceipt) -> (U256, U256) {
    // we should do some type checking to make sure this shit doesn't die on us
    // we should guarantee that the logs have non-zero length
    // and that the last log has length equal to 256*2 bits = 64 bytes
    assert!(!receipt.logs.is_empty());
    let last_log_data: &[u8] = receipt.logs[receipt.logs.len() - 1].data.as_ref(); // the bytes of the last log

    assert!(last_log_data.len() == 64);
    // the first U256 is the amount of stuff we purchased
    // the second is the amount of stuff we lost
    let gained = U256::from_little_endian(&last_log_data[0..32]);
    let lost = U256::from_little_endian(&last_log_data[32..64]);
    (gained, lost)
}

fn deserialize_offer_receipt(receipt: &TransactionReceipt) -> U256 {
    // we should do some type checking to make sure this shit doesn't die on us
    // we should guarantee that the logs have non-zero length
    // and that the last log has length equal to 256 bits = 32 bytes
    assert!(!receipt.logs.is_empty());
    let last_log_data: &[u8] = receipt.logs[receipt.logs.len() - 1].data.as_ref(); // the bytes of the last log
    println!("last_log_data {:?}", hex::encode(last_log_data));

    assert!(last_log_data.len() == 32);
    // the last log is the order id
    U256::from_big_endian(&last_log_data[0..32])
}

// pmmt => placeMarketMakingTrades
// should return a Result of a U256 which is the order ID
fn deserialize_pmmt_receipt() {}
