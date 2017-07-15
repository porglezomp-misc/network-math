extern crate network_math;

use network_math::NetMath;

fn main() {
    let mut server = NetMath::start_server("localhost:6666").unwrap();
    let x = 28980;
    let y = 420;
    println!("{}", x / y);
    server.close().unwrap();
}
