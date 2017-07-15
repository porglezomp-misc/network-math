extern crate reqwest;

use std::fmt::Display;

#[derive(Debug)]
pub struct NetMath<T>(pub T);

impl<T> NetMath<T> {
    pub fn new(x: T) -> Self {
        NetMath(x)
    }
}

impl<T: Display> Display for NetMath<T> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(fmt, "{}", self.0)
    }
}

macro_rules! impl_net_operator {
    ($T:ty, $Op:ident, $op:ident) => {
        impl ::std::ops::$Op for NetMath<$T> {
            type Output = NetMath<$T>;
            fn $op(self, rhs: Self) -> Self {
                use std::io::Read;
                let url = format!(
                    "http://localhost:4242/{op}/{type}/{lhs}/{rhs}",
                    op=stringify!($op),
                    type=stringify!($T),
                    lhs=self.0,
                    rhs=rhs.0,
                );
                let mut resp = reqwest::get(&url).expect("Couldn't request operation");
                assert!(resp.status().is_success());
                let mut result = String::new();
                resp.read_to_string(&mut result).expect("Couldn't read response");
                NetMath(result.parse().expect("Couldn't parse response"))
            }
        }
    }
}

macro_rules! impl_net_shift {
    ($T:ty, $Op:ident < $($U:ty),* >, $op:ident) => {
        $(
            impl ::std::ops::$Op <$U> for NetMath<$T> {
                type Output = NetMath<$T>;
                fn $op(self, rhs: $U) -> Self {
                    use std::io::Read;
                    let url = format!(
                        "http://localhost:4242/{op}/{type}/{shift_type}/{lhs}/{rhs}",
                        op=stringify!($op),
                        type=stringify!($T),
                        shift_type=stringify!($U),
                        lhs=self.0,
                        rhs=rhs,
                    );
                    let mut resp = reqwest::get(&url).expect("Couldn't request operation");
                    assert!(resp.status().is_success());
                    let mut result = String::new();
                    resp.read_to_string(&mut result).expect("Couldn't read response");
                    NetMath(result.parse().expect("Couldn't parse response"))
                }
            }
        )*
    }
}

macro_rules! impl_netmath {
    ($($T:ty),*) => {
        $(
            impl_net_operator!($T, Add, add);
            impl_net_operator!($T, Sub, sub);
            impl_net_operator!($T, Mul, mul);
            impl_net_operator!($T, Div, div);
            impl_net_operator!($T, Rem, rem);

            impl PartialEq for NetMath<$T> {
                fn eq(&self, rhs: &Self) -> bool {
                    use std::io::Read;
                    let url = format!(
                        "http://localhost:4242/eq/{type}/{lhs}/{rhs}",
                        type=stringify!($T),
                        lhs=self.0,
                        rhs=rhs.0,
                    );
                    let mut resp = reqwest::get(&url).expect("Couldn't request operation");
                    assert!(resp.status().is_success());
                    let mut result = String::new();
                    resp.read_to_string(&mut result).expect("Couldn't read response");
                    result.parse().expect("Couldn't parse response")
                }
            }

            impl PartialOrd for NetMath<$T> {
                fn partial_cmp(&self, rhs: &Self) -> Option<::std::cmp::Ordering> {
                    use std::io::Read;
                    let url = format!(
                        "http://localhost:4242/cmp/{type}/{lhs}/{rhs}",
                        type=stringify!($T),
                        lhs=self.0,
                        rhs=rhs.0,
                    );
                    let mut resp = reqwest::get(&url).expect("Couldn't request operation");
                    assert!(resp.status().is_success());
                    let mut result = String::new();
                    resp.read_to_string(&mut result).expect("Couldn't read response");
                    match &result[..] {
                        "less" => Some(::std::cmp::Ordering::Less),
                        "equal" => Some(::std::cmp::Ordering::Equal),
                        "greater" => Some(::std::cmp::Ordering::Greater),
                        "none" => None,
                        ord => panic!("Unexpected ordering '{}' from server", ord),
                    }
                }
            }
        )*
    }
}

macro_rules! impl_netbits {
    ($($T:ty),*) => {
        $(
            impl_net_operator!($T, BitAnd, bitand);
            impl_net_operator!($T, BitOr, bitor);
            impl_net_operator!($T, BitXor, bitxor);
            impl_net_shift!($T, Shl<u8, u16, u32, u64>, shl);
            impl_net_shift!($T, Shl<i8, i16, i32, i64>, shl);
            impl_net_shift!($T, Shr<u8, u16, u32, u64>, shr);
            impl_net_shift!($T, Shr<i8, i16, i32, i64>, shr);
        )*
    }
}

impl_netmath!(u8, u16, u32, u64);
impl_netmath!(i8, i16, i32, i64);
impl_netmath!(f32, f64);

impl_netbits!(u8, u16, u32, u64);
impl_netbits!(i8, i16, i32, i64);


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}