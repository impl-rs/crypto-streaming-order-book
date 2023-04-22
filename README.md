# Crypto streaming order book

This is a Rust project that implements a streaming order book for cryptocurrencies using a gRPC server. It connects to WebSocket streams from the supported exchanges. It then combines the order books from the exchanges and provides a combined order book. From that, it published the spread, the top 10 bids and asks through the gRPC server on each change in either exchange order book.

## Supported exchanges

- Binance
- Bitstamp

## How to run

You can run the gRPC server with the following command:

`cargo run --bin server -- --pair ethbtc`

Where `ethbtc` is the pair of currencies you want to stream an order book for. Currently, there is no validation if this pair exists on both exchanges. So you will have to check that before starting the server.

### Client

There is a client that you can use to test the gRPC server. You can run it with the following command:

`cargo run --bin client`

This will start the client and connect to the gRPC server. It will then print the summary order book to the console.

## Tests

You can run the tests with the following command:

`cargo test -- --test-threads 1`

The reason for only running the tests on 1 thread is that we spin up a test WebSocket server to mock the exchanges responses and connect to it. If we run the tests on multiple threads, the WebSocket server will be started multiple times, and the tests will fail because the address is already in use.

## Improvements

- Validate that the pair exists on both exchanges
- Refactor into cargo workspace to share code between the server and client
