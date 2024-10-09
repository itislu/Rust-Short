#![allow(dead_code)]

struct Point {
    x: f32,
    y: f32,
}

impl Point {
    fn new(x: f32, y: f32) -> Self {
        Point {
            x,
            y,
        }
    }

    fn zero() -> Self {
        Point {
            x: 0.0,
            y: 0.0,
        }
    }

    fn distance(&self, other: &Self) -> f32 {
        let a = self.x - other.x;
        let b = self.y - other.y;
        f32::sqrt(a * a + b * b)
    }

    fn translate(&mut self, dx: f32, dy: f32) {
        self.x += dx;
        self.y += dy;
    }
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_new() {
        let p = Point::new(3.0, 4.0);
        assert_eq!(p.x, 3.0);
        assert_eq!(p.y, 4.0);
    }

    #[test]
    fn test_zero() {
        let p = Point::zero();
        assert_eq!(p.x, 0.0);
        assert_eq!(p.y, 0.0);
    }

    #[test]
    fn test_distance() {
        let p1 = Point::new(0.0, 0.0);
        let p2 = Point::new(3.0, 4.0);
        assert_eq!(p1.distance(&p2), 5.0);
    }

    #[test]
    fn test_translate() {
        let mut p = Point::new(1.0, 1.0);
        p.translate(2.0, 3.0);
        assert_eq!(p.x, 3.0);
        assert_eq!(p.y, 4.0);
    }
}
