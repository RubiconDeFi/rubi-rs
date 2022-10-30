#![allow(dead_code)]

use super::*;

use ethers::{
    contract::Contract,
    prelude::{EthEvent, PubsubClient},
    providers::{Middleware, StreamExt},
};
use tracing::{instrument, Level, event};
use flume;
use postage::{
    broadcast,
    sink::{SendError, Sink},
};

impl<M: Middleware + Clone + 'static> RubiconSession<M>
where
    <M as Middleware>::Provider: PubsubClient,
{
    /*
     * What events should I be listening to???
     * Maybe we'll just let the users decide...
     */

    // first, we have Pair events
    // both Broadcast and Flume

    /// This subscribes to an [`EthEvent`] `E` on the RubiconPair contract, and forwards the resulting events stream over a [`broadcast`] channel.
    /// [`broadcast`] channels send every event to every receiver.
    #[instrument(level="debug", skip_all)]
    pub async fn broadcast_pair_events<E: EthEvent + Clone + std::fmt::Debug + 'static>(
        &self,
        tx: broadcast::Sender<E>,
    ) {
        broadcast_events_stream(self.pair(), tx).await;
    }

    /// This subscribes to an [`EthEvent`] `E` on the RubiconPair contract, filter maps the resulting event stream, and forwards the results over a [`broadcast`] channel.
    /// [`broadcast`] channels send every event to every receiver.
    #[instrument(level="debug", skip_all)]
    pub async fn broadcast_filter_pair_events<
        E: EthEvent + Clone + std::fmt::Debug + 'static,
        K: Clone + std::fmt::Debug,
        F: Fn(E) -> Option<K>,
    >(
        &self,
        tx: broadcast::Sender<K>,
        f: F,
    ) {
        broadcast_filter_events_stream(self.pair(), tx, f).await;
    }

    /// This subscribes to an [`EthEvent`] `E` on the RubiconPair contract, and forwards the resulting events stream over a [`flume`] channel.
    /// [`flume`] channels are MPMC channels. Only one receiver receives a given event (the first one to call `recv`). Useful for work stealing.
    #[instrument(level="debug", skip_all)]
    pub async fn flume_pair_events<E: EthEvent + Clone + std::fmt::Debug + 'static>(
        &self,
        tx: flume::Sender<E>,
    ) {
        flume_events_stream(self.pair(), tx).await;
    }

    /// This subscribes to an [`EthEvent`] `E` on the RubiconPair contract, filter maps the resulting event stream, and forwards the results over a [`flume`] channel.
    /// [`flume`] channels are MPMC channels. Only one receiver receives a given event (the first one to call `recv`). Useful for work stealing.
    #[instrument(level="debug", skip_all)]
    pub async fn flume_filter_pair_events<
        E: EthEvent + Clone + std::fmt::Debug + 'static,
        K: Clone + std::fmt::Debug,
        F: Fn(E) -> Option<K>,
    >(
        &self,
        tx: flume::Sender<K>,
        filter: F,
    ) {
        flume_filter_events_stream(self.pair(), tx, filter).await;
    }

    // next, market events (both Broadcast and Flume)
    /// This subscribes to an [`EthEvent`] `E` on the RubiconMarket contract, and forwards the resulting events stream over a [`broadcast`] channel.
    /// [`broadcast`] channels send every event to every receiver.
    #[instrument(level="debug", skip_all)]
    pub async fn broadcast_market_events<E: EthEvent + Clone + std::fmt::Debug + 'static>(
        &self,
        tx: broadcast::Sender<E>,
    ) {
        broadcast_events_stream(self.market(), tx).await;
    }

    /// This subscribes to an [`EthEvent`] `E` on the RubiconMarket contract, filter maps the resulting event stream, and forwards the results over a [`broadcast`] channel.
    /// [`broadcast`] channels send every event to every receiver.
    #[instrument(level="debug", skip_all)]
    pub async fn broadcast_filter_market_events<
        E: EthEvent + Clone + std::fmt::Debug + 'static,
        K: Clone + std::fmt::Debug,
        F: Fn(E) -> Option<K>,
    >(
        &self,
        tx: broadcast::Sender<K>,
        f: F,
    ) {
        broadcast_filter_events_stream(self.market(), tx, f).await;
    }

    /// This subscribes to an [`EthEvent`] `E` on the RubiconMarket contract, and forwards the resulting events stream over a [`flume`] channel.
    /// [`flume`] channels are MPMC channels. Only one receiver receives a given event (the first one to call `recv`). Useful for work stealing.
    #[instrument(level="debug", skip_all)]
    pub async fn flume_market_events<E: EthEvent + Clone + std::fmt::Debug + 'static>(
        &self,
        tx: flume::Sender<E>,
    ) {
        flume_events_stream(self.market(), tx).await;
    }

    /// This subscribes to an [`EthEvent`] `E` on the RubiconMarket contract, filter maps the resulting event stream, and forwards the results over a [`flume`] channel.
    /// [`flume`] channels are MPMC channels. Only one receiver receives a given event (the first one to call `recv`). Useful for work stealing.
    #[instrument(level="debug", skip_all)]
    pub async fn flume_filter_market_events<
        E: EthEvent + Clone + std::fmt::Debug + 'static,
        K: Clone + std::fmt::Debug,
        F: Fn(E) -> Option<K>,
    >(
        &self,
        tx: flume::Sender<K>,
        filter: F,
    ) {
        flume_filter_events_stream(self.market(), tx, filter).await;
    }
}

