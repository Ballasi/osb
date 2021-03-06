use crate::utils::Number;
use std::ops::{Add, AddAssign, Neg, Sub, SubAssign};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vec2 {
    pub x: Number,
    pub y: Number,
}

impl Default for Vec2 {
    fn default() -> Self {
        Self {
            x: Number::Int(0),
            y: Number::Int(0),
        }
    }
}

impl Vec2 {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from<T, U>(x: T, y: U) -> Self
    where
        T: Into<Number>,
        U: Into<Number>
    {
        (x, y).into()
    }

}


impl<T, U> From<(T, U)> for Vec2
where
    T: Into<Number>,
    U: Into<Number>,
{
    fn from((x, y): (T, U)) -> Self
    {
        Self {
            x: x.into(),
            y: y.into(),
        }
    }
}

impl Add for Vec2 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Vec2 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl AddAssign for Vec2 {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}

impl SubAssign for Vec2 {
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}

impl Neg for Vec2 {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::Vec2;

    #[test]
    fn add() {
        let v1 = Vec2::from(20, 30);
        let v2 = Vec2::from(10, 20);
        assert_eq!(v1 + v2, Vec2::from(30, 50));
    }

    #[test]
    fn sub() {
        let v1 = Vec2::from(20, 30);
        let v2 = Vec2::from(10, 20);
        assert_eq!(v1 - v2, Vec2::from(10, 10));
    }

    #[test]
    fn add_assign() {
        let mut v = Vec2::from(20, 30);
        v += Vec2::from(10, 20);
        assert_eq!(v, Vec2::from(30, 50));
    }

    #[test]
    fn sub_assign() {
        let mut v = Vec2::from(20, 30);
        v -= Vec2::from(10, 20);
        assert_eq!(v, Vec2::from(10, 10));
    }

    #[test]
    fn neg() {
        let v = Vec2::from(10, 20);
        assert_eq!(-v, Vec2::from(-10, -20));
    }
}
