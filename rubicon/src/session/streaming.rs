use super::events;
use super::*;
use anyhow::{anyhow, Result};

use ethers::{
    abi::Detokenize,
    contract::Contract,
    core::types::{Address, BlockNumber, Chain, TransactionReceipt, U256},
    prelude::{builders::ContractCall, EthEvent, PubsubClient},
    providers::{Middleware, StreamExt},
};
use futures::Future;
use numeraire::prelude::*;

use std::convert::Into;

use std::sync::Arc;
use tracing::{event, instrument, Level};

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

    async fn broadcast_pair_events<E: EthEvent + Clone + std::fmt::Debug + 'static>(&self, tx: broadcast::Sender<E>) {
        broadcast_events_stream(self.pair(), tx).await;
    }

    async fn broadcast_filter_transform_pair_events<E: EthEvent + Clone + std::fmt::Debug + 'static,
    K: Clone + std::fmt::Debug,
    F: Fn(E) -> Option<K>,>(&self, tx: broadcast::Sender<K>, f: F) {
        broadcast_filter_transform_events_stream(self.pair(), tx, f).await;
    }

    async fn flume_pair_events<E: EthEvent + Clone + std::fmt::Debug + 'static>(&self, tx: flume::Sender<E>,) {
        flume_events_stream(self.pair(), tx).await;
    }
    
}

// might reduce overhead in the Tokio scheduler???
// since we wouldn't have an .await.await, merely an .await
#[inline]
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
    let mut evt_stream = event.subscribe().await.unwrap();

    while let Some(rst_event) = evt_stream.next().await {
        match rst_event {
            Ok(new_event) => {
                if let Err(SendError(failed_event)) = tx.send(new_event.clone()).await {
                    println!(
                        "[forward_events_stream]: ERROR: failed to forward event {:?} to channel!",
                        failed_event
                    );
                }
            }
            Err(e) => println!("[forward_events_stream]: new event generated error {e}"), // are these fatal???
        }
    }
}

#[inline]
async fn broadcast_filter_transform_events_stream<
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
    let mut evt_stream = event.subscribe().await.unwrap();

    while let Some(rst_event) = evt_stream.next().await {
        if rst_event.is_err() {
            // we had some kind of error in the event stream
            continue;
        }

        if let Some(tfm) = filter(rst_event.unwrap()) {
            if let Err(SendError(failed_event)) = tx.send(tfm).await {
                println!(
                    "[forward_events_stream]: ERROR: failed to forward event {:?} to channel!",
                    failed_event
                );
            }
        }
    }
}

#[inline]
async fn flume_events_stream<
    M: Middleware + 'static,
    E: EthEvent + Clone + std::fmt::Debug + 'static,
>(
    contract: &Contract<M>,
    mut tx: flume::Sender<E>,
) where
    <M as Middleware>::Provider: PubsubClient,
{
    let event = contract.event::<E>();
    let mut evt_stream = event.subscribe().await.unwrap();

    while let Some(rst_event) = evt_stream.next().await {
        match rst_event {
            Ok(new_event) => {
                if let Err(e) = tx.send(new_event.clone()) {
                    println!(
                        "[forward_events_stream]: ERROR: failed to forward event to channel with error {e}"
                    );
                }
            }
            Err(e) => println!("[forward_events_stream]: new event generated error {e}"), // are these fatal???
        }
    }
}

#[inline]
async fn flume_filter_transform_events_stream<
    M: Middleware + 'static,
    E: EthEvent + Clone + std::fmt::Debug + 'static,
    K: Clone + std::fmt::Debug,
    F: Fn(E) -> Option<K>,
>(
    contract: &Contract<M>,
    mut tx: flume::Sender<K>,
    filter: F,
) where
    <M as Middleware>::Provider: PubsubClient,
{
    let event = contract.event::<E>();
    let mut evt_stream = event.subscribe().await.unwrap();

    while let Some(rst_event) = evt_stream.next().await {
        match rst_event {
            Ok(new_event) => {
                if let Some(tfm) = filter(new_event.clone()) {
                    if let Err(e) = tx.send(tfm) {
                        println!(
                            "[forward_events_stream]: ERROR: failed to forward event to channel with error {e}"
                        );
                    }
                }
            }
            Err(e) => println!("[forward_events_stream]: new event generated error {e}"), // are these fatal???
        }
    }
}

// might reduce overhead in the Tokio scheduler???
// since we wouldn't have an .await.await, merely an .await
//#[inline]
/*async fn forward_events_stream<
    M: Middleware + 'static,
    E: EthEvent + Clone + std::fmt::Debug + 'static,
>(
    contract: &Contract<M>,
    mut tx: flume::Sender<E>,
) where
    <M as Middleware>::Provider: PubsubClient,
{
    let event = contract.event::<E>();
    let mut evt_stream = event.subscribe().await.unwrap();

    while let Some(rst_event) = evt_stream.next().await {
        match rst_event {
            Ok(new_event) => {
                if let Err(SendError(failed_event)) = tx.send(new_event.clone()).await {
                    println!(
                        "[forward_events_stream]: ERROR: failed to forward event {:?} to channel!",
                        failed_event
                    );
                }
            }
            Err(e) => println!("[forward_events_stream]: new event generated error {e}"), // are these fatal???
        }
    }
}*/

async fn proc_forward_events_stream<
    M: Middleware + 'static,
    E: EthEvent + std::fmt::Debug + 'static,
    T: Clone + std::fmt::Debug,
    F: Fn(E) -> T,
>(
    contract: &Contract<M>,
    mut tx: broadcast::Sender<T>,
    func: F,
) where
    <M as Middleware>::Provider: PubsubClient,
{
    let event = contract.event::<E>();
    let mut evt_stream = event.subscribe().await.unwrap();

    while let Some(rst_event) = evt_stream.next().await {
        match rst_event {
            Ok(new_event) => {
                if let Err(SendError(failed_event)) = tx.send(func(new_event)).await {
                    println!(
                        "[forward_events_stream]: ERROR: failed to forward event {:?} to channel!",
                        failed_event
                    );
                }
            }
            Err(e) => println!("[forward_events_stream]: new event generated error {e}"), // are these fatal???
        }
    }
}

async fn local_filter_forward_events_stream<
    M: Middleware + 'static,
    E: EthEvent + std::fmt::Debug + 'static,
    T: Clone + std::fmt::Debug,
    F: Fn(&E) -> Option<T>,
>(
    contract: &Contract<M>,
    mut tx: broadcast::Sender<T>,
    filter: F,
) where
    <M as Middleware>::Provider: PubsubClient,
{
    let event = contract.event::<E>();
    let mut evt_stream = event.subscribe().await.unwrap();

    while let Some(rst_event) = evt_stream.next().await {
        if let Err(e) = rst_event {
            println!("[forward_events_stream]: new event generated error {e}"); // are these fatal???
            continue;
        }

        // now, we know that we've passed the filter
        // so we should full send it

        match rst_event {
            Ok(new_event) => {
                let tmp = filter(&new_event);
                if let Some(new_event) = tmp {
                    if let Err(SendError(failed_event)) = tx.send(new_event).await {
                        println!(
                            "[forward_events_stream]: ERROR: failed to forward event {:?} to channel!",
                            failed_event
                        );
                    }
                }
            }
            Err(e) => println!("[forward_events_stream]: new event generated error {e}"), // are these fatal???
        }
    }
}
