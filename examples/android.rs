#![cfg(target_os = "android")]
extern crate android_log;
#[macro_use]
extern crate log;
#[ndk_glue::main(backtrace = "on")]
fn main() {
    android_log::init("borealis").unwrap();

    trace!("Initialized Rust");
    debug!("Address is {:p}", main as *const ());
    info!("Did you know? {} = {}", "1 + 1", 2);
    warn!("Don't log sensitive information!");
    error!("Nothing more to say");
    borealis_rs::core::main()
}
