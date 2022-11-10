use anyhow::{anyhow, Result};

pub use ethers::prelude::builders::ContractCall;
use ethers::{
    contract::Contract,
    core::types::{Address, Chain, U256},
    providers::Middleware,
    signers::Signer,
    middleware::SignerMiddleware,
};
use numeraire::prelude::*;
use rust_decimal::Decimal;
use tracing::instrument;
use std::sync::Arc; 
// #[cfg(feature = "streaming")]
// mod streaming;

/*
 * TRACING METHODOLOGY:
 *      low level functions that interact mutably with the blockchain use DEBUG
 *      high level functions that interact mutably with the blockchain use INFO
 *      WARN is used for errors that won't likely result in the death of the program
 *      ERROR is used for errors that will likely result in the death of the program
 */

/**
 * [`RubiconSession`] wraps up all of the basic contracts that the protocol uses into one convenient struct.
 * It provides all of the basic functions you would need to tracsact with the protocol. All of the view/pure functions are async,
 * and will return the expected value. All of the mutating functions are sync, and will return a ContractCall<M>.
 * You can then take that ContractCall and manipulate it as you want (e.g. setting gas limits) before sending it to your configured provider with the `.send()` method.
 */
pub struct RubiconSession<M: Middleware + Clone + 'static> {
    chain: Chain,
    rbcn_market: Contract<M>,
    bath_house: Contract<M>,
    bath_pair: Contract<M>,
    #[cfg(feature = "aid")]
    market_aid: Contract<M>,
    router: Contract<M>,
    _internal_middleware: Arc<M>, // we just keep this around to clone if we build new contracts
}

/**
*  impl<M: Middleware + Clone + 'static> RubiconSession<M>
   where
       <M as Middleware>::Provider: PubsubClient,
*/

