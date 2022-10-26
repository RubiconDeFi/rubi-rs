use ethers::{middleware::SignerMiddleware, providers::{Provider, Ws}, signers::{coins_bip39::English, MnemonicBuilder, LocalWallet, Signer}, types::{Address, U256}};
use std::{env, sync::Arc};

use rubicon::prelude::*;



#[tokio::main(flavor = "multi_thread")]
async fn main() {
    let env_args = env::args().collect::<Vec<_>>();
    let env_path = env_args.get(1).unwrap();
    dotenv::from_path(env_path);

    let provider_url = env::var("PROVIDER_URL").unwrap();
    let phrase = env::var("WALLET_PHRASE").unwrap();

    let provider = Provider::<Ws>::connect(&provider_url).await.unwrap();
    let wallet: LocalWallet = MnemonicBuilder::<English>::default()
        .phrase(phrase.as_str())
        .build()
        .unwrap()
        .with_chain_id(10_u64);

    let client: SignerMiddleware<Provider<_>, LocalWallet> = SignerMiddleware::new(provider, wallet);
    
    let conn = Arc::new(RubiconSession::new_mainnet(client));
    println!("Connected to Rubicon with address = {:?}", conn.get_address());


    // USDC: 0x7F5c764cBc14f9669B88837ca1490cCa17c31607
    // DAI: 0xDA10009cBd5D07dd0CeCc66161FC93D7c9000da1
    let usdc_hx = hex::decode("7F5c764cBc14f9669B88837ca1490cCa17c31607").unwrap();
    let usdc_addr = Address::from_slice(usdc_hx.as_slice());

    let dai_hx = hex::decode("DA10009cBd5D07dd0CeCc66161FC93D7c9000da1").unwrap();
    let dai_addr = Address::from_slice(dai_hx.as_slice());

    let usdc_size = U256::from_str_radix("500000", 10).unwrap(); // 50 cents
    let dai_size = U256::from_str_radix("1000000000000000000", 10).unwrap(); // 1 DAI
    let max_u256 = U256::MAX;
    let zero_u256 = U256::zero();


    // let (_call, fut) = conn.buy_all_amount(dai_addr, dai_size, usdc_addr, max_u256).unwrap();
    // let receipt = fut.await;

    // let fut = conn.handle_contract_call_v2(call);
    // let call = conn.buy_all_amount_v2(dai_addr, dai_size, usdc_addr, max_u256).unwrap();
    // //tokio::time::sleep(std::time::Duration::from_millis(30)).await;
    // //let cancel = conn.handle_contract_call_v2(call.clone());
    // let receipt = fut.await;
    //let cancel = cancel.await;

    // println!("{:?}", &receipt);



}