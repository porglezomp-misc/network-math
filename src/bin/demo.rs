use network_math::NetMath;

pub struct Fib {
    a: NetMath<u64>,
    b: NetMath<u64>,
}

impl Fib {
    pub fn new() -> Self {
        Fib {
            a: NetMath(0),
            b: NetMath(1),
        }
    }
}

impl Iterator for Fib {
    type Item = NetMath<u64>;
    fn next(&mut self) -> Option<Self::Item> {
        let res = self.a;
        let tmp = self.b;
        self.b = self.a + self.b;
        self.a = tmp;
        Some(res)
    }
}

pub fn is_prime(x: NetMath<u64>) -> bool {
    if x % NetMath(2) == NetMath(0) {
        return false;
    }

    let mut i = NetMath(3);
    loop {
        if i * i > x {
            return true;
        }
        if x % i == NetMath(0) {
            return false;
        }
        i += NetMath(2);
    }
}