#[allow(dead_code)]
impl<M: Middleware + Clone + 'static> RubiconSession<M> {
    /// Creates a new [`RubiconSession`] configured to Optimism Mainnet.
    pub fn new_mainnet(client: M) -> Self {
        let arc_client = Arc::new(client);
        Self {
            chain: Chain::Optimism,
            rbcn_market: crate::contracts::market::build_default(arc_client.clone()).unwrap(),
            bath_house: crate::contracts::house::build_default(arc_client.clone()).unwrap(),
            bath_pair: crate::contracts::pair::build_default(arc_client.clone()).unwrap(),
            router: crate::contracts::router::build_default(arc_client.clone()).unwrap(),
            #[cfg(feature = "aid")]
            market_aid: crate::contracts::market_aid::build_default(arc_client.clone()).unwrap(),
            _internal_middleware: arc_client,
        }
    }

    /// Creates a new [`RubiconSession`] configured to Optimism Kovan. OP Kovan is now considered obsolete.
    /// This requires that the `aid` feature is not set - the Market Aid contract is not deployed on OP Kovan.
    #[cfg(not(feature = "aid"))]
    pub fn new_kovan(client: M) -> Self {
        let arc_client = Arc::new(client);
        Self {
            chain: Chain::OptimismKovan,
            rbcn_market: crate::contracts::market::build_kovan(arc_client.clone()).unwrap(),
            bath_house: crate::contracts::house::build_kovan(arc_client.clone()).unwrap(),
            bath_pair: crate::contracts::pair::build_kovan(arc_client.clone()).unwrap(),
            router: crate::contracts::router::build_kovan(arc_client.clone()).unwrap(),
            _internal_middleware: arc_client,
        }
    }

    /// Creates a new [`RubiconSession`] configured to Optimism Goerli.
    /// This requires that the `aid` feature is not set - the Market Aid contract is not deployed on OP Goerli.
    #[cfg(not(feature = "aid"))]
    pub fn new_goerli(client: M) -> Self {
        let arc_client = Arc::new(client);
        Self {
            chain: Chain::OptimismKovan,
            rbcn_market: crate::contracts::market::build_goerli(arc_client.clone()).unwrap(),
            bath_house: crate::contracts::house::build_goerli(arc_client.clone()).unwrap(),
            bath_pair: crate::contracts::pair::build_goerli(arc_client.clone()).unwrap(),
            router: crate::contracts::router::build_goerli(arc_client.clone()).unwrap(),
            _internal_middleware: arc_client,
        }
    }

    /// Changes the provider of the [`RubiconSession`].
    /// This may be useful to you if your provider endpoint goes down, and you want to switch to a backup on the fly.
    #[instrument(level = "info", skip(self))]
    fn change_provider(&mut self, provider: M) {
        let a = Arc::new(provider);

        self.rbcn_market = self.rbcn_market.connect(a.clone());
        self.bath_house = self.bath_house.connect(a.clone());
        self.bath_pair = self.bath_pair.connect(a.clone());
        self.router = self.router.connect(a.clone());
        self._internal_middleware = a;
    }

    // Getter functions, and some small helper functions
    /// Returns a reference to the RubiconMarket contract.
    pub fn market(&self) -> &Contract<M> {
        &self.rbcn_market
    }

    /// Returns a reference to the BathHouse contract.
    pub fn bath_house(&self) -> &Contract<M> {
        &self.bath_house
    }

    /// Returns a reference to the BathPair contract.
    pub fn pair(&self) -> &Contract<M> {
        &self.bath_pair
    }

    /// Returns a reference to the Router contract.
    pub fn router(&self) -> &Contract<M> {
        &self.router
    }

    /// Returns an Option on a reference to the MarketAid contract.
    /// Market Aid isn't deployed on Kovan and Goerli - we can't always depend on it being there.
    #[cfg(feature = "aid")]
    pub fn market_aid(&self) -> &Contract<M> {
        &self.market_aid
    }

    /// Returns a reference to the ethers-rs chain enum.
    pub fn chain(&self) -> &Chain {
        &self.chain
    }

    /// Address associated with the current middleware, if it exists.
    pub fn get_address(&self) -> Option<Address> {
        self._internal_middleware.default_sender()
    }

    /// Are we on a legacy chain (pre EIP-1559)? If so, we have to use legacy TX calls...
    pub fn is_legacy(&self) -> bool {
        self.chain().is_legacy()
    }

    // let's add in some builders for numeraire::ChainNativeAsset
    // just to make it easier to build those things, since they need to be pinned to the same chain as the session
    /// Returns a [`ChainNativeAsset`] localized to the current chain.
    pub fn local_asset(&self, asset: Asset, size: U256) -> Result<ChainNativeAsset> {
        ChainNativeAsset::new(*self.chain(), asset, size)
    }

    /// Returns a [`ChainNativeAsset`] localized to the current chain, with size equal to zero.
    pub fn local_zero_asset(&self, asset: Asset) -> Result<ChainNativeAsset> {
        ChainNativeAsset::new_zero(*self.chain(), asset)
    }

    /// Returns a [`ChainNativeAsset`] localized to the current chain, with size equal to 2^256-1.
    pub fn local_max_asset(&self, asset: Asset) -> Result<ChainNativeAsset> {
        ChainNativeAsset::new_max(*self.chain(), asset)
    }

    /// Returns a [`ChainNativeAsset`] localized to the current chain. `human_size` is human readable, and is converted to wei behind the scenes.
    pub fn local_asset_human_decimal(
        &self,
        asset: Asset,
        human_size: Decimal,
    ) -> Result<ChainNativeAsset> {
        ChainNativeAsset::from_human_decimal(*self.chain(), asset, human_size)
    }

    /// Returns a [`ChainNativeAsset`] localized to the current chain. `human_size` is human readable, and is converted to wei behind the scenes.
    pub fn local_asset_string<T: ToString>(
        &self,
        asset: Asset,
        human_size: &T,
    ) -> Result<ChainNativeAsset> {
        ChainNativeAsset::from_human_string(*self.chain(), asset, human_size)
    }

    // RUBICON MARKET FUNCTIONS

    // first, we have the raw functions that interact with the contracts on chain

    // RUBICON BATH HOUSE FUNCTIONS
    /// Strategists have to be approved by the Rubicon protocol before they can place market making trades with pooled funds.
    /// This function returns true if the current middleware is an approved strategist.
    #[instrument(level = "debug", skip(self))]
    pub async fn self_is_approved_strategist(&self) -> Result<bool> {
        self.is_approved_strategist(
            self.get_address()
                .ok_or(anyhow!("ERROR: the Session doesn't have an address!"))?,
        )
        .await
    }

    /// Strategists have to be approved by the Rubicon protocol before they can place market making trades with pooled funds.
    /// This function returns true if the supplied address is an approved strategist.
    #[instrument(level = "debug", skip(self))]
    pub async fn is_approved_strategist(&self, addr: Address) -> Result<bool> {
        let receipt = self
            .bath_house()
            .method::<_, bool>("isApprovedStrategist", (addr,))?
            .call()
            .await?;
        Ok(receipt)
    }

    // MarketAid functions
    // first, the raw functions, later, the helper functions
    /// Returns a Result on a Vector of trade IDs. This requires the `aid` feature.
    #[instrument(level = "debug", skip(self))]
    #[cfg(feature = "aid")]
    pub async fn get_outstanding_strategist_trades(
        &self,
        asset: Address,
        quote: Address,
        strategist: Address,
    ) -> Result<Vec<U256>> {
        let ctr = self.market_aid();
        Ok(ctr
            .method::<_, Vec<U256>>("getOutstandingStrategistTrades", (asset, quote, strategist))?
            .call()
            .await?)
    }

    // UTILITY FUNCTIONS AND WHATNOT
    /// This checks that the bids and asks are:
    /// 1. all local to the same chain
    /// 2. that that chain is the same as the session chain
    /// 3. that the bids and asks are all conjugate in the same way (i.e. all bids are QUOTE->BASE and all bids are BASE->QUOTE)
    #[instrument(level = "trace", skip_all)]
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
}

