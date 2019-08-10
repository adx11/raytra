use crate::sphere::Sphere;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Intersection {
    pub t: f32,
    pub object: Sphere,
}

impl Intersection {
    pub fn new(t: f32, object: Sphere) -> Intersection {
        Intersection{t, object}
    }
}

#[derive(Debug, Clone)]
pub struct Intersections {
    pub xs: Vec<Intersection>,
}

impl Intersections {
    pub fn new(xs: Vec<Intersection>) -> Intersections {
        Intersections{xs}
    }

    pub fn hit(&self) -> Option<Intersection> {
        self.xs.iter()
            .filter(|x| x.t > 0.0)
            .min_by(|x, y| x.t.partial_cmp(&y.t).unwrap())
            .cloned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sphere::Sphere;

    #[test]
    fn intersection() {
        let s = Sphere::new();
        let i = Intersection::new(3.5, s);

        assert_eq!(i.t, 3.5);
        assert_eq!(i.object, s);
    }

    #[test]
    fn intersections() {
        let s = Sphere::new();
        let i1 = Intersection::new(1.0, s);
        let i2 = Intersection::new(2.0, s);
        let is = Intersections::new(vec![i1, i2]);

        assert_eq!(is.xs.len(), 2);
        assert_eq!(is.xs[0].t, 1.0);
        assert_eq!(is.xs[1].t, 2.0);
    }

    #[test]
    fn hit() {
        let s = Sphere::new();
        let i1 = Intersection::new(1.0, s);
        let i2 = Intersection::new(2.0, s);
        let xs = Intersections::new(vec![i1, i2]);

        assert_eq!(xs.hit().unwrap(), i1);

        let i1 = Intersection::new(-1.0, s);
        let i2 = Intersection::new(2.0, s);
        let xs = Intersections::new(vec![i1, i2]);

        assert_eq!(xs.hit().unwrap(), i2);


        let i1 = Intersection::new(5.0, s);
        let i2 = Intersection::new(7.0, s);
        let i3 = Intersection::new(-3.0, s);
        let i4 = Intersection::new(2.0, s);
        let xs = Intersections::new(vec![i1, i2, i3, i4]);

        assert_eq!(xs.hit().unwrap(), i4);
    }

    #[test]
    #[should_panic]
    fn hit_none() {
        let s = Sphere::new();
        let i1 = Intersection::new(-1.0, s);
        let i2 = Intersection::new(-2.0, s);
        let xs = Intersections::new(vec![i1, i2]);

        assert_eq!(xs.hit().unwrap(), i2);
    }

}
