#![allow(unused)]
use std::collections::{BTreeMap, HashMap};

pub mod bybit;
pub mod endpoints;
pub mod errors;
pub mod helpers;

use bybit::{
    http_manager::{HttpManager, Manager},
    trade::{self, Trade},
};
use errors::app_error::AppError;
use serde_json::Value;

fn main() {
    dotenv::dotenv().ok();
    let rt = tokio::runtime::Builder::new_current_thread()
        .worker_threads(2)
        .enable_all()
        .build()
        .unwrap();
    if let Err(err) = rt.block_on(start_app()) {
        println!("Error: {}", err);
    }
}

async fn start_app() -> Result<(), AppError> {
    let http_api_key =
        std::env::var("API_KEY").map_err(|_| AppError::EnvVarMissing("API_KEY".to_string()))?;
    let http_api_secret = std::env::var("API_SECRET")
        .map_err(|_| AppError::EnvVarMissing("API_SECRET".to_string()))?;

    let testnet_str =
        std::env::var("API_TEST").map_err(|_| AppError::EnvVarMissing("API_TEST".to_string()))?;
    let testnet = if testnet_str == "true" { true } else { false };
    let manager = HttpManager::new(http_api_key, http_api_secret, testnet);

    // to get KLINe data

    println!("TO MARKET GET KLINE DAT");
    let mut query: HashMap<String, String> = HashMap::new();
    query.insert("category".to_string(), "inverse".to_string());
    query.insert("symbol".to_string(), "BTCUSD".to_string());
    query.insert("interval".to_string(), "60".to_string());
    query.insert("start".to_string(), "1670601600000".to_string());
    query.insert("end".to_string(), "1670608800000".to_string());

    match manager
        .submit_request(reqwest::Method::GET, "/v5/market/kline", query, true)
        .await
    {
        Ok(result) => println!("{:?}", result["result"].clone()),
        Err(e) => println!("{:?}", e),
    };

    println!("TO PLACE AN ORDER");
    let mut query: HashMap<String, String> = HashMap::new();
    query.insert("category".to_owned(), "linear".to_owned());
    query.insert("symbol".to_owned(), "BTCUSDT".to_owned());
    query.insert("orderType".to_owned(), "Limit".to_owned());
    query.insert("qty".to_owned(), "0.04".to_owned());
    query.insert("price".to_owned(), "27200".to_owned());
    query.insert("side".to_owned(), "Buy".to_owned());
    // query.insert("timeInForce".to_owned(), "FillOrKill".to_owned());

    let trade: trade::TradeHTTP = trade::TradeHTTP::new(manager);

    match trade.place_order(query).await {
        Ok(result) => println!("{:?}", result),
        Err(e) => println!("{:?}", e),
    }

    println!("TO AN ORDER");

    ////
    ///
    ///  To a single order
    ///
    ///
    let mut query: HashMap<String, String> = HashMap::new();
    query.insert("category".to_owned(), "linear".to_owned());
    query.insert("limit".to_owned(), "1".to_owned());
    query.insert("symbol".to_owned(), "BTCUSDT".to_owned());
    query.insert("openOnly".to_owned(), "0".to_owned());

    match trade.get_open_orders(query).await {
        Ok(result) => println!("{:?}", result),
        Err(e) => println!("{:?}", e),
    }

    println!("TO CANCEL AN ORDER");

    ////
    ///
    ///  To cancel a single order
    ///
    ///
    let mut query: HashMap<String, String> = HashMap::new();
    query.insert("category".to_owned(), "linear".to_owned());
    query.insert(
        "orderId".to_owned(),
        "3380b972-a334-4d00-87e9-3423fa27602f".to_owned(),
    );
    query.insert("symbol".to_owned(), "BTCUSDT".to_owned());
    query.insert("settleCoin".to_owned(), "USDT".to_owned());

    match trade.cancel_order(query).await {
        Ok(result) => println!("{:?}", result),
        Err(e) => println!("{:?}", e),
    }

    println!("TO CANCEL ALL ORDERS");
    ////
    ///
    ///  To cancel All Orders
    ///
    ///
    let mut query: HashMap<String, String> = HashMap::new();
    query.insert("category".to_owned(), "linear".to_owned());
    query.insert("symbol".to_owned(), "".to_owned());
    query.insert("settleCoin".to_owned(), "USDT".to_owned());

    match trade.cancel_all_orders(query).await {
        Ok(result) => println!("{:?}", result),
        Err(e) => println!("{:?}", e),
    }

    Ok(())
}