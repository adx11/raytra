use crate::tup::*;
use crate::matrix::Matrix4x4;

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    pub origin: Point,
    pub direction: Vector,
}

impl Ray {
    pub fn new(origin: Point, direction: Vector) -> Ray {
        Ray{origin, direction}
    }

    pub fn position(&self, t: f32) -> Point {
        self.origin + self.direction * t
    }

    pub fn transform(&self, trans: &Matrix4x4) -> Ray {
        let o = *trans * self.origin;
        let d = *trans * self.direction;

        Ray::new(o, d)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::transform::*;

    #[test]
    fn ray() {
        let o = Point::new(1.0, 2.0, 3.0);
        let d = Vector::new(4.0, 5.0, 6.0);
        let r = Ray::new(o, d);

        assert_eq!(r.origin, o);
        assert_eq!(r.direction, d);
    }

    #[test]
    fn distance() {
        let r = Ray::new(Point::new(2.0, 3.0, 4.0),
                         Vector::new(1.0, 0.0, 0.0));
        assert_eq!(r.position(0.0), Point::new(2.0, 3.0, 4.0));
        assert_eq!(r.position(1.0), Point::new(3.0, 3.0, 4.0));
        assert_eq!(r.position(-1.0), Point::new(1.0, 3.0, 4.0));
        assert_eq!(r.position(2.5), Point::new(4.5, 3.0, 4.0));
    }

    #[test]
    fn translate() {
        let r = Ray::new(Point::new(1.0, 2.0, 3.0),
                         Vector::new(0.0, 1.0, 0.0));
        let m = translation(3.0, 4.0, 5.0);
        let r2 = r.transform(&m);

        assert_eq!(r2.origin, Point::new(4.0, 6.0, 8.0));
        assert_eq!(r2.direction, Vector::new(0.0, 1.0, 0.0));
    }

    #[test]
    fn scale() {
        let r = Ray::new(Point::new(1.0, 2.0, 3.0),
                         Vector::new(0.0, 1.0, 0.0));
        let m = scaling(2.0, 3.0, 4.0);
        let r2 = r.transform(&m);

        assert_eq!(r2.origin, Point::new(2.0, 6.0, 12.0));
        assert_eq!(r2.direction, Vector::new(0.0, 3.0, 0.0));
    }
}
