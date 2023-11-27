fn main() {
    println!("cargo:rustc-link-lib=z");
    println!("cargo:rustc-link-lib=uv");
    println!("cargo:rustc-link-lib=ssl");
    println!("cargo:rustc-link-lib=crypto");

    // Conditional linking for C++ standard library based on the target OS
    #[cfg(target_os = "macos")]
    println!("cargo:rustc-link-lib=c++"); // Use libc++ for macOS
    #[cfg(not(target_os = "macos"))]
    println!("cargo:rustc-link-lib=stdc++"); // Use libstdc++ for other systems
}
