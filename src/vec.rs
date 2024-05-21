use std::ops::{Add, Div, Mul, Neg, Sub};

macro_rules! impl_number {
    ($($type:ty),+) => {
        $(
            impl Number for $type {}
        )+
    };
}

pub trait Number:
    Add<Output = Self> + Div<Output = Self> + Mul<Output = Self> + Sub<Output = Self> + Copy
{
}

impl_number!(i8, i16, i32, i64, i128, f32, f64, u8, u16, u32, u64, u128);

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
