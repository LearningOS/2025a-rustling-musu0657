//! This is the build script for both tests7 and tests8.
//!
//! You should modify this file to make both exercises pass.

fn main() {
    // In tests7, we should set up an environment variable
    // called `TEST_FOO`. Print in the standard output to let
    // Cargo do it.
    // The command to set up an environment variable is "rustc-env=VAR=VALUE".
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs(); // What's the use of this timestamp here?
    println!("cargo:rustc-env=TEST_FOO={}", timestamp);

    // In tests8, we should enable "pass" feature to make the
    // testcase return early. Fill in the command to tell
    // Cargo about that.
    // The command to set up an environment variable is "rustc-cfg=CFG[=\"VALUE\"]", while
    // the square brackets means optional. Be sure what `CFG` and `VALUE` you want here.
    // 核心：按提示格式设置rustc-cfg，CFG=feature，VALUE=pass（启用pass特性）
    println!("cargo:rustc-cfg=feature=\"pass\"");
}