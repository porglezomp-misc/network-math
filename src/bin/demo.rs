extern crate network_math;

use network_math::NetMath;

fn main() {
    let x = NetMath::new(6);
    let y = NetMath::new(7);
    println!("{}", x * y);
}
