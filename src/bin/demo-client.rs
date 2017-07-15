extern crate network_math;

use network_math::NetMath;

mod demo;

fn main() {
    NetMath::set_url("localhost:4242");
    let fib = demo::Fib::new();
    for i in fib.take(7) {
        println!("{} {}", i, demo::is_prime(i));
    }
}
