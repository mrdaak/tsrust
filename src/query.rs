use base64::{decode, encode};
use hmac::{Hmac, Mac};
use rand;
use reqwest;
use reqwest::header::{Authorization, ContentType, Headers, UserAgent};
use serde_json::to_string;
use sha2::Sha512;
use strum::AsStaticRef;
use url::form_urlencoded::byte_serialize;

use std;
use std::str;

use error::{Error, ErrorType};
use values::*;

const API_URL: &str = "https://tradesatoshi.com/api/";

/// Api type
#[derive(AsStaticStr)]
enum Api {
    Public,
    Private,
}

pub type Result<T> = std::result::Result<T, Error>;
pub type RunResult<T> = std::result::Result<T, reqwest::Error>;

pub struct Client {
    api_url: String,
    api_key: String,
    api_secret: String,
}

impl Client {
    pub fn new(api_key: String, api_secret: String) -> Self {
        Client {
            api_url: API_URL.to_string(),
            api_key: api_key.to_string(),
            api_secret: api_secret.to_string(),
        }
    }

    fn run(&self, query: Query) -> RunResult<reqwest::Response> {
        let mut url: String = format!(
            "{}{}/{}",
            self.api_url,
            query.kind.as_static().to_lowercase(),
            query.endpoint
        ).to_owned();
        match query.kind {
            Api::Public => {
                match query.params {
                    Some(params) => url.push_str(&params.to_query_params()),
                    None => (),
                }
                reqwest::get(&url)
            }
            Api::Private => {
                let headers: Headers = self.generate_header(&query.params.as_ref().unwrap());
                let client = reqwest::Client::new();
                let response = client
                    .post(&url)
                    .json(&query.params.unwrap())
                    .headers(headers)
                    .send();
                response
            }
        }
    }

    fn generate_header(&self, params: &Params) -> Headers {
        let url_encoded: String = byte_serialize(&self.api_url.as_bytes()).collect();
        let post_params = &to_string(&params).unwrap();
        let randn: f64 = rand::random();
        let nonce = &randn.to_string()[2..];
        let signature: String = format!(
            "{}POST{}{}{}",
            &self.api_secret,
            &url_encoded.to_lowercase(),
            &nonce,
            &encode(&post_params)
        );

        let mut mac = Hmac::<Sha512>::new_varkey(&decode(&self.api_secret.as_bytes()).unwrap())
            .expect("HMAC can take key of any size");
        mac.input(&signature.as_bytes());
        let hmac_sign = encode(&mac.result().code());

        let header: String = format!("Basic {}:{}:{}", self.api_key, hmac_sign, &nonce);

        let mut headers = Headers::new();
        headers.set(ContentType::json());
        headers.set(Authorization(header));
        headers.set(UserAgent::new(
            "Mozilla/4.0 (compatible; TradeSatoshi API Rust client)",
        ));
        headers
    }

    fn check_single_response<T>(&self, api_result: APIResult<T>) -> Result<T> {
        if api_result.success {
            return Ok(api_result.result.expect("Result should exist!"));
        }
        match api_result.message {
            Some(msg) => Err(Error {
                error_type: ErrorType::APIError,
                message: msg,
            }),
            None => Err(Error {
                error_type: ErrorType::APIError,
                message: "An error occured.".to_string(),
            }),
        }
    }

    fn check_vec_response<T>(&self, api_result: APIVecResult<T>) -> Result<Vec<T>> {
        if api_result.success {
            return Ok(api_result.result.expect("Result should exist!"));
        }
        match api_result.message {
            Some(msg) => Err(Error {
                error_type: ErrorType::APIError,
                message: msg,
            }),
            None => Err(Error {
                error_type: ErrorType::APIError,
                message: "An error occured.".to_string(),
            }),
        }
    }

    //////////////////////////
    // Public API Functions //
    //////////////////////////

    /// Get currencies
    pub fn get_currencies(&self) -> Result<Vec<Currency>> {
        let mut resp = self.run(Query::new("getcurrencies".to_string(), Api::Public))
            .unwrap();
        let data: APIVecResult<Currency> = resp.json().unwrap();
        self.check_vec_response(data)
    }

