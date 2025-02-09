# Message Store Pallet

A simple Substrate pallet that demonstrates basic storage and event handling capabilities. This pallet allows users to store and retrieve messages on-chain.

## Features

- Store messages (up to 1024 bytes)
- Retrieve messages by account ID
- Event emission on message storage
- Bounded storage to prevent spam

## Installation

Add this pallet to your runtime's `Cargo.toml`:

```toml
[dependencies]
pallet-message-store = { git = "https://github.com/habib-wahab/message-store.git" }
```

## Usage

1. Implement the pallet's configuration trait for your runtime:

```rust
impl pallet_message_store::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
}
```

2. Add the pallet to your `construct_runtime!` macro:

```rust
construct_runtime!(
    pub enum Runtime where
        Block = Block,
        NodeBlock = opaque::Block,
        UncheckedExtrinsic = UncheckedExtrinsic
    {
        // -- snip --
        MessageStore: pallet_message_store,
    }
);
```

## Testing

Run the test suite:

```bash
cargo test
```
