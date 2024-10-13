#![allow(dead_code)]

use std::ops;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct Vector<T> {
    x: T,
    y: T,
}

impl<T> Vector<T> {
    fn new(x: T, y: T) -> Self {
        Vector { x, y }
    }
}

impl<T> ops::Add for Vector<T>
where
    T: ops::Add<Output = T>,
{
    type Output = Vector<T>;

    fn add(self, rhs: Self) -> Self::Output {
        Vector::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl<T> ops::Sub for Vector<T>
where
    T: ops::Sub<Output = T>,
{
    type Output = Vector<T>;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl<T> ops::AddAssign for Vector<T>
where
    T: ops::AddAssign,
{
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl<T> ops::SubAssign for Vector<T>
where
    T: ops::SubAssign,
{
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl<T> ops::Mul<T> for Vector<T>
where
    T: ops::Mul<Output = T> + Copy,
{
    type Output = Vector<T>;

    fn mul(self, rhs: T) -> Self::Output {
        Vector::new(self.x * rhs, self.y * rhs)
    }
}

impl<T> ops::MulAssign<T> for Vector<T>
where
    T: ops::MulAssign + Copy,
{
    fn mul_assign(&mut self, rhs: T) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl<T> ops::Div<T> for Vector<T>
where
    T: ops::Div<Output = T> + Copy,
{
    type Output = Vector<T>;

    fn div(self, rhs: T) -> Self::Output {
        Vector::new(self.x / rhs, self.y / rhs)
    }
}

impl<T> ops::DivAssign<T> for Vector<T>
where
    T: ops::DivAssign + Copy,
{
    fn div_assign(&mut self, rhs: T) {
        self.x /= rhs;
        self.y /= rhs;
    }
}

impl Vector<f32> {
    fn length(&self) -> f32 {
        f32::sqrt(self.x * self.x + self.y * self.y)
    }
}

impl Vector<f64> {
    fn length(&self) -> f64 {
        f64::sqrt(self.x * self.x + self.y * self.y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a() {
        let v = Vector {
            x: String::from("Hello, World!"),
            y: String::from("Hello, Rust!"),
        };

        let w = v.clone();

        assert_eq!(&v, &w);
    }

    #[test]
    fn test_b() {
        let v = Vector::new("Hello, World!", "Hello, Rust!");
        let a = v;
        let b = v;
        assert_eq!(a, b);
    }

    #[test]
    fn test_vector_creation() {
        let v = Vector::new(1, 2);
        assert_eq!(v.x, 1);
        assert_eq!(v.y, 2);
    }

    #[test]
    fn test_vector_addition() {
        let v1 = Vector::new(1, 2);
        let v2 = Vector::new(3, 4);
        let v3 = v1 + v2;
        assert_eq!(v3, Vector::new(4, 6));
    }

    #[test]
    fn test_vector_subtraction() {
        let v1 = Vector::new(5, 7);
        let v2 = Vector::new(3, 4);
        let v3 = v1 - v2;
        assert_eq!(v3, Vector::new(2, 3));
    }

    #[test]
    fn test_vector_scalar_multiplication() {
        let v = Vector::new(2, 3);
        let v2 = v * 2;
        assert_eq!(v2, Vector::new(4, 6));
    }

    #[test]
    fn test_vector_scalar_division() {
        let v = Vector::new(4, 6);
        let v2 = v / 2;
        assert_eq!(v2, Vector::new(2, 3));
    }

    #[test]
    fn test_vector_add_assign() {
        let mut v1 = Vector::new(1, 2);
        let v2 = Vector::new(3, 4);
        v1 += v2;
        assert_eq!(v1, Vector::new(4, 6));
    }

    #[test]
    fn test_vector_sub_assign() {
        let mut v1 = Vector::new(5, 7);
        let v2 = Vector::new(3, 4);
        v1 -= v2;
        assert_eq!(v1, Vector::new(2, 3));
    }

    #[test]
    fn test_vector_mul_assign() {
        let mut v = Vector::new(2, 3);
        v *= 2;
        assert_eq!(v, Vector::new(4, 6));
    }

    #[test]
    fn test_vector_div_assign() {
        let mut v = Vector::new(4, 6);
        v /= 2;
        assert_eq!(v, Vector::new(2, 3));
    }

    #[test]
    fn test_vector_length_f32() {
        let v = Vector::new(3.0f32, 4.0f32);
        assert_eq!(v.length(), 5.0f32);
    }

    #[test]
    fn test_vector_length_f64() {
        let v = Vector::new(6.0f64, 8.0f64);
        assert_eq!(v.length(), 10.0f64);
    }
}
