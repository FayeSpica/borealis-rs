extern crate env_logger;

use log::LevelFilter::Info;

fn main() {
    // 初始化 env_logger
    env_logger::Builder::from_default_env()
        .filter_level(Info)
        .target(env_logger::Target::Stdout) // 将日志输出到 stdout
        .init();
    borealis_rs::core::main()
}
