use crate::matrix::*;

pub fn translation(x: f32, y: f32, z: f32) -> Matrix4x4 {
    let mut trans = Matrix4x4::IDENTITY;

    trans.elem[0][3] = x;
    trans.elem[1][3] = y;
    trans.elem[2][3] = z;

    trans
}

pub fn scaling(x: f32, y: f32, z: f32) -> Matrix4x4 {
    let mut trans = Matrix4x4::IDENTITY;

    trans.elem[0][0] = x;
    trans.elem[1][1] = y;
    trans.elem[2][2] = z;

    trans
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tup::*;

    #[test]
    fn translate() {
        let t = translation(5.0, -3.0, 2.0);
        assert_eq!(t, Matrix4x4::new(1.0, 0.0, 0.0, 5.0,
                                     0.0, 1.0, 0.0, -3.0,
                                     0.0, 0.0, 1.0, 2.0,
                                     0.0, 0.0, 0.0, 1.0));
        let p = Point::new(-3.0, 4.0, 5.0);

        assert_eq!(t * p, Point::new(2.0, 1.0, 7.0));
        assert_eq!(t.inverse().unwrap() * p, Point::new(-8.0, 7.0, 3.0));

        let v = Vector::new(-3.0, 4.0, 5.0);
        assert_eq!(t * v, v);
    }

    #[test]
    fn scale() {
        let t = scaling(2.0, 3.0, 4.0);
        assert_eq!(t, Matrix4x4::new(2.0, 0.0, 0.0, 0.0,
                                     0.0, 3.0, 0.0, 0.0,
                                     0.0, 0.0, 4.0, 0.0,
                                     0.0, 0.0, 0.0, 1.0));

        let p = Point::new(-4.0, 6.0, 8.0);
        assert_eq!(t * p, Point::new(-8.0, 18.0, 32.0));
        assert_eq!(t.inverse().unwrap() * p, Point::new(-2.0, 2.0, 2.0));

        let v = Vector::new(-4.0, 6.0, 8.0);
        assert_eq!(t * v, Vector::new(-8.0, 18.0, 32.0));
    }

    #[test]
    fn reflect() {
        let t = scaling(-1.0, 1.0, 1.0);
        let p = Point::new(2.0, 3.0, 4.0);

        assert_eq!(t * p, Point::new(-2.0, 3.0, 4.0));
    }
}
