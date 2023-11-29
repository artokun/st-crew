# SpaceTraders Real-time Crew Management Game

## Development

### Setup

> First time setup, you need to generate the flatbuffers from the schema files.
```sh
sh ./generate.sh
```

#### Rust
```sh
cargo install cargo-watch
cargo watch -w src -cx run
```

#### Node
```sh
cd sdk
npm i
npm run dev
``````