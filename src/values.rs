#[derive(Serialize, Deserialize)]
pub struct APIResult<T> {
    pub success: bool,
    pub message: Option<String>,
    pub result: Option<T>,
}

#[derive(Serialize, Deserialize)]
pub struct APIVecResult<T> {
    pub success: bool,
    pub message: Option<String>,
    pub result: Option<Vec<T>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Currency {
    pub currency: String,
    #[serde(rename = "currencyLong")]
    pub currency_long: String,
    #[serde(rename = "minConfirmation")]
    pub min_confirmation: u32,
    #[serde(rename = "txFee")]
    pub tx_fee: f32,
    pub status: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Address {
    pub currency: String,
    pub address: String,
}

#[derive(Deserialize, Debug)]
pub struct Balance {
    pub currency: String,
    #[serde(rename = "currencyLong")]
    pub currency_long: String,
    pub available: f32,
    pub total: f32,
    #[serde(rename = "heldForTrades")]
    pub held_for_trades: f32,
    pub unconfirmed: f32,
    #[serde(rename = "pendingWithdraw")]
    pub pending_withdraw: f32,
    pub address: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct Order {
    pub id: bool,
    pub market: String,
    // type is a reserved keyword
    #[serde(rename = "type")]
    pub order_type: String,
    pub amount: f32,
    pub rate: f32,
    pub remaining: f32,
    pub total: f32,
    pub status: String,
    pub timestamp: String,
    #[serde(rename = "isApi")]
    pub is_api: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MarketSummary {
    pub market: String,
    pub high: f32,
    pub low: f32,
    pub volume: f32,
    pub last: f32,
    #[serde(rename = "baseVolume")]
    pub base_volume: f32,
    pub bid: f32,
    pub ask: f32,
    #[serde(rename = "openBuyOrders")]
    pub open_buy_orders: u32,
    #[serde(rename = "openSellOrders")]
    pub open_sell_orders: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Ticker {
    pub ask: f32,
    pub bid: f32,
    pub last: f32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PublicOrderBook {
    pub buy: Vec<PublicOrder>,
    pub sell: Vec<PublicOrder>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PublicOrder {
    pub quantity: f32,
    pub rate: f32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Trade {
    pub id: u32,
    #[serde(rename = "timeStamp")]
    pub time_stamp: String,
    pub quantity: f32,
    pub price: f32,
    pub total: f32,
    #[serde(rename = "orderType")]
    pub order_type: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TradeHistory {
    pub id: u32,
    pub market: String,
    #[serde(rename = "type")]
    pub typeo: String,
    pub amount: f32,
    pub rate: f32,
    pub fee: f32,
    pub total: f32,
    #[serde(rename = "timeStamp")]
    pub time_stamp: String,
    #[serde(rename = "isApi")]
    pub is_api: bool,
}

#[derive(Serialize, Deserialize)]
pub struct Transaction {
    pub id: String,
    pub currency: String,
    #[serde(rename = "currencyLong")]
    pub currency_long: String,
    pub amount: f32,
    pub fee: f32,
    pub address: String,
    pub status: String,
    #[serde(rename = "txId")]
    pub tx_id: Option<String>,
    pub confirmations: u32,
    #[serde(rename = "timeStamp")]
    pub time_stamp: String,
    #[serde(rename = "isApi")]
    pub is_api: bool,
}

#[derive(Serialize, Deserialize)]
pub struct Id {
    #[serde(rename = "withdrawalId")]
    pub withdrawal_id: String,
}

#[derive(Serialize, Deserialize)]
pub struct SubmitOrder {
    #[serde(rename = "orderId")]
    pub order_id: u32,
    pub filled: Vec<u32>,
}

#[derive(Serialize, Deserialize)]
pub struct CancelOrder {
    #[serde(rename = "canceledOrders")]
    pub canceled_orders: Vec<u32>,
}

#[derive(Serialize, Deserialize)]
pub struct SubmitTransfer {
    pub data: String,
}
