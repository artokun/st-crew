# Bevy uWebSockets Plugin

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

### Usage
Add this to your `build.rs`:
```rs
// Start uWebsocket support
println!("cargo:rustc-link-lib=z");
println!("cargo:rustc-link-lib=uv");
println!("cargo:rustc-link-lib=ssl");
println!("cargo:rustc-link-lib=crypto");

// Conditional linking for C++ standard library based on the target OS
#[cfg(target_os = "macos")]
println!("cargo:rustc-link-lib=c++"); // Use libc++ for macOS
#[cfg(not(target_os = "macos"))]
println!("cargo:rustc-link-lib=stdc++"); // Use libstdc++ for other systems
// End uWebsocket support
```
See the examples folder for usage examples.