impl<M: Middleware + Clone + 'static, S: Clone + Signer> RubiconSession<SignerMiddleware<M,S>>
{
    /*
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
    ) -> Result<ContractCall<SignerMiddleware<M,S>, U256>> {
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

    /// This is a market sell, where we spend pay_amt to buy as much as possible of buy_gem (and we get *at least* min_fill_amount)
    /// the returned value is the filled amount
    #[instrument(level = "debug", skip(self))]
    pub fn sell_all_amount(
        &self,
        pay_gem: Address,
        pay_amt: U256,
        buy_gem: Address,
        min_fill_amount: U256,
    ) -> Result<ContractCall<SignerMiddleware<M,S>, U256>> {
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

    /// This represents a market sell, where we sell the `source.size()` worth of `source.asset()`
    /// in exchange for some undetermined amount `target`
    #[instrument(level = "debug", skip(self))]
    pub fn market_sell(
        &self,
        source: &ChainNativeAsset,
        target: &Asset,
    ) -> Result<ContractCall<SignerMiddleware<M,S>, U256>> {
        if source.chain() != self.chain() {
            return Err(anyhow!(
                "[market_sell]: source chain does not match session chain! ({}!={})",
                source.chain(),
                self.chain()
            ));
        } else {
            self.sell_all_amount(
                source.address()?,
                source.size().clone(),
                target.to_address(self.chain())?,
                U256::zero(),
            )
        }
    }

    /// This represents a market sell, where we sell the `source.size()` worth of `source.asset()`
    /// in exchange for some undetermined amount `target`
    #[instrument(level = "debug", skip(self))]
    pub fn market_buy(
        &self,
        source: &Asset,
        target: &ChainNativeAsset,
    ) -> Result<ContractCall<SignerMiddleware<M,S>, U256>> {
        if target.chain() != self.chain() {
            return Err(anyhow!(
                "[market_sell]: target chain does not match session chain! ({}!={})",
                target.chain(),
                self.chain()
            ));
        } else {
            self.buy_all_amount(
                target.address()?,
                target.size().clone(),
                source.to_address(self.chain())?,
                U256::MAX,
            )
        }
    }

    /// This is used to construct a limit order, where we want to sell `pay_amt` of `pay_gem` for at least `buy_amt` of `buy_gem`.
    /// The `pos` parameter should be `None` unless you know the new position of the order in the sorted orderbook.
    /// `pay_gem` and `buy_gem` must not be equal.
    #[instrument(level = "debug", skip(self))]
    pub fn offer(
        &self,
        pay_amt: U256,
        pay_gem: Address,
        buy_amt: U256,
        buy_gem: Address,
        pos: Option<U256>,
    ) -> Result<ContractCall<SignerMiddleware<M,S>, U256>> {
        if pay_gem == buy_gem {
            // we can't sell `x` for `x`
            return Err(anyhow!(
                "[offer]: pay_gem and buy_gem are the same! ({}=={})",
                pay_gem,
                buy_gem
            ));
        }
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

    /// This constructs a limit order transaction.
    /// We want to sell `source.size()` of `source.asset()` for at least `target.size()` of `target.asset()`.
    /// `source.asset()` and `target.asset()` must not be equal.
    #[instrument(level = "debug", skip(self))]
    pub fn limit_order_bins(
        &self,
        source: &ChainNativeAsset,
        target: &ChainNativeAsset,
    ) -> Result<ContractCall<SignerMiddleware<M,S>, U256>> {
        self.offer(
            *source.size(),
            source.address()?,
            *target.size(),
            target.address()?,
            None,
        )
    }

    /// This constructs a limit order transaction.
    /// We sell `base_size` worth of `base` for `quote`, at a price greater than or equal to `price`.
    /// `price` has units `quote/base`. Both `price` and `base_size` are in human readable units, not wei.
    /// `base` and `quote` must not be equal.
    #[instrument(level = "debug", skip(self))]
    pub fn limit_sell(
        &self,
        base: &Asset,
        quote: &Asset,
        price: Decimal,
        base_size: Decimal,
    ) -> Result<ContractCall<SignerMiddleware<M,S>, U256>> {
        // what's the equivalent quote size?
        let quote_size = base_size * price;
        let base_bin: ChainNativeAsset = self.local_asset_human_decimal(*base, base_size)?;
        let quote_bin: ChainNativeAsset = self.local_asset_human_decimal(*quote, quote_size)?;
        self.limit_order_bins(&base_bin, &quote_bin)
    }

    /// This constructs a limit order transaction.
    /// We buy `base_size` worth of `base` for `quote`, at a price less than or equal to `price`.
    /// `price` has units `quote/base`. Both `price` and `base_size` are in human readable units, not wei.
    /// `base` and `quote` must not be equal.
    #[instrument(level = "debug", skip(self))]
    pub fn limit_buy(
        &self,
        base: &Asset,
        quote: &Asset,
        price: Decimal,
        base_size: Decimal,
    ) -> Result<ContractCall<SignerMiddleware<M,S>, U256>> {
        // what's the equivalent quote size?
        let quote_size = base_size * price;
        let base_bin: ChainNativeAsset = self.local_asset_human_decimal(*base, base_size)?;
        let quote_bin: ChainNativeAsset = self.local_asset_human_decimal(*quote, quote_size)?;
        self.limit_order_bins(&quote_bin, &base_bin)
    }

    /// This constructs a limit order transaction.
    /// We want to sell `quute_size` worth of `quote` for `base`, at a price greater than or equal to `price`.
    /// `price` has units `quote/base`. Both `price` and `quote_size` are in human readable units, not wei.
    /// `base` and `quote` must not be equal.
    #[instrument(level = "debug", skip(self))]
    pub fn conj_limit_sell(
        &self,
        base: &Asset,
        quote: &Asset,
        price: Decimal,
        quote_size: Decimal,
    ) -> Result<ContractCall<SignerMiddleware<M,S>, U256>> {
        // what's the equivalent base size?
        let base_size = quote_size / price;
        let base_bin: ChainNativeAsset = self.local_asset_human_decimal(*base, base_size)?;
        let quote_bin: ChainNativeAsset = self.local_asset_human_decimal(*quote, quote_size)?;
        self.limit_order_bins(&quote_bin, &base_bin)
    }
    /// This constructs a limit order transaction.
    /// We want to buy `quute_size` worth of `quote` for `base`, at a price less than or equal to `price`.
    /// `price` has units `quote/base`. Both `price` and `quote_size` are in human readable units, not wei.
    /// `base` and `quote` must not be equal.
    #[instrument(level = "debug", skip(self))]
    pub fn conj_limit_buy(
        &self,
        base: &Asset,
        quote: &Asset,
        price: Decimal,
        quote_size: Decimal,
    ) -> Result<ContractCall<SignerMiddleware<M,S>, U256>> {
        // what's the equivalent base size?
        let base_size = quote_size / price;
        let base_bin: ChainNativeAsset = self.local_asset_human_decimal(*base, base_size)?;
        let quote_bin: ChainNativeAsset = self.local_asset_human_decimal(*quote, quote_size)?;
        self.limit_order_bins(&base_bin, &quote_bin)
    }

    /// Cancels an order that's already on the Rubicon book
    #[instrument(level = "debug", skip(self))]
    pub fn cancel(&self, order_id: U256) -> Result<ContractCall<SignerMiddleware<M,S>, U256>> {
        let tx = if self.is_legacy() {
            self.market()
                .method::<_, U256>("cancel", (order_id,))?
                .legacy()
        } else {
            self.market().method::<_, U256>("cancel", (order_id,))?
        };
        Ok(tx)
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
     * - scrub strategist orders (DONE)
     */

