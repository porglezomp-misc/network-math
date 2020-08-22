use std::ops::{Add, BitAnd, BitOr, BitXor, Div, Mul, Rem, Shl, Shr, Sub};
use std::str::FromStr;

use iron::error::HttpResult;
use iron::prelude::*;
use iron::{self, status};

pub fn start_server(url: &str) -> HttpResult<iron::Listening> {
    Iron::new(handler).http(url)
}

fn parse_val<T: FromStr>(text: &str) -> Result<T, IronError>
where
    T::Err: ::std::error::Error + Send + 'static,
{
    text.parse()
        .map_err(|err| IronError::new(err, status::BadRequest))
}

fn ok<S: Into<String>>(text: S) -> IronResult<Response> {
    Ok(Response::with((status::Ok, text.into())))
}

fn bad<S: Into<String>>(text: S) -> IronResult<Response> {
    Ok(Response::with((status::BadRequest, text.into())))
}

macro_rules! handle_bit {
    ($op:ident, $path:expr) => {
        handle_op!($op, $path, {u8, u16, u32, u64, i8, i16, i32, i64})
    }
}

macro_rules! handle_op {
    ($op:ident, $path:expr) => {
        handle_op!($op, $path, {u8, u16, u32, u64, i8, i16, i32, i64, f32, f64})
    };
    ($op:ident, $path:expr, {$($ty:ty),*}) => {
        if $path.len() != 4 {
            bad(format!("Wrong number of path segments in {}", stringify!($op)))
        } else {
            handle_op!($op, $path, [$($ty),*])
        }
    };
    ($op:ident, $path:expr, [$($ty:ty),*]) => {
        match $path[1] {
            $(
                stringify!($ty) => {
                    let lhs: $ty = parse_val($path[2])?;
                    let rhs: $ty = parse_val($path[3])?;
                    Ok(Response::with((
                        status::Ok,
                        format!("{}", lhs.$op(rhs)),
                    )))
                }
            )*
            ty => bad(format!("Invalid type '{}'", ty)),
        }
    }
}

macro_rules! handle_shift {
    ($op:ident, $path:expr) => {
        if $path.len() != 5 {
            bad(format!("Wrong number of path segments in {}", stringify!($op)))
        } else {
            handle_shift!(
                $op,
                $path,
                [u8, u16, u32, u64, i8, i16, i32, i64]
            )
        }
    };
    ($op:ident, $path:expr, [$($ty:ty),*]) => {
        match $path[1] {
            $(
                stringify!($ty) => handle_shift!(
                    $op,
                    $path,
                    $ty,
                    [u8, u16, u32, u64, i8, i16, i32, i64]
                ),
            )*
            ty => bad(format!("Invalid type '{}'", ty)),
        }
    };
    ($op:ident, $path:expr, $ty:ty, [$($shift:ty),*]) => {
        match $path[2] {
            $(
                stringify!($shift) => {
                    let lhs: $ty = parse_val($path[3])?;
                    let rhs: $shift = parse_val($path[4])?;
                    ok(format!("{}", lhs.$op(rhs)))
                }
            )*
            ty => bad(format!("Invalid type '{}'", ty)),
        }
    }
}

macro_rules! handle_eq {
    ($path:expr) => {
        if $path.len() != 4 {
            bad("Wrong number of path segments in eq")
        } else {
            handle_eq!(
                $path,
                [u8, u16, u32, u64, i8, i16, i32, i64, f32, f64]
            )
        }
    };
    ($path:expr, [$($ty:ty),*]) => {
        match $path[1] {
            $(
                stringify!($ty) => {
                    let lhs: $ty = parse_val($path[2])?;
                    let rhs: $ty = parse_val($path[3])?;
                    ok(format!("{}", lhs == rhs))
                }
            )*
            ty => bad(format!("Invalid type '{}'", ty)),
        }
    }
}

macro_rules! handle_cmp {
    ($path:expr) => {
        if $path.len() != 4 {
            bad("Wrong number of path segments in cmp")
        } else {
            handle_cmp!(
                $path,
                [u8, u16, u32, u64, i8, i16, i32, i64, f32, f64]
            )
        }
    };
    ($path:expr, [$($ty:ty),*]) => {
        match $path[1] {
            $(
                stringify!($ty) => {
                    let lhs: $ty = parse_val($path[2])?;
                    let rhs: $ty = parse_val($path[3])?;
                    use std::cmp::Ordering::*;
                    match lhs.partial_cmp(&rhs) {
                        Some(Less) => ok("less"),
                        Some(Equal) => ok("equal"),
                        Some(Greater) => ok("greater"),
                        None => ok("none"),
                    }
                }
            )*
            ty => bad(format!("Invalid type '{}'", ty)),
        }
    }
}

fn handler(req: &mut Request) -> IronResult<Response> {
    let path = req.url.path();
    info!("GET /{}", path.join("/"));
    if path.len() < 4 {
        return Ok(Response::with((
            status::BadRequest,
            "Not enough path segments!",
        )));
    }

    match path[0] {
        "add" => handle_op!(add, path),
        "sub" => handle_op!(sub, path),
        "mul" => handle_op!(mul, path),
        "div" => handle_op!(div, path),
        "rem" => handle_op!(rem, path),
        "bitand" => handle_bit!(bitand, path),
        "bitor" => handle_bit!(bitor, path),
        "bitxor" => handle_bit!(bitxor, path),
        "shl" => handle_shift!(shl, path),
        "shr" => handle_shift!(shr, path),
        "eq" => handle_eq!(path),
        "cmp" => handle_cmp!(path),
        op => bad(format!("Invalid op '{}'", op)),
    }
}
