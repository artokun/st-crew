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
cargo watch -i sdk -i src/generated/ -cx run
```

#### Node
```sh
cd sdk
npm i
npm run dev
``````

### System Specific setup

#### Mac
```sh
brew install zlib
brew install libuv
brew install openssl

```

Also add these to your `.zshrc` or `.bashrc`:
```sh
export OPENSSL_INCLUDE_DIR=$(brew --prefix openssl)/include
export DEP_OPENSSL_INCLUDE=$(brew --prefix openssl)/include
export OPENSSL_LIB_DIR=$(brew --prefix openssl)/lib
export LIBRARY_PATH=/opt/homebrew/lib:$LIBRARY_PATH
export CPATH=/opt/homebrew/include:$CPATH
```

#### Linux
```sh
sudo apt-get update
sudo apt-get install zlib1g-dev
sudo apt-get install libuv1-dev
sudo apt-get install libssl-dev
sudo apt-get install libcrypto++-dev
```