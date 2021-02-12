# Russell

A matrix.org bot in Rust. Uses the [Matrix Rust SDK](https://github.com/matrix-org/matrix-rust-sdk).

## Running

1. Clone repo, build with `cargo build`.
1. Run with `cargo run`. On the first run, if a bot_config.toml file is not present, a default config
   file will be generated.
1. Fill in the bot_config.toml with a username.
1. Run with `cargo run`. You'll be prompted for a password, and then an authentication token will be
   generated for future runs and saved to the bot_config.toml file. The password is not stored.

## Design

Based heavily on the [examples in the Matrix Rust SDK repository](https://github.com/matrix-org/matrix-rust-sdk/tree/master/matrix_sdk/examples).

- The `Bot` struct has a reference to the Matrix API client.
- Plugins implement the `Plugin` trait and are located in `src/plugins`. Plugins are registered with
  the bot in `plugins.rs`.
- The matrix-sdk defines an `EventEmitter` trait, which allows implementers to receive callbacks for
  matrix events. However, the matrix-sdk only allows a single instance to be registered and receive
  events. The `EventForwarder` struct (in `src/plugins.rs`) is that instance and forwards events to
  all the registered plugins.