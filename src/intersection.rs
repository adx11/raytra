struct Intersection<T> {
    t: f32,
    object: T,
}

impl<T> Intersection<T> {
    pub fn new(t: f32, object: T) -> Intersection<T> {
        Intersection{t, object}
    }
}


struct Intersections<T> {
    xs: Vec<Intersection<T>>,
}

impl<T> Intersections<T> {
    pub fn new(xs: Vec<Intersection<T>>) -> Intersections<T> {
        Intersections{xs}
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
}
