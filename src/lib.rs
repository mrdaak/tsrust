extern crate reqwest;
#[macro_use]
extern crate hyper;
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
    use super::*;

    //////////////////////
    // Public API tests //
    //////////////////////

    // #[test]
    // fn test_get_currencies() {
    //     assert_eq!(query::get_currencies().status(), reqwest::StatusCode::Ok);
    // }
    // #[test]
    // fn test_get_ticker() {
    //     assert_eq!(
    //         query::get_ticker("LTC_BTC".to_string()).status(),
    //         reqwest::StatusCode::Ok
    //     );
    // }
    // #[test]
    // fn test_get_market_history() {
    //     assert_eq!(
    //         query::get_market_history("LTC_BTC".to_string(), None).status(),
    //         reqwest::StatusCode::Ok
    //     );
    // }
    // #[test]
    // fn test_get_market_summary() {
    //     assert_eq!(
    //         query::get_market_summary("LTC_BTC".to_string()).status(),
    //         reqwest::StatusCode::Ok
    //     );
    // }
    // #[test]
    // fn test_get_market_summaries() {
    //     assert_eq!(
    //         query::get_market_summaries().status(),
    //         reqwest::StatusCode::Ok
    //     );
    // }
    // #[test]
    // fn test_get_order_book() {
    //     assert_eq!(
    //         query::get_order_book("LTC_BTC".to_string(), Some("both".to_string()), Some(10))
    //             .status(),
    //         reqwest::StatusCode::Ok
    //     );
    // }

    ///////////////////////
    // Private API tests //
    ///////////////////////

    // #[test]
    // fn test_get_balance() {
    //     let balance: query::Balance = query::get_balance("PPC".to_string());
    //     println!("{:?}", balance);
    //     // assert_eq!(
    //     //     query::get_balance("BTC".to_string()).status(),
    //     //     reqwest::StatusCode::Ok
    //     // );
    // }

    // #[test]
    // fn test_get_balances() {
    //     let response: query::Response = query::get_balances();
    //     println!("{:?}", response);
    // }

    #[test]
    fn test_get_orders() {
        let response: query::OrderResponse = query::get_orders(None, None);
        println!("{:?}", response);
    }

    // #[test]
    // fn test_generate_address() {
    //     let text: String = query::generate_address("SAFEX".to_string()).text().unwrap();
    //     println!("{}", text);

    //     assert_eq!(
    //         query::generate_address("SAFEX".to_string()).status(),
    //         reqwest::StatusCode::Ok
    //     );
    // }

    // #[test]
    // fn test_submit_withdraw() {
    //     let text: String =
    //         query::submit_withdraw("SAFEX".to_string(), "123456".to_string(), 1222223.12323)
    //             .text()
    //             .unwrap();
    //     println!("{}", text);

    //     assert_eq!(
    //         query::submit_withdraw("SAFEX".to_string(), "123456".to_string(), 1222223.12323)
    //             .status(),
    //         reqwest::StatusCode::Ok
    //     );
    // }
}
