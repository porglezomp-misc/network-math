#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
extern crate iron;
extern crate reqwest;

use std::fmt::Display;
use std::sync::RwLock;

use iron::error::HttpResult;

mod server;

lazy_static! {
    static ref MATH_URL: RwLock<Option<String>> = RwLock::new(None);
}

#[derive(Debug, Copy, Clone)]
pub struct NetMath<T>(pub T);

impl<T> NetMath<T> {
    pub fn new(x: T) -> Self {
        NetMath(x)
    }
}

impl NetMath<()> {
    pub fn start_server(url: &str) -> HttpResult<iron::Listening> {
        Self::set_url(url);
        server::start_server(url)
    }

    pub fn set_url(url: &str) {
        *MATH_URL.write().unwrap() = Some(String::from(url));
    }
}

impl<T: Display> Display for NetMath<T> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(fmt, "{}", self.0)
    }
}

macro_rules! impl_net_operator {
    ($T:ty, $Op:ident, $OpAssign:ident, $op:ident, $op_assign:ident) => {
        impl ::std::ops::$Op for NetMath<$T> {
            type Output = NetMath<$T>;
            fn $op(self, rhs: Self) -> Self {
                use std::io::Read;
                let url = format!(
                    "http://{base}/{op}/{type}/{lhs}/{rhs}",
                    base=MATH_URL.read().unwrap().as_ref().unwrap(),
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

        impl ::std::ops::$OpAssign for NetMath<$T> {
            fn $op_assign(&mut self, rhs: Self) {
                use std::ops::$Op;
                *self = NetMath(self.0).$op(rhs);
            }
        }
    }
}

macro_rules! impl_net_shift {
    ($T:ty, $Op:ident < $($U:ty),* >, $OpAssign:ident, $op:ident, $op_assign:ident) => {
        $(
            impl ::std::ops::$Op <$U> for NetMath<$T> {
                type Output = NetMath<$T>;
                fn $op(self, rhs: $U) -> Self {
                    use std::io::Read;
                    let url = format!(
                        "http://{base}/{op}/{type}/{shift_type}/{lhs}/{rhs}",
                        base=MATH_URL.read().unwrap().as_ref().unwrap(),
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

            impl ::std::ops::$OpAssign <$U> for NetMath<$T> {
                fn $op_assign(&mut self, rhs: $U) {
                    use std::ops::$Op;
                    *self = NetMath(self.0).$op(rhs);
                }
            }
        )*
    }
}

macro_rules! impl_netmath {
    ($($T:ty),*) => {
        $(
            impl_net_operator!($T, Add, AddAssign, add, add_assign);
            impl_net_operator!($T, Sub, SubAssign, sub, sub_assign);
            impl_net_operator!($T, Mul, MulAssign, mul, mul_assign);
            impl_net_operator!($T, Div, DivAssign, div, div_assign);
            impl_net_operator!($T, Rem, RemAssign, rem, rem_assign);

            impl PartialEq for NetMath<$T> {
                fn eq(&self, rhs: &Self) -> bool {
                    use std::io::Read;
                    let url = format!(
                        "http://{base}/eq/{type}/{lhs}/{rhs}",
                        base=MATH_URL.read().unwrap().as_ref().unwrap(),
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
                        "http://{base}/cmp/{type}/{lhs}/{rhs}",
                        base=MATH_URL.read().unwrap().as_ref().unwrap(),
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
            impl_net_operator!($T, BitAnd, BitAndAssign, bitand, bitand_assign);
            impl_net_operator!($T, BitOr, BitOrAssign, bitor, bitor_assign);
            impl_net_operator!($T, BitXor, BitXorAssign, bitxor, bitxor_assign);
            impl_net_shift!($T, Shl<u8, u16, u32, u64>, ShlAssign, shl, shl_assign);
            impl_net_shift!($T, Shl<i8, i16, i32, i64>, ShlAssign, shl, shl_assign);
            impl_net_shift!($T, Shr<u8, u16, u32, u64>, ShrAssign, shr, shr_assign);
            impl_net_shift!($T, Shr<i8, i16, i32, i64>, ShrAssign, shr, shr_assign);
        )*
    }
}

impl_netmath!(u8, u16, u32, u64);
impl_netmath!(i8, i16, i32, i64);
impl_netmath!(f32, f64);

impl_netbits!(u8, u16, u32, u64);
impl_netbits!(i8, i16, i32, i64);
