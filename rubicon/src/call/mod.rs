// TX HANDLING...

use super::session::TxResult;
use ethers::{abi::Detokenize, prelude::builders::ContractCall, providers::Middleware};
use tracing::{event, Level};

/**
 * This is a helper function that executes a transaction, and waits for a receipt.
 * By using the (ContractCall<M,D>, Future<...>) model for all of our transactions, we should retain the ability to
 * cancel stuff in flight, by calling the ContractCall part of the returned tuple again.
 */
async fn handle_contract_call<M: Middleware + 'static, T: Detokenize>(
    call: ContractCall<M, T>,
) -> TxResult {
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

/**
 * This is a helper function that executes a transaction, and waits for a receipt.
 * By using the (ContractCall<M,D>, Future<...>) model for all of our transactions, we should retain the ability to
 * cancel stuff in flight, by calling the ContractCall part of the returned tuple again.
 */
pub async fn handle_contract_call_v2<M: Middleware + 'static, T: Detokenize>(
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
