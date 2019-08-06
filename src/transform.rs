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

pub fn rotation_x(rad: f32) -> Matrix4x4 {
    let mut trans = Matrix4x4::IDENTITY;

    trans.elem[1][1] = rad.cos();
    trans.elem[1][2] = -rad.sin();
    trans.elem[2][1] = rad.sin();
    trans.elem[2][2] = rad.cos();

    trans
}

pub fn rotation_y(rad: f32) -> Matrix4x4 {
    let mut trans = Matrix4x4::IDENTITY;

    trans.elem[0][0] = rad.cos();
    trans.elem[0][2] = rad.sin();
    trans.elem[2][0] = -rad.sin();
    trans.elem[2][2] = rad.cos();

    trans
}

pub fn rotation_z(rad: f32) -> Matrix4x4 {
    let mut trans = Matrix4x4::IDENTITY;

    trans.elem[0][0] = rad.cos();
    trans.elem[0][1] = -rad.sin();
    trans.elem[1][0] = rad.sin();
    trans.elem[1][1] = rad.cos();

    trans
}

pub fn shearing(xy: f32, xz: f32,
                yx: f32, yz: f32,
                zx: f32, zy: f32) -> Matrix4x4 {
    let mut trans = Matrix4x4::IDENTITY;

    trans.elem[0][1] = xy;
    trans.elem[0][2] = xz;
    trans.elem[1][0] = yx;
    trans.elem[1][2] = yz;
    trans.elem[2][0] = zx;
    trans.elem[2][1] = zy;

    trans
}

impl Matrix4x4 {
    pub fn translate(self, x: f32, y: f32, z: f32) -> Matrix4x4 {
        translation(x, y, z) * self
    }

    pub fn scale(self, x: f32, y: f32, z: f32) -> Matrix4x4 {
        scaling(x, y, z) * self
    }

    pub fn rotate_x(self, rad: f32) -> Matrix4x4 {
        rotation_x(rad) * self
    }

    pub fn rotate_y(self, rad: f32) -> Matrix4x4 {
        rotation_y(rad) * self
    }

    pub fn rotate_z(self, rad: f32) -> Matrix4x4 {
        rotation_z(rad) * self
    }

    pub fn shear(self,
                 xy: f32, xz: f32,
                 yx: f32, yz: f32,
                 zx: f32, zy: f32) -> Matrix4x4 {
        shearing(xy, xz, yx, yz, zx, zy) * self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tup::*;
    use std::f32::consts::PI;

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

    #[test]
    fn rotate() {
        let p = Point::new(0.0, 1.0, 0.0);
        assert_eq!(rotation_x(PI / 4.0) * p,
                   Point::new(0.0, 2.0_f32.sqrt() / 2.0, 2.0_f32.sqrt() / 2.0));
        assert_eq!(rotation_x(PI / 2.0) * p,
                   Point::new(0.0, 0.0, 1.0));
        assert_eq!(rotation_x(PI / 4.0).inverse().unwrap() * p,
                   Point::new(0.0, 2.0_f32.sqrt() / 2.0, -2.0_f32.sqrt() / 2.0));

        let p = Point::new(0.0, 0.0, 1.0);
        assert_eq!(rotation_y(PI / 4.0) * p,
                   Point::new(2.0_f32.sqrt() / 2.0, 0.0, 2.0_f32.sqrt() / 2.0));
        assert_eq!(rotation_y(PI / 2.0) * p,
                   Point::new(1.0, 0.0, 0.0));

        let p = Point::new(0.0, 1.0, 0.0);
        assert_eq!(rotation_z(PI / 4.0) * p,
                   Point::new(-2.0_f32.sqrt() / 2.0, 2.0_f32.sqrt() / 2.0, 0.0));
        assert_eq!(rotation_z(PI / 2.0) * p,
                   Point::new(-1.0, 0.0, 0.0));
    }

    #[test]
    fn shear() {
        let p = Point::new(2.0, 3.0, 4.0);

        let t = shearing(1.0, 0.0, 0.0, 0.0, 0.0, 0.0);
        assert_eq!(t * p, Point::new(5.0, 3.0, 4.0));

        let t = shearing(0.0, 0.0, 1.0, 0.0, 0.0, 0.0);
        assert_eq!(t * p, Point::new(2.0, 5.0, 4.0));

        let t = shearing(0.0, 0.0, 0.0, 1.0, 0.0, 0.0);
        assert_eq!(t * p, Point::new(2.0, 7.0, 4.0));

        let t = shearing(0.0, 0.0, 0.0, 0.0, 1.0, 0.0);
        assert_eq!(t * p, Point::new(2.0, 3.0, 6.0));

        let t = shearing(0.0, 0.0, 0.0, 0.0, 0.0, 1.0);
        assert_eq!(t * p, Point::new(2.0, 3.0, 7.0));
    }

    #[test]
    fn chain() {
        let p = Point::new(1.0, 0.0, 1.0);
        let t = Matrix4x4::IDENTITY.rotate_x(PI / 2.0)
            .scale(5.0, 5.0, 5.0)
            .translate(10.0, 5.0, 7.0);

        assert_eq!(translation(10.0, 5.0, 7.0) * scaling(5.0, 5.0, 5.0) * rotation_x(PI / 2.0) * p,
                   Point::new(15.0, 0.0, 7.0));
        assert_eq!(t * p, Point::new(15.0, 0.0, 7.0));
    }
}
