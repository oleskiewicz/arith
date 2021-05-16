//! Containers with arithmetic operations support.

use std::collections::HashMap;
use std::fmt;
use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign};


#[derive(Eq, PartialEq, Default)]
pub struct ArithMap<'a, V> {
    pub hashmap: HashMap<&'a str, V>,
}

impl<V> fmt::Debug for ArithMap<'_, V>
where
    V: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_map()
            .entries(self.hashmap.iter().map(|(k, v)| (k, v)))
            .finish()
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

impl<V> ArithMap<'_, V>
where
    V: Copy + Default + PartialEq,
{
    /// ```
    /// # use arith::*;
    /// let mut x = arithmap!{"a" => 0, "b" => 1};
    /// let y = arithmap!{"b" => 1};
    /// x.prune();
    ///
    /// assert_eq!(x, y);
    /// ```
    pub fn prune(&mut self) {
        let zero: V = Default::default();
        self.hashmap.retain(|_, &mut v| v != zero);
    }
}

/// ```
/// # use arith::*;
/// let x = arithmap!{"a" => 1, "b" => 2};
/// let y = arithmap!{"a" => 2, "b" => 3};
///
/// assert_eq!(x + 1, y);
/// ```
impl<V> Add<V> for ArithMap<'_, V>
where
    V: AddAssign + Copy,
{
    type Output = Self;
    fn add(mut self: Self, other: V) -> Self {
        for v in self.hashmap.values_mut() {
            *v += other;
        }
        self
    }
}

/// ```
/// # use arith::*;
/// let mut x = arithmap!{"a" => 1, "b" => 2};
/// x += 1;
/// let y = arithmap!{"a" => 2, "b" => 3};
///
/// assert_eq!(x, y);
/// ```
impl<V> AddAssign<V> for ArithMap<'_, V>
where
    V: AddAssign + Copy,
{
    fn add_assign(&mut self, other: V) {
        for v in self.hashmap.values_mut() {
            *v += other;
        }
    }
}

/// ```
/// # use arith::*;
/// let x = arithmap!{"a" => 1, "b" => 2};
/// let y = arithmap!{"a" => 0, "b" => 1};
///
/// assert_eq!(x - 1, y);
/// ```
impl<V> Sub<V> for ArithMap<'_, V>
where
    V: SubAssign + Copy,
{
    type Output = Self;
    fn sub(mut self: Self, other: V) -> Self {
        for v in self.hashmap.values_mut() {
            *v -= other;
        }
        self
    }
}

/// ```
/// # use arith::*;
/// let mut x = arithmap!{"a" => 1, "b" => 2};
/// x -= 1;
/// let y = arithmap!{"a" => 0, "b" => 1};
///
/// assert_eq!(x, y);
/// ```
impl<V> SubAssign<V> for ArithMap<'_, V>
where
    V: SubAssign + Copy,
{
    fn sub_assign(&mut self, other: V) {
        for v in self.hashmap.values_mut() {
            *v -= other;
        }
    }
}

/// ```
/// # use arith::*;
/// let x = arithmap!{"a" => 1.0, "b" => 2.0};
/// let y = arithmap!{"a" => 2.0, "b" => 4.0};
///
/// assert_eq!(x * 2.0, y);
/// ```
impl<V> Mul<V> for ArithMap<'_, V>
where
    V: MulAssign + Copy,
{
    type Output = Self;
    fn mul(mut self: Self, other: V) -> Self {
        for v in self.hashmap.values_mut() {
            *v *= other;
        }
        self
    }
}

/// ```
/// # use arith::*;
/// let mut x = arithmap!{"a" => 1, "b" => 2};
/// x *= 2;
/// let y = arithmap!{"a" => 2, "b" => 4};
///
/// assert_eq!(x, y);
/// ```
impl<V> MulAssign<V> for ArithMap<'_, V>
where
    V: MulAssign + Copy,
{
    fn mul_assign(&mut self, other: V) {
        for v in self.hashmap.values_mut() {
            *v *= other;
        }
    }
}

/// ```
/// # use arith::*;
/// let x = arithmap!{"a" => 1, "b" => 2};
/// let y = arithmap!{"b" => 2, "c" => 3};
/// let z = arithmap!{"a" => 1, "b" => 4, "c" => 3};
///
/// assert_eq!(x + y, z);
/// ```
impl<V> Add for ArithMap<'_, V>
where
    V: Add<Output = V> + AddAssign + Copy + Default,
{
    type Output = Self;
    fn add(self: Self, other: Self) -> Self {
        let mut r: Self = Default::default();
        for (k, v) in self.hashmap.iter() {
            r.hashmap.insert(*k, *v);
        }
        for (k, v2) in other.hashmap.iter() {
            if let Some(v1) = r.hashmap.get_mut(k) {
                *v1 += *v2;
            } else {
                r.hashmap.insert(*k, *v2);
            }
        }
        r
    }
}

/// ```
/// # use arith::*;
/// let mut x = arithmap!{"a" => 1, "b" => 2};
/// let y = arithmap!{"b" => 2, "c" => 3};
/// x += y;
/// let z = arithmap!{"a" => 1, "b" => 4, "c" => 3};
///
/// assert_eq!(x, z);
/// ```
impl<V> AddAssign for ArithMap<'_, V>
where
    V: Add<Output = V> + AddAssign + Copy + Default,
{
    fn add_assign(&mut self, other: Self) {
        for (k, v2) in other.hashmap.iter() {
            if let Some(v1) = self.hashmap.get_mut(k) {
                *v1 += *v2;
            } else {
                self.hashmap.insert(*k, *v2);
            }
        }
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
impl<V> Sub for ArithMap<'_, V>
where
    V: Sub<Output = V> + SubAssign + Copy + Default,
{
    type Output = Self;
    fn sub(self: Self, other: Self) -> Self {
        let zero: V = Default::default();
        let mut r: Self = Default::default();
        for (k, v) in self.hashmap.iter() {
            r.hashmap.insert(*k, *v);
        }
        for (k, v2) in other.hashmap.iter() {
            if let Some(v1) = r.hashmap.get_mut(k) {
                *v1 -= *v2;
            } else {
                r.hashmap.insert(*k, zero - *v2);
            }
        }
        r
    }
}

/// ```
/// # use arith::*;
/// let mut x = arithmap!{"a" => 1, "b" => 2};
/// let y = arithmap!{"b" => 2, "c" => 3};
/// x -= y;
/// let z = arithmap!{"a" => 1, "b" => 0, "c" => -3};
///
/// assert_eq!(x, z);
/// ```
impl<V> SubAssign for ArithMap<'_, V>
where
    V: Sub<Output = V> + SubAssign + Copy + Default,
{
    fn sub_assign(&mut self, other: Self) {
        let zero: V = Default::default();
        for (k, v2) in other.hashmap.iter() {
            if let Some(v1) = self.hashmap.get_mut(k) {
                *v1 -= *v2;
            } else {
                self.hashmap.insert(*k, zero - *v2);
            }
        }
    }
}
