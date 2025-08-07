//! This is the build script for both tests7 and tests8.

fn main() {
    // 1. 为 tests7 设置环境变量 `TEST_FOO`
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    // 将时间戳作为环境变量值传递给 Cargo
    println!("cargo:rustc-env=TEST_FOO={}", timestamp); // [1,9](@ref)

    // 2. 为 tests8 启用 "pass" 特性
    // 通知 Cargo 启用特性 `pass`
    println!("cargo:rustc-cfg=feature=\"pass\""); // [4](@ref)
}