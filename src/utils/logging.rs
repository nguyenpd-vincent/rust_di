use env_logger::Env;
use log::info;

pub fn init_logging() {
    // env_logger::init_from_env(Env::default().default_filter_or("info"));
    info!("Logger initialized");
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();
}