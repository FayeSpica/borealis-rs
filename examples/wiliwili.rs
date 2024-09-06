extern crate env_logger;

use borealis_rs::core::application;
use log::info;
use log::LevelFilter::Info;

fn main() -> anyhow::Result<()> {
    env_logger::Builder::from_default_env()
        .filter_level(Info)
        .target(env_logger::Target::Stdout) // stdout
        .init();
    let application = application::Application::init()?;

    application.create_window("wiliwili");
    info!("create_window done");

    while application.main_loop() {}

    info!("main_loop done");

    Ok(())
}
