#![cfg(target_os = "android")]

#[ndk_glue::main(backtrace = "on")]
fn main() {
    borealis_rs::core::main()
}
