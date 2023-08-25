use reqwest::Url;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct WalletQuery {
    data: Data,
}

#[derive(Serialize, Deserialize)]
pub struct Data {
    address: String,
    updated_at: String,
    next_update_at: String,
    quote_currency: String,
    chain_id: i64,
    chain_name: String,
    items: Vec<Item>,
}

#[derive(Serialize, Deserialize)]
pub struct Item {
    contract_decimals: Option<i64>,
    contract_name: Option<String>,
    contract_ticker_symbol: Option<String>,
    contract_address: String,
    supports_erc: Option<Vec<SupportsErc>>,
    logo_url: String,
    last_transferred_at: String,
    native_token: bool,
    #[serde(rename = "type")]
    item_type: Type,
    is_spam: bool,
    balance: String,
    #[serde(rename = "balance_24h")]
    balance_24_h: String,
    quote_rate: Option<f64>,
    #[serde(rename = "quote_rate_24h")]
    quote_rate_24_h: Option<f64>,
    quote: Option<f64>,
    pretty_quote: Option<String>,
    #[serde(rename = "quote_24h")]
    quote_24_h: Option<f64>,
    #[serde(rename = "pretty_quote_24h")]
    pretty_quote_24_h: Option<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Type {
    Cryptocurrency,
    Dust,
    Stablecoin,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SupportsErc {
    Erc20,
}

impl WalletQuery {
    async fn get(public_address: &String) -> WalletQuery {
        let api_key = dotenv::var("COVALENT_API_TOKEN").unwrap();
        let url = format!(
            "https://api.covalenthq.com/v1/1/address/{}/balances_v2/?&key={}",
            public_address, api_key
        );
        let url = Url::parse(&*url).unwrap();
        let resp = reqwest::get(url)
            .await
            .unwrap()
            .json::<WalletQuery>()
            .await
            .unwrap();
        return resp;
    }
}

//items
#[tokio::main]
pub async fn print(
    public_address: String,
    coins: usize,
) -> HashMap<std::option::Option<std::string::String>, std::option::Option<f64>> {
    let result: WalletQuery = WalletQuery::get(&public_address).await;
    let mut map = HashMap::new();

    let mut i = 0;
    while i < coins {
        map.insert(
            result.data.items[i].contract_name.clone(),
            result.data.items[i].quote,
        );
        i = i + 1;
    }

    return map;
}