    /// Get ticker
    ///
    /// market: The market name e.g. 'LTC_BTC' (required)
    pub fn get_ticker(&self, market: String) -> Result<Ticker> {
        let mut resp = self.run(
            Query::new("getticker".to_string(), Api::Public).params(Params::new().market(market)),
        ).unwrap();
        let data: APIResult<Ticker> = resp.json().unwrap();
        self.check_single_response(data)
    }

    /// Get market history
    ///
    /// market: The market name e.g. 'LTC_BTC' (required)
    /// count: The max amount of records to return (optional, default: 20)
    pub fn get_market_history(&self, market: String, count: Option<u32>) -> Result<Vec<Trade>> {
        let count: u32 = match count {
            Some(val) => val,
            None => 20,
        };
        let mut resp = self.run(
            Query::new("getmarkethistory".to_string(), Api::Public)
                .params(Params::new().market(market).count(count)),
        ).unwrap();
        let data: APIVecResult<Trade> = resp.json().unwrap();
        self.check_vec_response(data)
    }

    /// Get market summary
    ///
    /// market: The market name e.g. 'LTC_BTC' (required)
    pub fn get_market_summary(&self, market: String) -> Result<MarketSummary> {
        let mut resp = self.run(
            Query::new("getmarketsummary".to_string(), Api::Public)
                .params(Params::new().market(market)),
        ).unwrap();
        let data: APIResult<MarketSummary> = resp.json().unwrap();
        self.check_single_response(data)
    }

    /// Get market summaries
    pub fn get_market_summaries(&self) -> Result<Vec<MarketSummary>> {
        let mut resp = self.run(Query::new("getmarketsummaries".to_string(), Api::Public))
            .unwrap();
        let data: APIVecResult<MarketSummary> = resp.json().unwrap();
        self.check_vec_response(data)
    }

    /// Get order book
    ///
    /// market: The market name e.g. 'LTC_BTC' (required)
    /// type: The order book type 'buy', 'sell', 'both' (optional, default: 'both')
    /// depth: Max of records to return (optional, default: 20)
    pub fn get_order_book(
        &self,
        market: String,
        typeo: Option<String>,
        depth: Option<u32>,
    ) -> Result<PublicOrderBook> {
        let typeo: String = match typeo {
            Some(val) => val,
            None => "both".to_string(),
        };
        let depth: u32 = match depth {
            Some(val) => val,
            None => 20,
        };
        let mut resp = self.run(
            Query::new("getorderbook".to_string(), Api::Public)
                .params(Params::new().market(market).typeo(typeo).depth(depth)),
        ).unwrap();
        let data: APIResult<PublicOrderBook> = resp.json().unwrap();
        self.check_single_response(data)
    }

    ///////////////////////////
    // Private API Functions //
    ///////////////////////////

    /// Get balance
    ///
    /// currency: The currency of the balance to return e.g. 'BTC' (required)
    pub fn get_balance(&self, currency: String) -> Result<Balance> {
        let mut resp = self.run(
            Query::new("getbalance".to_string(), Api::Private)
                .params(Params::new().currency(currency)),
        ).unwrap();
        let data: APIResult<Balance> = resp.json().unwrap();
        self.check_single_response(data)
    }

    /// Get balances
    pub fn get_balances(&self) -> Result<Vec<Balance>> {
        let mut resp = self.run(
            Query::new("getbalances".to_string(), Api::Private).params(Params::new()),
        ).unwrap();
        let data: APIVecResult<Balance> = resp.json().unwrap();
        self.check_vec_response(data)
    }

    /// Get order
    ///
    /// orderid: The order to return (required)
    pub fn get_order(&self, orderid: u32) -> Result<Order> {
        let mut resp = self.run(
            Query::new("getorder".to_string(), Api::Private).params(Params::new().orderid(orderid)),
        ).unwrap();
        let data: APIResult<Order> = resp.json().unwrap();
        self.check_single_response(data)
    }

