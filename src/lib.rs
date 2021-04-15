//! Containers with arithmetic operations support.

use std::collections::HashMap;
use std::fmt;
use std::ops::{Add, Sub, Mul};


#[derive(Eq, PartialEq, Default)]
pub struct ArithMap<'a> {
    pub hashmap: HashMap<&'a str, i32>,
}

impl fmt::Debug for ArithMap<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_map().entries(self.hashmap.iter().map(|(k, v)| (k, v))).finish()
    }
}

#[macro_export(local_inner_macros)]
/// ```
/// # use arith::*;
/// let map = arithmap!{"a" => 1, "b" => 2};
/// assert_eq!(map.hashmap["a"], 1);
/// assert_eq!(map.hashmap["b"], 2);
/// assert_eq!(map.hashmap.get("c"), None);
/// ```
macro_rules! arithmap {
    (@single $($x:tt)*) => (());
    (@count $($rest:expr),*) => (<[()]>::len(&[$(arithmap!(@single $rest)),*]));
    ($($key:expr => $value:expr,)+) => { arithmap!($($key => $value),+) };
    ($($key:expr => $value:expr),*) => {
        {
            let _cap = arithmap!(@count $($key),*);
            let mut _map = ::std::collections::HashMap::with_capacity(_cap);
            $(
                let _ = _map.insert($key, $value);
            )*
            ArithMap{hashmap: _map}
        }
    };
}


/// ```
/// # use arith::*;
/// let x = arithmap!{"a" => 1, "b" => 2};
/// let y = arithmap!{"b" => 2, "c" => 3};
/// let z = arithmap!{"a" => 1, "b" => 4, "c" => 3};
///
/// assert_eq!(x + y, z);
/// ```
impl Add for ArithMap<'_> {
    type Output = Self;
    fn add(self: Self, other: Self) -> Self {
        let mut r: Self = Default::default();
        for k in self.hashmap.keys() {
            if !r.hashmap.contains_key(k) {
                r.hashmap.insert(*k, 0);
            }
        }
        for k in other.hashmap.keys() {
            if !r.hashmap.contains_key(k) {
                r.hashmap.insert(*k, 0);
            }
        }
        for (k, v) in r.hashmap.iter_mut() {
            *v = *self.hashmap.get(k).unwrap_or(&0) + *other.hashmap.get(k).unwrap_or(&0);
        }
        r
    }
}


/// ```
/// # use arith::*;
/// let x = arithmap!{"a" => 1, "b" => 2};
/// let y = arithmap!{"a" => 2, "b" => 3};
///
/// assert_eq!(x + 1, y);
/// ```
impl Add<i32> for ArithMap<'_> {
    type Output = Self;
    fn add(mut self: Self, other: i32) -> Self {
        for v in self.hashmap.values_mut() {
            *v += other;
        }
        self
    }
}


/// ```
/// # use arith::*;
/// let x = arithmap!{"a" => 1, "b" => 2};
/// let y = arithmap!{"b" => 2, "c" => 3};
/// let z = arithmap!{"a" => 1, "b" => 0, "c" => -3};
///
/// assert_eq!(x - y, z);
/// ```
impl Sub for ArithMap<'_> {
    type Output = Self;
    fn sub(self: Self, other: Self) -> Self {
        let mut r: Self = Default::default();
        for k in self.hashmap.keys() {
            if !r.hashmap.contains_key(k) {
                r.hashmap.insert(k, 0);
            }
        }
        for k in other.hashmap.keys() {
            if !r.hashmap.contains_key(k) {
                r.hashmap.insert(k, 0);
            }
        }
        for (k, v) in r.hashmap.iter_mut() {
            *v = *self.hashmap.get(k).unwrap_or(&0) - *other.hashmap.get(k).unwrap_or(&0);
        }
        r
    }
}


/// ```
/// # use arith::*;
/// let x = arithmap!{"a" => 1, "b" => 2};
/// let y = arithmap!{"a" => 2, "b" => 4};
///
/// assert_eq!(x * 2, y);
/// ```
impl Mul<i32> for ArithMap<'_> {
    type Output = Self;
    fn mul(mut self: Self, other: i32) -> Self {
        for v in self.hashmap.values_mut() {
            *v *= other;
        }
        self
    }
}
