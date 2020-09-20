use simplelog::{ConfigBuilder, LevelFilter, SimpleLogger};
use std::error::Error;

mod closure;
mod future;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let config = ConfigBuilder::new()
        .set_target_level(LevelFilter::Trace)
        .build();
    let _ = SimpleLogger::init(LevelFilter::Debug, config);
    // 闭包的学习
    closure::run();

    //  Futures学习
    future::run().await;
    Ok(())
}
