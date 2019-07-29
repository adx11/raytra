use std::cmp::PartialEq;
use std::ops::{Add, Sub, Neg};

#[derive(Debug, Clone, Copy)]
struct Tup {
    x: f32,
    y: f32,
    z: f32,
    w: f32,
}

impl Tup {
    fn new(x: f32, y: f32, z: f32, w: f32) -> Tup {
        Tup{x, y, z, w}
    }

    fn is_point(&self) -> bool {
        self.w == 1.0
    }

    fn is_vector(&self) -> bool {
        self.w == 0.0
    }
}


#[derive(Debug, Clone, Copy)]
pub struct Point(Tup);
impl Point {
    pub fn new(x: f32, y: f32, z: f32) -> Point {
        Point(Tup::new(x, y, z, 1.0))
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Vector(Tup);

impl Vector {
    pub fn new(x: f32, y:f32, z: f32) -> Vector {
        Vector(Tup::new(x, y, z, 0.0))
    }
}


impl Add for Tup {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self::new(self.x + other.x,
                  self.y + other.y,
                  self.z + other.z,
                  self.w + other.w)
    }
}

impl Add<Vector> for Point {
    type Output = Self;

    fn add(self, other: Vector) -> Self {
        Self::new(self.0.x + other.0.x,
                    self.0.y + other.0.y,
                    self.0.z + other.0.z)
    }
}

impl Add for Vector {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self::new(self.0.x + other.0.x,
                    self.0.y + other.0.y,
                    self.0.z + other.0.z)
    }

}

impl Sub for Point {
    type Output = Vector;

    fn sub(self, other: Self) -> Vector {
        Vector::new(self.0.x - other.0.x,
                    self.0.y - other.0.y,
                    self.0.z - other.0.z)
    }
}

impl Sub<Vector> for Point {
    type Output = Self;

    fn sub(self, other: Vector) -> Self {
        Point::new(self.0.x - other.0.x,
                   self.0.y - other.0.y,
                   self.0.z - other.0.z)
    }
}

impl Sub for Vector {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self::new(self.0.x - other.0.x,
                   self.0.y - other.0.y,
                   self.0.z - other.0.z)
    }
}

impl Neg for Tup {
    type Output = Self;

    fn neg(self) -> Self {
        Self::new(-self.x, -self.y, -self.z, -self.w)
    }
}

impl Neg for Vector {
    type Output = Self;

    fn neg(self) -> Self {
        Vector(-self.0)
    }
}



impl PartialEq for Tup {
    fn eq(&self, other: &Self) -> bool {
        abs_diff_eq!(self.x, other.x)
            && abs_diff_eq!(self.y, other.y)
            && abs_diff_eq!(self.z, other.z)
            && abs_diff_eq!(self.w, other.w)
    }
}

impl PartialEq<Point> for Tup {
    fn eq(&self, other: &Point) -> bool {
        *self == other.0
    }
}

impl PartialEq<Vector> for Tup {
    fn eq(&self, other: &Vector) -> bool {
        *self == other.0
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl PartialEq<Tup> for Point {
    fn eq(&self, other: &Tup) -> bool {
        self.0 == *other
    }
}

impl PartialEq for Vector {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl PartialEq<Tup> for Vector {
    fn eq(&self, other: &Tup) -> bool {
        self.0 == *other
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tup_point() {
        let a = Tup::new(4.3, -4.2, 3.1, 1.0);

        assert_eq!(a.x,  4.3);
        assert_eq!(a.y, -4.2);
        assert_eq!(a.z,  3.1);
        assert_eq!(a.w,  1.0);

        assert!(a.is_point());
        assert!(!a.is_vector());
    }

    #[test]
    fn tup_vector() {
        let a = Tup::new(4.3, -4.2, 3.1, 0.0);

        assert_eq!(a.x,  4.3);
        assert_eq!(a.y, -4.2);
        assert_eq!(a.z,  3.1);
        assert_eq!(a.w,  0.0);

        assert!(!a.is_point());
        assert!(a.is_vector());
    }

    #[test]
    fn point() {
        let p = Point::new(4.0, -4.0, 3.0);
        assert_eq!(p, Tup::new(4.0, -4.0, 3.0, 1.0));
    }

    #[test]
    fn vector() {
        let v = Vector::new(4.0, -4.0, 3.0);
        assert_eq!(v, Tup::new(4.0, -4.0, 3.0, 0.0));
    }

    #[test]
    fn add() {
        let a = Tup::new(3.0, -2.0, 5.0, 1.0);
        let b = Tup::new(-2.0, 3.0, 1.0, 0.0);

        assert_eq!(a + b, Tup::new(1.0, 1.0, 6.0, 1.0));

        let a = Point::new(3.0, -2.0, 5.0);
        let b = Vector::new(-2.0, 3.0, 1.0);

        assert_eq!(a + b, Point::new(1.0, 1.0, 6.0));

        let a = Vector::new(3.0, -2.0, 5.0);
        let b = Vector::new(-2.0, 3.0, 1.0);

        assert_eq!(a + b, Vector::new(1.0, 1.0, 6.0));
    }

    #[test]
    fn sub() {
        let a = Point::new(3.0, 2.0, 1.0);
        let b = Point::new(5.0, 6.0, 7.0);

        assert_eq!(a - b, Vector::new(-2.0, -4.0, -6.0));

        let a = Point::new(3.0, 2.0, 1.0);
        let b = Vector::new(5.0, 6.0, 7.0);

        assert_eq!(a - b, Point::new(-2.0, -4.0, -6.0));

        let a = Vector::new(3.0, 2.0, 1.0);
        let b = Vector::new(5.0, 6.0, 7.0);

        assert_eq!(a - b, Vector::new(-2.0, -4.0, -6.0));

        let a = Vector::new(0.0, 0.0, 0.0);
        let b = Vector::new(1.0, -2.0, 3.0);

        assert_eq!(a - b, Vector::new(-1.0, 2.0, -3.0));
    }

    #[test]
    fn neg() {
        let a = Tup::new(1.0, -2.0, 3.0, -4.0);
        assert_eq!(-a, Tup::new(-1.0, 2.0, -3.0, 4.0));

        let a = Vector::new(1.0, -2.0, 3.0);
        assert_eq!(-a, Vector::new(-1.0, 2.0, -3.0));
    }
}
