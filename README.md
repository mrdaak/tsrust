# tsrust
tradesatoshi.com api wrapper in rust

## Example

```
extern crate tradesatoshi_api
use tradesatoshi_api::Client

let client = Client::new("public key".to_string(), "private key".to_string());
let orders = client.get_orders().unwrap();
```

