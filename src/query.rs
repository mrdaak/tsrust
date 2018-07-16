// extern
use base64::{decode, encode};
use hmac::{Hmac, Mac};
use rand;
use reqwest;
use reqwest::header::{qitem, AcceptEncoding, Authorization, Encoding, Headers, UserAgent};
use serde_json::to_string;
use sha2::Sha512;
use strum::AsStaticRef;
use url::form_urlencoded::byte_serialize;

// built-in
use std::str;

use values::*;

const API_PUBLIC_KEY: &str = "";
const API_PRIVATE_KEY: &str = "";
const API_URL: &str = "https://tradesatoshi.com/api/";

#[derive(AsStaticStr)]
enum Api {
    Public,
    Private,
}

struct Query {
    kind: Api,
    endpoint: String,
    params: Option<Params>,
}

impl Query {
    fn new(endpoint: String, kind: Api) -> Query {
        Query {
            endpoint,
            kind,
            params: None,
        }
    }

    fn params(mut self, params: Params) -> Self {
        self.params = Some(params);
        self
    }

    fn run(self) -> Result<reqwest::Response, reqwest::Error> {
        let mut url: String = format!(
            "{}{}/{}",
            API_URL,
            self.kind.as_static().to_lowercase(),
            self.endpoint
        ).to_owned();
        println!("url: {}", url);
        match self.kind {
            Api::Public => {
                match self.params {
                    Some(params) => url.push_str(&params.to_query_params()),
                    None => (),
                }
                reqwest::get(&url)
            }
            Api::Private => {
                let headers: Headers = generate_header(&url, &self.params.as_ref().unwrap());
                let client = reqwest::Client::new();
                let response = client
                    .post(&url)
                    .json(&self.params.unwrap())
                    .headers(headers)
                    .send();
                response
            }
        }
    }
}

#[derive(QueryParams, Serialize, Debug)]
struct Params {
    #[serde(skip_serializing_if = "Option::is_none", rename = "Market")]
    market: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Count")]
    count: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Currency")]
    currency: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Type")]
    typeo: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Depth")]
    depth: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Amount")]
    amount: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Price")]
    price: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Address")]
    address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "PageNumber")]
    page_num: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "OrderId")]
    orderid: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Username")]
    username: Option<String>,
}

impl Params {
    fn new() -> Params {
        Params {
            market: None,
            count: None,
            currency: None,
            typeo: None,
            depth: None,
            amount: None,
            price: None,
            address: None,
            page_num: None,
            orderid: None,
            username: None,
        }
    }

    fn market(mut self, market: String) -> Self {
        self.market = Some(market);
        self
    }

    fn count(mut self, count: u8) -> Self {
        self.count = Some(count);
        self
    }

    fn currency(mut self, currency: String) -> Self {
        self.currency = Some(currency);
        self
    }

    fn typeo(mut self, typeo: String) -> Self {
        self.typeo = Some(typeo);
        self
    }

    fn depth(mut self, depth: u8) -> Self {
        self.depth = Some(depth);
        self
    }
    fn amount(mut self, amount: f64) -> Self {
        self.amount = Some(amount);
        self
    }
    fn price(mut self, price: f64) -> Self {
        self.price = Some(price);
        self
    }
    fn address(mut self, address: String) -> Self {
        self.address = Some(address);
        self
    }
    fn page_num(mut self, page_num: u8) -> Self {
        self.page_num = Some(page_num);
        self
    }
    fn orderid(mut self, orderid: u64) -> Self {
        self.orderid = Some(orderid);
        self
    }
    fn username(mut self, username: String) -> Self {
        self.username = Some(username);
        self
    }
}

header! { (CT, "Content-Type") => [String] }