    /// Get orders
    ///
    /// market: The market name e.g. 'LTC_BTC' (optional, default: 'all')
    /// count: The maximum count of records to return (optional, default: 20)
    pub fn get_orders(&self, market: Option<String>, count: Option<u32>) -> Result<Vec<Order>> {
        let market: String = match market {
            Some(val) => val,
            None => "all".to_string(),
        };
        let count: u32 = match count {
            Some(val) => val,
            None => 20,
        };
        let mut resp = self.run(
            Query::new("getorders".to_string(), Api::Private)
                .params(Params::new().market(market).count(count)),
        ).unwrap();
        let data: APIVecResult<Order> = resp.json().unwrap();
        self.check_vec_response(data)
    }

    /// Submit order
    ///
    /// market: The market name e.g. 'LTC_BTC' (required)
    /// type: The order type name e.g. 'Buy', 'Sell' (required)
    /// amount: The amount to buy/sell (required)
    /// price: The price to buy/sell for (required)
    pub fn submit_order(
        &self,
        market: String,
        typeo: String,
        amount: f32,
        price: f32,
    ) -> Result<SubmitOrder> {
        let mut resp = self.run(
            Query::new("submitorder".to_string(), Api::Private).params(
                Params::new()
                    .market(market)
                    .typeo(typeo)
                    .amount(amount)
                    .price(price),
            ),
        ).unwrap();
        let data: APIResult<SubmitOrder> = resp.json().unwrap();
        self.check_single_response(data)
    }

    /// Cancel order
    ///
    /// type: The cancel type, options: 'Single','Market','MarketBuys','MarketSells','AllBuys','AllSells','All'(required)
    /// orderId: The order to cancel(required if cancel type 'Single')
    /// market: The order to cancel(required if cancel type 'Market','MarketBuys','MarketSells')
    pub fn cancel_order(
        &self,
        typeo: String,
        orderid: Option<u32>,
        market: Option<String>,
    ) -> Result<CancelOrder> {
        let mut params: Params = Params::new().typeo(typeo);
        params = match market {
            Some(market) => params.market(market),
            None => params,
        };
        params = match orderid {
            Some(orderid) => params.orderid(orderid),
            None => params,
        };

        let mut resp = self.run(Query::new("cancelorder".to_string(), Api::Private).params(params))
            .unwrap();
        let data: APIResult<CancelOrder> = resp.json().unwrap();
        self.check_single_response(data)
    }

    /// Get trade history
    ///
    /// market: The market name e.g. 'LTC_BTC' (optional, default: 'all')
    /// count: The maximum count of records to return (optional, default: 20)
    /// page_num: The Pagenumber for maintain pagination (optional, default: 0)
    pub fn get_trade_history(
        &self,
        market: Option<String>,
        count: Option<u32>,
        page_num: Option<u32>,
    ) -> Result<Vec<TradeHistory>> {
        let market: String = match market {
            Some(val) => val,
            None => "all".to_string(),
        };
        let count: u32 = match count {
            Some(val) => val,
            None => 20,
        };
        let page_num: u32 = match page_num {
            Some(val) => val,
            None => 0,
        };
        let mut resp = self.run(
            Query::new("gettradehistory".to_string(), Api::Private)
                .params(Params::new().market(market).count(count).page_num(page_num)),
        ).unwrap();
        let data: APIVecResult<TradeHistory> = resp.json().unwrap();
        self.check_vec_response(data)
    }

    /// Generate address
    ///
    /// currency: The currency to generate address for e.g. 'BTC' (required)
    pub fn generate_address(&self, currency: String) -> Result<Address> {
        let mut resp = self.run(
            Query::new("generateaddress".to_string(), Api::Private)
                .params(Params::new().currency(currency)),
        ).unwrap();
        let data: APIResult<Address> = resp.json().unwrap();
        self.check_single_response(data)
    }

