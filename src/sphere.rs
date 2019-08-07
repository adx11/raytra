use crate::rays::Ray;
use crate::tup::Point;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Sphere();

impl Sphere {
    pub fn new() -> Sphere {
        Sphere()
    }

    pub fn intersect(&self, ray: Ray) -> Vec<f32> {
        let sphere_to_ray = ray.origin - Point::new(0.0, 0.0, 0.0);
        let a = ray.direction.dot(ray.direction);
        let b = 2.0 * ray.direction.dot(sphere_to_ray);
        let c = sphere_to_ray.dot(sphere_to_ray) - 1.0;

        let discriminant = b.powf(2.0) - (4.0 * a * c);

        if discriminant < 0.0 {
            vec![]
        } else {
            vec![(-b - discriminant.sqrt()) / (2.0 * a),
                 (-b + discriminant.sqrt()) / (2.0 * a)]
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
        let xs = s.intersect(r);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0], 4.0);
        assert_eq!(xs[1], 6.0);
    }

    #[test]
    fn intersect_tangent() {
        let r = Ray::new(Point::new(0.0, 1.0, -5.0),
                         Vector::new(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let xs = s.intersect(r);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0], 5.0);
        assert_eq!(xs[1], 5.0);
    }

    #[test]
    fn intersect_miss() {
        let r = Ray::new(Point::new(0.0, 2.0, -5.0),
                         Vector::new(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let xs = s.intersect(r);

        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn intersect_interior() {
        let r = Ray::new(Point::new(0.0, 0.0, 0.0),
                         Vector::new(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let xs = s.intersect(r);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0], -1.0);
        assert_eq!(xs[1], 1.0);
    }

    #[test]
    fn intersect_behind() {
        let r = Ray::new(Point::new(0.0, 0.0, 5.0),
                         Vector::new(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let xs = s.intersect(r);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0], -6.0);
        assert_eq!(xs[1], -4.0);
    }
}