fn generate_header(url: &str, params: &Params) -> Headers {
    let url_encoded: String = byte_serialize(&url.as_bytes()).collect();
    let post_params = &to_string(&params).unwrap();
    let randn: f64 = rand::random();
    let nonce = &randn.to_string()[2..];
    let signature: String = format!(
        "{}POST{}{}{}",
        &API_PUBLIC_KEY,
        &url_encoded.to_lowercase(),
        &nonce,
        &encode(&post_params)
    );

    let mut mac = Hmac::<Sha512>::new_varkey(&decode(&API_PRIVATE_KEY.as_bytes()).unwrap())
        .expect("HMAC can take key of any size");
    mac.input(&signature.as_bytes());
    let hmac_sign = encode(&mac.result().code());

    let header: String = format!("Basic {}:{}:{}", API_PUBLIC_KEY, hmac_sign, &nonce);

    let mut headers = Headers::new();
    headers.set(CT("application/json; charset=utf-8".to_string()));
    headers.set(Authorization(header));
    headers.set(UserAgent::new(
        "Mozilla/4.0 (compatible; TradeSatoshi API Rust client)",
    ));
    headers.set(AcceptEncoding(vec![
        qitem(Encoding::Identity),
        qitem(Encoding::Chunked),
        qitem(Encoding::Gzip),
        qitem(Encoding::Deflate),
    ]));
    headers
}

//////////////////////////
// Public API Functions //
//////////////////////////

pub fn get_currencies() -> Result<Vec<Currency>> {
    let mut resp = Query::new("getcurrencies".to_string(), Api::Public)
        .run()
        .unwrap();
    let data: APIResult<Vec<Currency>> = resp.json().unwrap();
    check_vec_response(data)
}

pub fn get_ticker(market: String) -> Result<Ticker> {
    let mut resp = Query::new("getticker".to_string(), Api::Public)
        .params(Params::new().market(market))
        .run()
        .unwrap();
    let data: APIResult<Ticker> = resp.json().unwrap();
    check_single_response(data)
}

pub fn get_market_history(market: String, count: Option<u8>) -> Result<Vec<Trade>> {
    let count: u8 = match count {
        Some(val) => val,
        None => 20,
    };
    let mut resp = Query::new("getmarkethistory".to_string(), Api::Public)
        .params(Params::new().market(market).count(count))
        .run()
        .unwrap();
    let data: APIResult<Vec<Trade>> = resp.json().unwrap();
    check_vec_response(data)
}

pub fn get_market_summary(market: String) -> Result<MarketSummary> {
    let mut resp = Query::new("getmarketsummary".to_string(), Api::Public)
        .params(Params::new().market(market))
        .run()
        .unwrap();
    let data: APIResult<MarketSummary> = resp.json().unwrap();
    check_single_response(data)
}

pub fn get_market_summaries() -> Result<Vec<MarketSummary>> {
    let mut resp = Query::new("getmarketsummaries".to_string(), Api::Public)
        .run()
        .unwrap();
    let data: APIResult<Vec<MarketSummary>> = resp.json().unwrap();
    check_vec_response(data)
}

pub fn get_order_book(
    market: String,
    typeo: Option<String>,
    depth: Option<u8>,
) -> Result<PublicOrderBook> {
    let typeo: String = match typeo {
        Some(val) => val,
        None => "both".to_string(),
    };
    let depth: u8 = match depth {
        Some(val) => val,
        None => 20,
    };
    let mut resp = Query::new("getorderbook".to_string(), Api::Public)
        .params(Params::new().market(market).typeo(typeo).depth(depth))
        .run()
        .unwrap();
    let data: APIResult<PublicOrderBook> = resp.json().unwrap();
    check_single_response(data)
}

///////////////////////////
// Private API Functions //
///////////////////////////

pub fn get_balance(currency: String) -> Result<Balance> {
    let mut resp = Query::new("getbalance".to_string(), Api::Private)
        .params(Params::new().currency(currency))
        .run()
        .unwrap();
    let data: APIResult<Balance> = resp.json().unwrap();
    check_single_response(data)
}

pub fn get_balances() -> Result<Vec<Balance>> {
    let mut resp = Query::new("getbalances".to_string(), Api::Private)
        .params(Params::new())
        .run()
        .unwrap();
    let data: APIResult<Vec<Balance>> = resp.json().unwrap();
    check_vec_response(data)
}

pub fn get_order(orderid: u64) -> Result<Order> {
    let mut resp = Query::new("getorder".to_string(), Api::Private)
        .params(Params::new().orderid(orderid))
        .run()
        .unwrap();
    let data: APIResult<Order> = resp.json().unwrap();
    check_single_response(data)
}

