extern crate num_traits;

use std::fmt;
use num_traits::{Num, NumOps, NumCast};
use std::ops::{Add, Sub, Mul, Div, AddAssign, SubAssign, MulAssign};
use std::fmt::{Display, Formatter};

#[derive(Debug)]
#[derive(Copy, Clone)]
#[derive(Default)]
pub struct Vec3<T>(pub T, pub T, pub T);

impl<T: PartialEq> PartialEq for Vec3<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1 && self.2 == other.2
    }
}
impl<T: Eq> Eq for Vec3<T> {}

impl<T: Add> Add<Vec3<T>> for Vec3<T> {
    type Output = Vec3<T::Output>;
    fn add(self, rhs: Self) -> Self::Output {
        Vec3(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}
impl<T: Sub> Sub for Vec3<T> {
    type Output = Vec3<T::Output>;
    fn sub(self, rhs: Self) -> Self::Output {
        Vec3(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}
impl<T: Mul> Mul for Vec3<T> {
    type Output = Vec3<T::Output>;
    fn mul(self, rhs: Self) -> Self::Output {
        Vec3(self.0 * rhs.0, self.1 * rhs.1, self.2 * rhs.2)
    }
}
impl<T: Div> Div for Vec3<T> {
    type Output = Vec3<T::Output>;
    fn div(self, rhs: Self) -> Self::Output {
        Vec3(self.0 / rhs.0, self.1 / rhs.1, self.2 / rhs.2)
    }
}

/// element-wise operation between vector and primitives
impl<T: Add + Copy> Add<T> for Vec3<T> {
    type Output = Vec3<T::Output>;
    fn add(self, rhs: T) -> Self::Output {
        Vec3(self.0 + rhs, self.1 + rhs, self.2 + rhs)
    }
}
impl<T: Sub + Copy> Sub<T> for Vec3<T> {
    type Output = Vec3<T::Output>;
    fn sub(self, rhs: T) -> Self::Output {
        Vec3(self.0 - rhs, self.1 - rhs, self.2 - rhs)
    }
}
impl<T: Mul + Copy> Mul<T> for Vec3<T> {
    type Output = Vec3<T::Output>;

    fn mul(self, rhs: T) -> Self::Output {
        Vec3(self.0 * rhs, self.1 * rhs, self.2 * rhs)
    }
}
impl<T: Div + Copy> Div<T> for Vec3<T> {
    type Output = Vec3<T::Output>;

    fn div(self, rhs: T) -> Self::Output {
        Vec3(self.0 / rhs, self.1 / rhs, self.2 / rhs)
    }
}

impl<T: AddAssign> AddAssign<Vec3<T>> for Vec3<T> {
    fn add_assign(&mut self, rhs: Vec3<T>) {
        self.0 += rhs.0;
        self.1 += rhs.1;
        self.2 += rhs.2;
    }
}

impl<T: SubAssign> SubAssign<Vec3<T>> for Vec3<T> {
    fn sub_assign(&mut self, rhs: Vec3<T>) {
        self.0 -= rhs.0;
        self.1 -= rhs.1;
        self.2 -= rhs.2;
    }
}

impl<T: MulAssign> MulAssign<Vec3<T>> for Vec3<T> {
    fn mul_assign(&mut self, rhs: Vec3<T>) {
        self.0 *= rhs.0;
        self.1 *= rhs.1;
        self.2 *= rhs.2;
    }
}

impl<T: MulAssign + Copy> MulAssign<T> for Vec3<T> {
    fn mul_assign(&mut self, rhs: T) {
        self.0 *= rhs;
        self.1 *= rhs;
        self.2 *= rhs;
    }
}


impl<T: Display> Display for Vec3<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result<> {
        write!(f, "({}, {}, {})", self.0, self.1, self.2)
    }
}

/// casting to f64
impl<T: NumCast> Vec3<T> {
    pub fn to_f64(&self) -> Vec3<f64> {
        Vec3 (
            self.0.to_f64().unwrap(),
            self.1.to_f64().unwrap(),
            self.2.to_f64().unwrap(),
        )
    }
}

impl<T: Num> Vec3<T> {
    pub fn one() -> Vec3<T> { Vec3(T::one(), T::one(), T::one()) }
}

impl<T: NumOps + NumCast + Copy> Vec3<T> {
    pub fn length_square(&self) -> T {
        self.0 * self.0 + self.1 * self.1 + self.2 * self.2
    }
    pub fn length(&self) -> f64 {
        self.length_square().to_f64().unwrap().sqrt()
    }
    pub fn unit_vector(&self) -> Vec3<f64> { self.to_f64() / self.length() }
    pub fn x(&self) -> T { self.0 }
    pub fn y(&self) -> T { self.1 }
    pub fn z(&self) -> T { self.2 }
}

/// various products
impl<T: Num + Copy> Vec3<T> {
    pub fn dot(self, rhs: Self) -> T {
        self.0 * rhs.0 + self.1 * rhs.1 + self.2 * rhs.2
    }

    pub fn ele_mul(self, rhs: Self) -> Vec3<T> { self * rhs }

    pub fn cross(self, rhs: &Self) -> Vec3<T> {
        Vec3(
            self.1 * rhs.2 - self.2 * rhs.1,
            T::zero() - (self.0 * rhs.2 - self.2 * rhs.0),
            self.0 * rhs.1 - self.1 * rhs.0
        )
    }
}