# Rust SDK for [Phoenixd](https://phoenix.acinq.co/server)


## Status

### Receive
- [x] Create invoice
- [x] Get Invoice
- [x] Get Incoming Invoice

### Pay
- [x] Get LN payment quote
- [x] Execute LN Payment Quote

### Webhook

## Minimum Supported Rust Version (MSRV)

The `phoenixd` library should always compile with any combination of features on Rust **1.63.0**.

To build and test with the MSRV you will need to pin the below dependency versions:

```shell
cargo update -p tokio --precise 1.38.1
cargo update -p reqwest --precise 0.12.4
```