// might reduce overhead in the Tokio scheduler???
// since we wouldn't have an .await.await, merely an .await
#[inline]
#[instrument(level="trace", skip_all)]
async fn broadcast_events_stream<
    M: Middleware + 'static,
    E: EthEvent + Clone + std::fmt::Debug + 'static,
>(
    contract: &Contract<M>,
    mut tx: broadcast::Sender<E>,
) where
    <M as Middleware>::Provider: PubsubClient,
{
    let event = contract.event::<E>();
    let mut evt_stream = match event.subscribe().await {
        Ok(evt_stream) => {
            event!(Level::TRACE, "[broadcast_events_stream]: subscribed to event stream");
            evt_stream
        },
        Err(e) => {
            event!(Level::ERROR, "[broadcast_events_stream]: failed to subscribe to event stream with error {e}");
            return;
        }
    };

    while let Some(rst_event) = evt_stream.next().await {
        match rst_event {
            Ok(new_event) => {
                if let Err(e) = tx.send(new_event).await {
                    event!(Level::WARN, "[broadcast_events_stream]: ERROR: failed to forward event to channel with error {e}");
                } else {
                    // we successfully forwarded something over the channel
                    event!(Level::TRACE, "[broadcast_events_stream]: successfully forwarded event to channel");
                }
            }
            Err(e) => event!(Level::WARN, "[broadcast_events_stream]: new event generated error {e}"), // are these fatal???
        }
    }
    event!(Level::WARN, "[broadcast_events_stream]: event stream ended");
}

#[inline]
#[instrument(level="trace", skip_all)]
async fn broadcast_filter_events_stream<
    M: Middleware + 'static,
    E: EthEvent + Clone + std::fmt::Debug + 'static,
    K: Clone + std::fmt::Debug,
    F: Fn(E) -> Option<K>,
