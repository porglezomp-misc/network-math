extern crate network_math;

use network_math::NetMath;

mod demo;

fn main() {
    let mut server = NetMath::start_server("localhost:6666").unwrap();
    let fib = demo::Fib::new();
    for i in fib.take(7) {
        println!("{} {}", i, demo::is_prime(i));
    }
    server.close().unwrap();
}
