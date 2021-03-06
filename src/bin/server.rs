#[macro_use]
extern crate log;
extern crate env_logger;
extern crate network_math;

use network_math::NetMath;

fn main() {
    env_logger::LogBuilder::new()
        .filter(None, log::LogLevelFilter::Info)
        .init()
        .unwrap();

    let url = "localhost:4242";
    let _server = NetMath::start_server(url).unwrap();
    info!("Running on {}", url);
}
