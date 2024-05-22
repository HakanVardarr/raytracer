#![allow(unused)]
use std::ops::{Add, Div, Index, Mul, Neg, Sub};

macro_rules! impl_number {
    ($($type:ty),+) => {
        $(
            impl Number for $type {
                fn is_zero(self) -> bool {
                    self == 0 as $type
                }
                fn to_f64(self) -> f64{
                    self as f64
                }
            }
        )+
    };
}

pub trait Number:
    Add<Output = Self>
    + Div<Output = Self>
    + Mul<Output = Self>
    + Sub<Output = Self>
    + Copy
    + PartialEq
    + PartialOrd
    + Sized
{
    fn is_zero(self) -> bool;
    fn to_f64(self) -> f64;
}

impl_number!(i8, i16, i32, i64, i128, f32, f64, u8, u16, u32, u64, u128);

#[derive(PartialEq, PartialOrd, Clone, Copy)]
pub struct Vec3<T>
where
    T: Number,
{
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> std::fmt::Debug for Vec3<T>
where
    T: Number + std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Vec3")
            .field("x", &self.x)
            .field("y", &self.y)
            .field("z", &self.z)
            .finish()
    }
}

impl<T> From<[T; 3]> for Vec3<T>
where
    T: Number,
{
    fn from(value: [T; 3]) -> Self {
        Vec3::new(value[0], value[1], value[2])
    }
}

impl<T> From<(T, T, T)> for Vec3<T>
where
    T: Number,
{
    fn from(value: (T, T, T)) -> Self {
        Vec3::new(value.0, value.1, value.2)
    }
}

impl<T> IntoIterator for Vec3<T>
where
    T: Number,
{
    type Item = T;
    type IntoIter = std::array::IntoIter<T, 3>;
    fn into_iter(self) -> Self::IntoIter {
        [self.x, self.y, self.z].into_iter()
    }
}

impl<T> Vec3<T>
where
    T: Number,
{
    pub fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }

    pub fn dot(self, rhs: Self) -> T {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    pub fn cross(self, rhs: Self) -> Self {
        Vec3::new(
            self.y * rhs.z - self.z * rhs.y,
            self.z * rhs.x - self.x * rhs.z,
            self.x * rhs.y - self.y * rhs.x,
        )
    }

    pub fn distance(self, rhs: Self) -> f64 {
        ((self.x - rhs.x).to_f64().powi(2)
            + (self.y - rhs.y).to_f64().powi(2)
            + (self.z - rhs.z).to_f64().powi(2))
        .sqrt()
    }

    pub fn length(self) -> f64 {
        (self.x * self.x + self.y + self.y + self.z + self.z)
            .to_f64()
            .sqrt()
    }
}

impl<T> Add for Vec3<T>
where
    T: Number,
{
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Vec3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl<T> Sub for Vec3<T>
where
    T: Number,
{
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Vec3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl<T> Mul for Vec3<T>
where
    T: Number,
{
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Vec3::new(self.x * rhs.x, self.y * rhs.y, self.z * rhs.z)
    }
}

impl<T> Div for Vec3<T>
where
    T: Number,
{
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        if rhs.x.is_zero() || rhs.y.is_zero() || rhs.z.is_zero() {
            panic!("Division by zero");
        }

        Vec3::new(self.x / rhs.x, self.y / rhs.y, self.z / rhs.z)
    }
}

impl<T> Neg for Vec3<T>
where
    T: Number + Neg<Output = T>,
{
    type Output = Self;
    fn neg(self) -> Self::Output {
        Vec3::new(-self.x, -self.y, -self.z)
    }
}

impl<T> Mul<T> for Vec3<T>
where
    T: Number,
{
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        Vec3::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl<T> Div<T> for Vec3<T>
where
    T: Number,
{
    type Output = Self;
    fn div(self, rhs: T) -> Self::Output {
        if rhs.is_zero() {
            panic!("Division by zero");
        }

        Vec3::new(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}

impl<T> Index<usize> for Vec3<T>
where
    T: Number,
{
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => {
                panic!("index out of bounds: the len is 3 but the index is {index}");
            }
        }
    }
}