>(
    contract: &Contract<M>,
    mut tx: broadcast::Sender<K>,
    filter: F,
) where
    <M as Middleware>::Provider: PubsubClient,
{
    let event = contract.event::<E>();
    let mut evt_stream = match event.subscribe().await {
        Ok(evt_stream) => {
            event!(Level::TRACE, "[broadcast_filter_events_stream]: subscribed to event stream");
            evt_stream
        },
        Err(e) => {
            event!(Level::ERROR, "[broadcast_filter_events_stream]: failed to subscribe to event stream with error {e}");
            return;
        }
    };

    while let Some(rst_event) = evt_stream.next().await {
        match rst_event {
            Ok(new_event) => {
                if let Some(tfm) = filter(new_event.clone()) {
                    if let Err(e) = tx.send(tfm).await {
                        event!(Level::WARN, "[broadcast_filter_events_stream]: ERROR: failed to forward event to channel with error {e}");
                    } else {
                        // we successfully forwarded something over the channel
                        event!(Level::TRACE, "[broadcast_filter_events_stream]: successfully forwarded event to channel");
                    }
                } else {
                    // we don't need to forward anything to the channel
                    event!(Level::TRACE, "[broadcast_filter_events_stream]: received event that didn't match filter");
                }
            }
            Err(e) => event!(Level::WARN, "[broadcast_filter_events_stream]: new event generated error {e}"), // are these fatal???
        }
    }
    event!(Level::WARN, "[broadcast_filter_events_stream]: event stream ended");
}

#[inline]
#[instrument(level="trace", skip_all)]
async fn flume_events_stream<
    M: Middleware + 'static,
    E: EthEvent + Clone + std::fmt::Debug + 'static,
>(
    contract: &Contract<M>,
    tx: flume::Sender<E>,
) where
    <M as Middleware>::Provider: PubsubClient,
{
    let event = contract.event::<E>();
    let mut evt_stream = match event.subscribe().await {
        Ok(evt_stream) => {
            event!(Level::TRACE, "[flume_events_stream]: subscribed to event stream");
            evt_stream
        },
        Err(e) => {
            event!(Level::ERROR, "[flume_events_stream]: failed to subscribe to event stream with error {e}");
            return;
        }
    };

    while let Some(rst_event) = evt_stream.next().await {
        match rst_event {
            Ok(new_event) => {
                if let Err(e) = tx.send(new_event) {
                    event!(Level::WARN, "[flume_events_stream]: ERROR: failed to forward event to channel with error {e}");
                } else {
                    // we successfully forwarded something over the channel
                    event!(Level::TRACE, "[flume_events_stream]: successfully forwarded event to channel");
                }
            }
            Err(e) => event!(Level::WARN, "[flume_events_stream]: new event generated error {e}"), // are these fatal???
        }
    }
    event!(Level::WARN, "[flume_events_stream]: event stream ended");
}

#[inline]
#[instrument(level="trace", skip_all)]
async fn flume_filter_events_stream<
    M: Middleware + 'static,
    E: EthEvent + Clone + std::fmt::Debug + 'static,
    K: Clone + std::fmt::Debug,
    F: Fn(E) -> Option<K>,
>(
    contract: &Contract<M>,
    tx: flume::Sender<K>,
    filter: F,
) where
    <M as Middleware>::Provider: PubsubClient,
{
    let event = contract.event::<E>();
    let mut evt_stream = match event.subscribe().await {
        Ok(evt_stream) => {
            event!(Level::TRACE, "[flume_filter_events_stream]: subscribed to event stream");
            evt_stream
        },
        Err(e) => {
            event!(Level::ERROR, "[flume_filter_events_stream]: failed to subscribe to event stream with error {e}");
            return;
        }
    };

    while let Some(rst_event) = evt_stream.next().await {
        match rst_event {
            Ok(new_event) => {
                if let Some(tfm) = filter(new_event.clone()) {
                    if let Err(e) = tx.send(tfm) {
                        event!(Level::WARN, "[flume_filter_events_stream]: ERROR: failed to forward event to channel with error {e}");
                    } else {
                        // we successfully forwarded something over the channel
                        event!(Level::TRACE, "[flume_filter_events_stream]: successfully forwarded event to channel");
                    }
                } else {
                    // we don't need to forward anything to the channel
                    event!(Level::TRACE, "[flume_filter_events_stream]: received event that didn't match filter");
                }
            }
            Err(e) => event!(Level::WARN, "[flume_filter_events_stream]: new event generated error {e}"), // are these fatal???
        }
    }
    event!(Level::WARN, "[flume_filter_events_stream]: event stream ended");
}