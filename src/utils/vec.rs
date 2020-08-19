extern crate num_traits;

use std::fmt;
use std::fmt::{Display, Formatter};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

use num_traits::{Num, NumCast, NumOps};
use rand::distributions::uniform::SampleUniform;
use rand::distributions::{Distribution, Uniform};
use rand::thread_rng;

#[derive(Debug, Copy, Clone, Default)]
pub struct Vec3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> Vec3<T> {
    pub fn new(x: T, y: T, z: T) -> Vec3<T> {
        Vec3 { x, y, z }
    }
}

impl<T: PartialEq> PartialEq for Vec3<T> {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}
impl<T: PartialEq> Eq for Vec3<T> {}

impl<T: Copy> Vec3<T> {
    #[inline(always)]
    pub fn apply<F, K>(&self, f: F) -> Vec3<K>
    where
        F: Fn(T) -> K,
    {
        Vec3::new(f(self.x), f(self.y), f(self.z))
    }
}

/// Macro to implement element-wise vector-to-vector arithmetic operation.
macro_rules! impl_arith {
    ($tr: ident, $method: ident) => {
        impl<T: $tr + Copy> $tr<Vec3<T>> for Vec3<T> {
            type Output = Vec3<T::Output>;
            #[inline(always)]
            fn $method(self, rhs: Self) -> Self::Output {
                Vec3 {
                    x: self.x.$method(rhs.x),
                    y: self.y.$method(rhs.y),
                    z: self.z.$method(rhs.z),
                }
            }
        }
    };
}

/// Macro to implement element-wise vector-to-element arithmetic operation.
macro_rules! impl_ele_arith {
    ($tr: ident, $method: ident) => {
        impl<T: $tr + Copy> $tr<T> for Vec3<T> {
            type Output = Vec3<T::Output>;
            #[inline(always)]
            fn $method(self, rhs: T) -> Self::Output {
                Vec3::<T::Output>::new(
                    self.x.$method(rhs),
                    self.y.$method(rhs),
                    self.z.$method(rhs),
                )
            }
        }
    };
}

/// Macro to support element-wise vector-to-vector assignment operations.
macro_rules! impl_assign_arith {
    ($tr: ident, $method: ident) => {
        impl<T: $tr> $tr<Vec3<T>> for Vec3<T> {
            #[inline(always)]
            fn $method(&mut self, rhs: Vec3<T>) {
                self.x.$method(rhs.x);
                self.y.$method(rhs.y);
                self.z.$method(rhs.z);
            }
        }
    };
}

/// Macro to support element-wise vector-to-element assignment operations.
macro_rules! impl_ele_assign_arith {
    ($tr: ident, $method: ident) => {
        impl<T: $tr + Copy> $tr<T> for Vec3<T> {
            #[inline(always)]
            fn $method(&mut self, rhs: T) {
                self.x.$method(rhs);
                self.y.$method(rhs);
                self.z.$method(rhs);
            }
        }
    };
}

impl_arith!(Add, add);
impl_arith!(Sub, sub);
impl_arith!(Mul, mul);
impl_arith!(Div, div);

impl_ele_arith!(Add, add);
impl_ele_arith!(Sub, sub);
impl_ele_arith!(Mul, mul);
impl_ele_arith!(Div, div);

impl_assign_arith!(AddAssign, add_assign);
impl_assign_arith!(SubAssign, sub_assign);
impl_assign_arith!(MulAssign, mul_assign);
impl_assign_arith!(DivAssign, div_assign);

impl_ele_assign_arith!(AddAssign, add_assign);
impl_ele_assign_arith!(SubAssign, sub_assign);
impl_ele_assign_arith!(MulAssign, mul_assign);
impl_ele_assign_arith!(DivAssign, div_assign);

/// unary neg
impl<T: Neg> Neg for Vec3<T> {
    type Output = Vec3<T::Output>;

    fn neg(self) -> Self::Output {
        Vec3::new(self.x.neg(), self.y.neg(), self.z.neg())
    }
}

impl<T: Display> Display for Vec3<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

/// casting to f64
impl<T: NumCast + Copy> Vec3<T> {
    pub fn to_f64(&self) -> Vec3<f64> {
        self.apply(|t| t.to_f64().unwrap())
    }
}

impl<T: Num> Vec3<T> {
    pub fn zero() -> Vec3<T> {
        Vec3::new(T::zero(), T::zero(), T::zero())
    }
    pub fn one() -> Vec3<T> {
        Vec3::new(T::one(), T::one(), T::one())
    }
}

impl<T: Copy + SampleUniform> Vec3<T> {
    pub fn random(min: T, max: T) -> Vec3<T> {
        let mut rng = thread_rng();
        let dis = Uniform::new(min, max);
        Vec3::new(
            dis.sample(&mut rng),
            dis.sample(&mut rng),
            dis.sample(&mut rng),
        )
    }
}

impl<T: NumOps + NumCast + Copy> Vec3<T> {
    pub fn length_square(&self) -> T {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
    pub fn length(&self) -> f64 {
        self.length_square().to_f64().unwrap().sqrt()
    }
    pub fn unit_vector(&self) -> Vec3<f64> {
        self.to_f64() / self.length()
    }
}

/// various arithmetic functions that requires `T` to be `Num`.
impl<T: Num + Copy> Vec3<T> {
    pub fn dot(&self, rhs: Self) -> T {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    pub fn ele_mul(&self, rhs: Self) -> Vec3<T> {
        *self * rhs
    }

    pub fn cross(&self, rhs: Self) -> Vec3<T> {
        Vec3::new(
            self.y * rhs.z - self.z * rhs.y,
            T::zero() - (self.x * rhs.z - self.z * rhs.x),
            self.x * rhs.y - self.y * rhs.x,
        )
    }
}