pub fn get_orders(market: Option<String>, count: Option<u8>) -> Result<Vec<Order>> {
    let market: String = match market {
        Some(val) => val,
        None => "all".to_string(),
    };
    let count: u8 = match count {
        Some(val) => val,
        None => 20,
    };
    let mut resp = Query::new("getorders".to_string(), Api::Private)
        .params(Params::new().market(market).count(count))
        .run()
        .unwrap();
    let data: APIResult<Vec<Order>> = resp.json().unwrap();
    check_vec_response(data)
}

pub fn submit_order(market: String, typeo: String, amount: f64, price: f64) -> Result<SubmitOrder> {
    let mut resp = Query::new("submitorder".to_string(), Api::Private)
        .params(
            Params::new()
                .market(market)
                .typeo(typeo)
                .amount(amount)
                .price(price),
        )
        .run()
        .unwrap();
    let data: APIResult<SubmitOrder> = resp.json().unwrap();
    check_single_response(data)
}

pub fn cancel_order(typeo: String, orderid: u64, market: String) -> Result<CancelOrder> {
    let mut resp = Query::new("cancelorder".to_string(), Api::Private)
        .params(Params::new().market(market).typeo(typeo).orderid(orderid))
        .run()
        .unwrap();
    let data: APIResult<CancelOrder> = resp.json().unwrap();
    check_single_response(data)
}

pub fn get_trade_history(market: String, count: u8, page_num: u8) -> Result<Vec<Trade>> {
    let mut resp = Query::new("gettradehistory".to_string(), Api::Private)
        .params(Params::new().market(market).count(count).page_num(page_num))
        .run()
        .unwrap();
    let data: APIResult<Vec<Trade>> = resp.json().unwrap();
    check_vec_response(data)
}

pub fn generate_address(currency: String) -> Result<Address> {
    let mut resp = Query::new("generateaddress".to_string(), Api::Private)
        .params(Params::new().currency(currency))
        .run()
        .unwrap();
    let data: APIResult<Address> = resp.json().unwrap();
    check_single_response(data)
}

pub fn submit_withdraw(currency: String, address: String, amount: f64) -> Result<Id> {
    let mut resp = Query::new("gettradehistory".to_string(), Api::Private)
        .params(
            Params::new()
                .currency(currency)
                .address(address)
                .amount(amount),
        )
        .run()
        .unwrap();
    let data: APIResult<Id> = resp.json().unwrap();
    check_single_response(data)
}

pub fn get_deposits(currency: Option<String>, count: Option<u8>) -> Result<Vec<Transaction>> {
    let currency: String = match currency {
        Some(val) => val,
        None => "all".to_string(),
    };
    let count: u8 = match count {
        Some(val) => val,
        None => 20,
    };
    let mut resp = Query::new("gettradehistory".to_string(), Api::Private)
        .params(Params::new().currency(currency).count(count))
        .run()
        .unwrap();
    let data: APIResult<Vec<Transaction>> = resp.json().unwrap();
    check_vec_response(data)
}

pub fn get_withdrawals(currency: Option<String>, count: Option<u8>) -> Result<Vec<Transaction>> {
    let currency: String = match currency {
        Some(val) => val,
        None => "all".to_string(),
    };
    let count: u8 = match count {
        Some(val) => val,
        None => 20,
    };
    let mut resp = Query::new("gettradehistory".to_string(), Api::Private)
        .params(Params::new().currency(currency).count(count))
        .run()
        .unwrap();
    let data: APIResult<Vec<Transaction>> = resp.json().unwrap();
    check_vec_response(data)
}

pub fn submit_transfer(currency: String, username: String, amount: f64) -> Result<SubmitTransfer> {
    let mut resp = Query::new("gettradehistory".to_string(), Api::Private)
        .params(
            Params::new()
                .currency(currency)
                .username(username)
                .amount(amount),
        )
        .run()
        .unwrap();
    let data: APIResult<SubmitTransfer> = resp.json().unwrap();
    check_single_response(data)
}

fn check_single_response<T>(api_result: APIResult<T>) -> Result<T> {
    if api_result.success {
        return Ok(api_result.result.expect("Result should exist!"));
    }
    Err("An error occured!")
}

fn check_vec_response<T>(api_result: APIVecResult<T>) -> Result<Vec<T>> {
    if api_result.success {
        return Ok(api_result.result.expect("Result should exist!"));
    }
    Err("An error occured!")
}
