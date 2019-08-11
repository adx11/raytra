use crate::rays::Ray;
use crate::tup::Point;
use crate::matrix::Matrix4x4;
use crate::intersection::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Sphere {
    transform: Matrix4x4,
}

impl Sphere {
    pub fn new() -> Sphere {
        Sphere {
            transform: Matrix4x4::IDENTITY,
        }
    }

    pub fn with_transform(transform: Matrix4x4) -> Sphere {
        Sphere {
            transform
        }
    }

    pub fn intersect(&self, ray: Ray) -> Intersections {
        if let Some(inv) = self.transform.inverse() {
            let ray_transform = ray.transform(&inv);

            let sphere_to_ray_transform = ray_transform.origin - Point::new(0.0, 0.0, 0.0);
            let a = ray_transform.direction.dot(ray_transform.direction);
            let b = 2.0 * ray_transform.direction.dot(sphere_to_ray_transform);
            let c = sphere_to_ray_transform.dot(sphere_to_ray_transform) - 1.0;

            let discriminant = b.powf(2.0) - (4.0 * a * c);

            if discriminant < 0.0 {
                Intersections::new(vec![])
            } else {
                let i1 = (-b - discriminant.sqrt()) / (2.0 * a);
                let i2 = (-b + discriminant.sqrt()) / (2.0 * a);

                Intersections::new(vec![Intersection::new(i1, *self),
                                        Intersection::new(i2, *self)])
            }
        } else {
            Intersections::new(vec![])
        }
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    use crate::rays::Ray;
    use crate::tup::{Point, Vector};
    use crate::transform::*;

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

    #[test]
    fn default_transform() {
        let s = Sphere::new();
        assert_eq!(s.transform, Matrix4x4::IDENTITY);
    }

    #[test]
    fn change_transform() {
        let t = translation(2.0, 3.0, 4.0);
        let s = Sphere::with_transform(t);

        assert_eq!(s.transform, t);
    }

    #[test]
    fn intersect_scaled() {
        let t = scaling(2.0, 2.0, 2.0);
        let s = Sphere::with_transform(t);
        let r = Ray::new(Point::new(0.0, 0.0, -5.0),
                         Vector::new(0.0, 0.0, 1.0));
        let xs = s.intersect(r);

        assert_eq!(xs.xs.len(), 2);
        assert_eq!(xs.xs[0].t, 3.0);
        assert_eq!(xs.xs[1].t, 7.0);
    }
}