    /// Submit withdraw
    ///
    /// currency: The currency name e.g. 'BTC' (required)
    /// address: The receiving address (required)
    /// amount: The amount to withdraw (required)
    pub fn submit_withdraw(&self, currency: String, address: String, amount: f32) -> Result<Id> {
        let mut resp = self.run(
            Query::new("gettradehistory".to_string(), Api::Private).params(
                Params::new()
                    .currency(currency)
                    .address(address)
                    .amount(amount),
            ),
        ).unwrap();
        let data: APIResult<Id> = resp.json().unwrap();
        self.check_single_response(data)
    }

    /// Get deposits
    ///
    /// currency: The currency name e.g. 'BTC' (optional, default: 'all')
    /// count: The maximum count of records to return (optional, default: 20)
    pub fn get_deposits(
        &self,
        currency: Option<String>,
        count: Option<u32>,
    ) -> Result<Vec<Transaction>> {
        let currency: String = match currency {
            Some(val) => val,
            None => "all".to_string(),
        };
        let count: u32 = match count {
            Some(val) => val,
            None => 20,
        };
        let mut resp = self.run(
            Query::new("gettradehistory".to_string(), Api::Private)
                .params(Params::new().currency(currency).count(count)),
        ).unwrap();
        let data: APIVecResult<Transaction> = resp.json().unwrap();
        self.check_vec_response(data)
    }

    /// Get withdrawals
    ///
    /// currency: The currency name e.g. 'BTC' (optional, default: 'all')
    /// count: The maximum count of records to return (optional, default: 20)
    pub fn get_withdrawals(
        &self,
        currency: Option<String>,
        count: Option<u32>,
    ) -> Result<Vec<Transaction>> {
        let currency: String = match currency {
            Some(val) => val,
            None => "all".to_string(),
        };
        let count: u32 = match count {
            Some(val) => val,
            None => 20,
        };
        let mut resp = self.run(
            Query::new("gettradehistory".to_string(), Api::Private)
                .params(Params::new().currency(currency).count(count)),
        ).unwrap();
        let data: APIVecResult<Transaction> = resp.json().unwrap();
        self.check_vec_response(data)
    }

    /// Submit transfer
    ///
    /// currency: The currency name e.g. 'BTC' (required)
    /// username: The TradeSatoshi username of the person to transfer the funds to. (required)
    /// amount: The amount of coin to transfer e.g. 251.00000000 (required)
    pub fn submit_transfer(
        &self,
        currency: String,
        username: String,
        amount: f32,
    ) -> Result<SubmitTransfer> {
        let mut resp = self.run(
            Query::new("gettradehistory".to_string(), Api::Private).params(
                Params::new()
                    .currency(currency)
                    .username(username)
                    .amount(amount),
            ),
        ).unwrap();
        let data: APIResult<SubmitTransfer> = resp.json().unwrap();
        self.check_single_response(data)
    }
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
}

/// Query parameters
#[derive(QueryParams, Serialize, Debug)]
struct Params {
    #[serde(skip_serializing_if = "Option::is_none", rename = "Market")]
    market: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Count")]
    count: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Currency")]
    currency: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Type")]
    typeo: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Depth")]
    depth: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Amount")]
    amount: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Price")]
    price: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Address")]
    address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "PageNumber")]
    page_num: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "OrderId")]
    orderid: Option<u32>,
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

    fn count(mut self, count: u32) -> Self {
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

    fn depth(mut self, depth: u32) -> Self {
        self.depth = Some(depth);
        self
    }
    fn amount(mut self, amount: f32) -> Self {
        self.amount = Some(amount);
        self
    }
    fn price(mut self, price: f32) -> Self {
        self.price = Some(price);
        self
    }
    fn address(mut self, address: String) -> Self {
        self.address = Some(address);
        self
    }
    fn page_num(mut self, page_num: u32) -> Self {
        self.page_num = Some(page_num);
        self
    }
    fn orderid(mut self, orderid: u32) -> Self {
        self.orderid = Some(orderid);
        self
    }
    fn username(mut self, username: String) -> Self {
        self.username = Some(username);
        self
    }
}
