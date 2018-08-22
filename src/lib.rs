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
    use super::*;

    //////////////////////
    // Public API tests //
    //////////////////////

    // #[test]
    // fn test_get_currencies() {}

    // #[test]
    // fn test_get_ticker() {}

    // #[test]
    // fn test_get_market_history() {}

    // #[test]
    // fn test_get_market_summary() {}

    // #[test]
    // fn test_get_market_summaries() {}

    // #[test]
    // fn test_get_order_book() {}

    ///////////////////////
    // Private API tests //
    ///////////////////////

    // #[test]
    // fn test_get_balance() {}

    // #[test]
    // fn test_get_balances() {}

    // #[test]
    // fn test_get_orders() {}

    // #[test]
    // fn test_generate_address() {}

    // #[test]
    // fn test_submit_withdraw() {}
}