    // these are the raw functions for interacting with the chain
    // they are not meant to be used directly, but rather through the
    // helper functions below
    // ask_num is ask_numerator, ask_dem -> ask_denominator
    // token_pair is of the form [base, quote]
    // returns a trade ID (U256)
    /// This is used to place a strategist trade on the market.
    /// Trades are placed in pairs - a bid and an ask.
    /// `ask_num` and `ask_den` are the numerator and denominator of the ask price, respectively. The same is true of the bid.
    /// `token_pair` is of the form [base, quote], where `base != quote`.
    #[instrument(level = "debug", skip(self))]
    pub fn place_market_making_trades(
        &self,
        token_pair: [Address; 2],
        ask_num: U256,
        ask_den: U256,
        bid_num: U256,
        bid_den: U256,
    ) -> Result<ContractCall<SignerMiddleware<M,S>, ()>> {
        let tx = match self.is_legacy() {
            true => self
                .pair()
                .method::<_, ()>(
                    "placeMarketMakingTrades",
                    (token_pair, ask_num, ask_den, bid_num, bid_den),
                )?
                .legacy(),
            false => self.pair().method::<_, ()>(
                "placeMarketMakingTrades",
                (token_pair, ask_num, ask_den, bid_num, bid_den),
            )?,
        };
        Ok(tx)
    }

