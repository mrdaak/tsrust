use std::fmt;

#[derive(Serialize, Deserialize)]
pub struct APIResult<T> {
    pub success: bool,
    pub message: String,
    pub result: Option<T>,
}

#[derive(Serialize, Deserialize)]
pub struct APIVecResult<T> {
    pub success: bool,
    pub message: String,
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
    pub total: f64,
    pub available: f64,
    #[serde(rename = "pendingWithdraw")]
    pub pending: f64,
    pub address: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct Order {
    pub id: bool,
    pub market: String,
    #[serde(rename = "type")]
    pub Type: String,
    pub amount: f64,
    pub rate: f64,
    pub remaining: f64,
    pub total: f64,
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
    pub volume: f64,
    pub last: f64,
    #[serde(rename = "baseVolume")]
    pub base_volume: f64,
    pub bid: f32,
    pub ask: f32,
    #[serde(rename = "openBuyOrders")]
    pub open_buy_orders: u32,
    #[serde(rename = "openSellOrders")]
    pub open_sell_orders: u32,
}

#[derive(Serialize, Deserialize)]
pub struct HistoryOrder {
    #[serde(rename = "OrderUuid")]
    pub order_uuid: String,
    #[serde(rename = "Exchange")]
    pub exchange: String,
    #[serde(rename = "TimeStamp")]
    pub time_stamp: String,
    #[serde(rename = "OrderType")]
    pub order_type: String,
    #[serde(rename = "Quantity")]
    pub quantity: f32,
    #[serde(rename = "QuantityRemaining")]
    pub quantity_remaining: f32,
    #[serde(rename = "Limit")]
    pub limit: f32,
    #[serde(rename = "Commission")]
    pub comission: f32,
    #[serde(rename = "Price")]
    pub price: f32,
    #[serde(rename = "PricePerUnit")]
    pub price_per_unit: Option<f32>,
    #[serde(rename = "ImmediateOrCancel")]
    pub immediate_or_cancel: bool,
    #[serde(rename = "IsConditional")]
    pub is_conditional: bool,
    #[serde(rename = "Condition")]
    pub condition: Option<String>,
    #[serde(rename = "ConditionalTarget")]
    pub conditional_target: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Ticker {
    pub ask: f32,
    pub bid: f32,
    pub last: f32,
}

#[derive(Serialize, Deserialize)]
pub struct PublicOrderBook {
    pub buy: Vec<PublicOrder>,
    pub sell: Vec<PublicOrder>,
}

#[derive(Serialize, Deserialize)]
pub struct PublicOrder {
    pub quantity: f32,
    pub rate: f32,
}

#[derive(Serialize, Deserialize)]
pub struct Trade {
    pub id: u32,
    #[serde(rename = "timeStamp")]
    pub time_stamp: String,
    pub quantity: f32,
    pub price: f32,
    pub total: f32,
    pub order: String,
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
    #[serde(rename = "TxCost")]
    pub tx_cost: f32,
    pub tx_id: Option<String>,
    pub confirmations: u32,
    #[serde(rename = "isApi")]
    pub is_api: bool,
    #[serde(rename = "timeStamp")]
    pub time_stamp: String,
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
