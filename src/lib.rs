extern crate reqwest;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate base64;
extern crate hmac;
#[macro_use]
extern crate query_params;
extern crate sha2;
extern crate strum;
extern crate url;
#[macro_use]
extern crate strum_macros;
extern crate rand;
pub mod error;
pub mod query;
pub mod values;

#[cfg(test)]
mod tests {
    // use super::*;

    //////////////////////
    // Public API tests //
    //////////////////////

    // #[test]
    // fn test_get_currencies() {
    //     let response = query::get_currencies().unwrap();
    //     println!("{:?}", response);
    // }

    // #[test]
    // fn test_get_ticker() {
    //     let response = query::get_ticker("LTC_BTC".to_string()).unwrap();
    //     println!("{}", response.ask);
    // }

    // #[test]
    // fn test_get_market_history() {
    //     let response = query::get_market_history("LTC_BTC".to_string(), None).unwrap();
    //     println!("{:?}", response);
    // }

    // #[test]
    // fn test_get_market_summary() {
    //     let response = query::get_market_summary("LTC_BTC".to_string()).unwrap();
    //     println!("{:?}", response);
    // }

    // #[test]
    // fn test_get_market_summaries() {
    //     let response = query::get_market_summaries().unwrap();
    //     println!("{:?}", response);
    // }

    // #[test]
    // fn test_get_order_book() {
    //     let response =
    //         query::get_order_book("LTC_BTC".to_string(), Some("both".to_string()), Some(10));
    //     println!("{:?}", response);
    // }

    ///////////////////////
    // Private API tests //
    ///////////////////////

    // #[test]
    // fn test_get_balance() {
    //     let balance = query::get_balance("PPC".to_string()).unwrap();
    //     println!("{}", balance.currency);
    // }

    // #[test]
    // fn test_get_balances() {
    //     let response = query::get_balances().unwrap();
    //     println!("{:?}", response);
    // }

    // #[test]
    // fn test_get_orders() {
    //     let response = query::get_orders(None, None).unwrap();
    //     println!("{:?}", response);
    // }

    // #[test]
    // fn test_generate_address() {
    //     let response = query::generate_address("PPC".to_string()).unwrap();
    //     println!("{}", response.address);
    // }

    // #[test]
    // fn test_submit_withdraw() {
    //     let response =
    //         query::submit_withdraw("PPC".to_string(), "123456".to_string(), 1222223.12323)
    //             .unwrap();
    //     println!("{:?}", response);
    // }
}
