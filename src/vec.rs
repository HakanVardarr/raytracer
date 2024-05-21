use std::ops::{Add, Div, Mul, Neg, Sub};

macro_rules! impl_number {
    ($($type:ty),+) => {
        $(
            impl Number for $type {
                fn is_zero(self) -> bool {
                    self == 0 as $type
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
{
    fn is_zero(self) -> bool;
}

impl_number!(i8, i16, i32, i64, i128, f32, f64, u8, u16, u32, u64, u128);

#[derive(Debug, PartialEq, PartialOrd)]
pub struct Vec3<T>
where
    T: Number,
{
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> Vec3<T>
where
    T: Number,
{
    pub fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
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
