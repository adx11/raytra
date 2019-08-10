use crate::rays::Ray;
use crate::tup::Point;
use crate::intersection::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Sphere();

impl Sphere {
    pub fn new() -> Sphere {
        Sphere()
    }

    pub fn intersect(&self, ray: Ray) -> Intersections<Sphere> {
        let sphere_to_ray = ray.origin - Point::new(0.0, 0.0, 0.0);
        let a = ray.direction.dot(ray.direction);
        let b = 2.0 * ray.direction.dot(sphere_to_ray);
        let c = sphere_to_ray.dot(sphere_to_ray) - 1.0;

        let discriminant = b.powf(2.0) - (4.0 * a * c);

        if discriminant < 0.0 {
            Intersections::new(vec![])
        } else {
            Intersections::new(vec![Intersection::new((-b - discriminant.sqrt()) / (2.0 * a), *self),
                                    Intersection::new((-b + discriminant.sqrt()) / (2.0 * a), *self)])
        }
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    use crate::rays::Ray;
    use crate::tup::{Point, Vector};

    #[test]
    fn intersect() {
        let r = Ray::new(Point::new(0.0, 0.0, -5.0),
                         Vector::new(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let is = s.intersect(r);

        assert_eq!(is.xs.len(), 2);
        assert_eq!(is.xs[0].t, 4.0);
        assert_eq!(is.xs[1].t, 6.0);
    }

    #[test]
    fn intersect_tangent() {
        let r = Ray::new(Point::new(0.0, 1.0, -5.0),
                         Vector::new(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let is = s.intersect(r);

        assert_eq!(is.xs.len(), 2);
        assert_eq!(is.xs[0].t, 5.0);
        assert_eq!(is.xs[1].t, 5.0);
    }

    #[test]
    fn intersect_miss() {
        let r = Ray::new(Point::new(0.0, 2.0, -5.0),
                         Vector::new(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let is = s.intersect(r);

        assert_eq!(is.xs.len(), 0);
    }

    #[test]
    fn intersect_interior() {
        let r = Ray::new(Point::new(0.0, 0.0, 0.0),
                         Vector::new(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let is = s.intersect(r);

        assert_eq!(is.xs.len(), 2);
        assert_eq!(is.xs[0].t, -1.0);
        assert_eq!(is.xs[1].t, 1.0);
    }

    #[test]
    fn intersect_behind() {
        let r = Ray::new(Point::new(0.0, 0.0, 5.0),
                         Vector::new(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let is = s.intersect(r);

        assert_eq!(is.xs.len(), 2);
        assert_eq!(is.xs[0].t, -6.0);
        assert_eq!(is.xs[1].t, -4.0);
    }

    #[test]
    fn intersect_objects() {
        let r = Ray::new(Point::new(0.0, 0.0, -5.0),
                         Vector::new(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let is = s.intersect(r);

        assert_eq!(is.xs.len(), 2);
        assert_eq!(is.xs[0].object, s);
        assert_eq!(is.xs[1].object, s);
    }
}