    // INCOMPLETE: shouldn't this return a vector of Order IDs?
    /// This returns a [`ContractCall`] that will place a series of paired market making trades.
    #[instrument(level = "debug", skip(self))]
    pub fn batch_place_market_making_trades(
        &self,
        token_pair: [Address; 2],
        ask_nums: Vec<U256>,
        ask_dems: Vec<U256>,
        bid_nums: Vec<U256>,
        bid_dems: Vec<U256>,
    ) -> Result<ContractCall<SignerMiddleware<M,S>, ()>> {
        if !(ask_nums.len() == ask_dems.len() && bid_nums.len() == bid_dems.len()) {
            // there's some mismatch in the input values...
            // we should return an error and log it
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
    /// This returns a [`ContractCall`] that requotes the given pair of orders.
    #[instrument(level = "debug", skip(self))]
    pub fn requote_offers(
        &self,
        order_id: U256,
        token_pair: [Address; 2],
        ask_num: U256,
        ask_dem: U256,
        bid_num: U256,
        bid_dem: U256,
    ) -> Result<ContractCall<SignerMiddleware<M,S>, ()>> {
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
    /** This returns a [`ContractCall`] that requotes a series of paired strategist orders. */
    #[instrument(level = "debug", skip(self))]
    pub fn batch_requote_offers(
        &self,
        ids: Vec<U256>,
        token_pair: [Address; 2],
        ask_nums: Vec<U256>,
        ask_dems: Vec<U256>,
        bid_nums: Vec<U256>,
        bid_dems: Vec<U256>,
    ) -> Result<ContractCall<SignerMiddleware<M,S>, ()>> {
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
    /** This returns a [`ContractCall`] that cancels an outstanding strategist orders. */
    #[instrument(level = "debug", skip(self))]
    pub fn scrub_strategist_trade(&self, trade_id: U256) -> Result<ContractCall<SignerMiddleware<M,S>, ()>> {
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
    /** This returns a [`ContractCall`] that cancels a list of outstanding strategist orders.  */
    #[instrument(level = "debug", skip(self))]
    pub fn scrub_strategist_trades(&self, trade_ids: Vec<U256>) -> Result<ContractCall<SignerMiddleware<M,S>, ()>> {
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
    pub fn tailoff(
        &self,
        target_pool: Address,
        token_to_handle: Address,
        target_token: Address,
        strat_util: Address,
        amount: U256,
        hurdle: U256,
        pool_fee: u32,
    ) -> Result<ContractCall<SignerMiddleware<M,S>, ()>> {
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
    pub fn tailoff_multi(
        &self,
        target_pool: Address,
        amount: U256,
        assets: Vec<Address>,
        fees: Vec<u32>,
        hurdle: U256,
        strat_util: Address,
    ) -> Result<ContractCall<SignerMiddleware<M,S>, ()>> {
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
    pub fn rebalance_pair(
        &self,
        asset_rebal_amt: U256,
        quote_rebal_amt: U256,
        underlying_asset: Address,
        underlying_quote: Address,
    ) -> Result<ContractCall<SignerMiddleware<M,S>, ()>> {
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
}

