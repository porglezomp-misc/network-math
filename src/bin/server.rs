extern crate network_math;

use network_math::NetMath;

fn main() {
    let url = "localhost:4242";
    let _server = NetMath::start_server(url).unwrap();
    println!("Running on {}", url);
}
