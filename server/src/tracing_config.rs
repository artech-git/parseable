


use tracing_subscriber::fmt::{self, format,};
use tracing_subscriber::prelude::*;
use tracing_subscriber::registry::Registry;
use tracing_subscriber::layer::Layered;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{EnvFilter, FmtSubscriber};
use tracing_subscriber::layer::Layer;
use tracing_subscriber::filter::LevelFilter;


pub fn init_tracing_subscriber() -> Result<impl SubscriberInitExt, Box<dyn std::error::Error + Send + 'static>> {

    let crate_filter = EnvFilter::from_default_env()
        .add_directive(format!("{}=trace", env!("CARGO_PKG_NAME")).parse().unwrap());
    
    let console_subs_layer = fmt::Layer::default()
                        // .with_file(true)
                        .with_level(true)
                        .with_line_number(true);
                        // .with_writer(std::io::stdout())

    let exp_layer = MyLayer {};

    let global_subs = tracing_subscriber::registry()
                        .with(console_subs_layer)
                        .with(EnvFilter::from_default_env()
                                    .add_directive(LevelFilter::INFO.into()))
                        .with(exp_layer);
    
    Ok(global_subs)

}
