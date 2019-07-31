use std::cmp::PartialEq;
use std::ops::Mul;

use crate::tup::{Tup};

#[derive(Debug, Clone, Copy)]
pub struct Matrix4x4 {
    elem: [[f32; 4]; 4]
}

pub struct Matrix3x3 {
    elem: [[f32; 3]; 3]
}

pub struct Matrix2x2 {
    elem: [[f32; 2]; 2]
}

impl Matrix4x4 {
    pub fn new(e11: f32, e12: f32, e13: f32, e14:f32,
               e21: f32, e22: f32, e23: f32, e24:f32,
               e31: f32, e32: f32, e33: f32, e34:f32,
               e41: f32, e42: f32, e43: f32, e44:f32)
               -> Matrix4x4 {
        Matrix4x4{
            elem: [[e11, e12, e13, e14],
                   [e21, e22, e23, e24],
                   [e31, e32, e33, e34],
                   [e41, e42, e43, e44]]
        }
    }

    const IDENTITY: Matrix4x4 = Matrix4x4{elem: [[1.0, 0.0, 0.0, 0.0],
                                                 [0.0, 1.0, 0.0, 0.0],
                                                 [0.0, 0.0, 1.0, 0.0],
                                                 [0.0, 0.0, 0.0, 1.0]]};

    fn row(&self, row: usize) -> Tup {
        Tup::new(self.elem[row][0],
                 self.elem[row][1],
                 self.elem[row][2],
                 self.elem[row][3])

    }

    fn col(&self, col: usize) -> Tup {
        Tup::new(self.elem[0][col],
                 self.elem[1][col],
                 self.elem[2][col],
                 self.elem[3][col])
    }

    pub fn transpose(&self) -> Matrix4x4 {
        let mut elem = [[0.0f32; 4]; 4];

        for i in 0..4 {
            for j in 0..4 {
                elem[j][i] = self.elem[i][j];
            }
        }

        Matrix4x4{elem}
    }

    pub fn at(&self, a: usize, b: usize) -> f32 {
        self.elem[a][b]
    }

}

impl Matrix3x3 {
    pub fn new(e11: f32, e12: f32, e13: f32,
               e21: f32, e22: f32, e23: f32,
               e31: f32, e32: f32, e33: f32)
               -> Matrix3x3 {
        Matrix3x3{
            elem: [[e11, e12, e13],
                   [e21, e22, e23],
                   [e31, e32, e33]]
        }
    }

    pub fn at(&self, a: usize, b: usize) -> f32 {
        self.elem[a][b]
    }

}

impl Matrix2x2 {
    pub fn new(e11: f32, e12: f32,
               e21: f32, e22: f32)
               -> Matrix2x2 {
        Matrix2x2{
            elem: [[e11, e12],
                   [e21, e22]]
        }
    }

    pub fn at(&self, a: usize, b: usize) -> f32 {
        self.elem[a][b]
    }
}

impl Mul for Matrix4x4 {
    type Output = Matrix4x4;

    fn mul(self, rhs: Matrix4x4) -> Matrix4x4 {
        let rows = [self.row(0), self.row(1), self.row(2), self.row(3)];
        let cols = [rhs.col(0), rhs.col(1), rhs.col(2), rhs.col(3)];

        let mut elem = [[0.0f32; 4]; 4];

        for i in 0..4 {
            for j in 0..4 {
                elem[i][j] = rows[i].dot(cols[j]);
            }
        }

        Matrix4x4{elem}
    }
}

impl Mul<Tup> for Matrix4x4 {
    type Output = Tup;

    fn mul(self, rhs: Tup) -> Tup {
        Tup::new(self.row(0).dot(rhs),
                 self.row(1).dot(rhs),
                 self.row(2).dot(rhs),
                 self.row(3).dot(rhs))
    }
}

impl PartialEq for Matrix4x4 {
    fn eq(&self, rhs: &Matrix4x4) -> bool {
        for r in 0..3 {
            for c in 0..3 {
                if !abs_diff_eq!(self.elem[r][c], rhs.elem[r][c]) {
                    return false;
                }
            }
        }
        true
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let m = Matrix4x4::new(1.0,   2.0,  3.0,  4.0,
                               5.5,   6.5,  7.5,  8.5,
                               9.0,  10.0, 11.0, 12.0,
                               13.5, 14.5, 15.5, 16.5);
        assert_eq!(m.elem[0][0], 1.0 );
        assert_eq!(m.elem[0][3], 4.0 );
        assert_eq!(m.elem[1][0], 5.5 );
        assert_eq!(m.elem[1][2], 7.5 );
        assert_eq!(m.elem[2][2], 11.0);
        assert_eq!(m.elem[3][0], 13.5);
        assert_eq!(m.elem[3][2], 15.5);

        let m = Matrix2x2::new(-3.0,  5.0,
                                1.0, -2.0);
        assert_eq!(m.elem[0][0], -3.0);
        assert_eq!(m.elem[0][1],  5.0);
        assert_eq!(m.elem[1][0],  1.0);
        assert_eq!(m.elem[1][1], -2.0);

        let m = Matrix3x3::new(-3.0,  5.0,  0.0,
                               1.0, -2.0, -7.0,
                               0.0, 1.0, 1.0);
        assert_eq!(m.elem[0][0], -3.0);
        assert_eq!(m.elem[1][1], -2.0);
        assert_eq!(m.elem[2][2],  1.0);
    }

    #[test]
    fn eq() {
        let a = Matrix4x4::new(1.0, 2.0, 3.0, 4.0,
                               5.0, 6.0, 7.0, 8.0,
                               9.0, 8.0, 7.0, 6.0,
                               5.0, 4.0, 3.0, 2.0);
        let b = Matrix4x4::new(1.0, 2.0, 3.0, 4.0,
                               5.0, 6.0, 7.0, 8.0,
                               9.0, 8.0, 7.0, 6.0,
                               5.0, 4.0, 3.0, 2.0);
        assert_eq!(a, b);

        let a = Matrix4x4::new(1.0, 2.0, 3.0, 4.0,
                               5.0, 6.0, 7.0, 8.0,
                               9.0, 8.0, 7.0, 6.0,
                               5.0, 4.0, 3.0, 2.0);
        let b = Matrix4x4::new(5.0, 6.0, 7.0, 8.0,
                               9.0, 8.0, 7.0, 6.0,
                               5.0, 4.0, 3.0, 2.0,
                               1.0, 2.0, 3.0, 4.0);
        assert_ne!(a, b);

    }

    #[test]
    fn mul() {
        let a = Matrix4x4::new(1.0, 2.0, 3.0, 4.0,
                               5.0, 6.0, 7.0, 8.0,
                               9.0, 8.0, 7.0, 6.0,
                               5.0, 4.0, 3.0, 2.0);
        let b = Matrix4x4::new(-2.0, 1.0, 2.0, 3.0,
                               3.0, 2.0, 1.0, -1.0,
                               4.0, 3.0, 6.0, 5.0,
                               1.0, 2.0, 7.0, 8.0);

        assert_eq!(a * b,
                   Matrix4x4::new(20.0, 22.0, 50.0, 48.0,
                                  44.0, 54.0, 114.0, 108.0,
                                  40.0, 58.0, 110.0, 102.0,
                                  16.0, 26.0, 46.0, 42.0));
    }

    #[test]
    fn mul_tup() {
        let a = Matrix4x4::new(1.0, 2.0, 3.0, 4.0,
                               2.0, 4.0, 4.0, 2.0,
                               8.0, 6.0, 4.0, 1.0,
                               0.0, 0.0, 0.0, 1.0);
        let b = Tup::new(1.0, 2.0, 3.0, 1.0);

        assert_eq!(a * b, Tup::new(18.0, 24.0, 33.0, 1.0));
    }

    #[test]
    fn mul_identity() {
        let a = Matrix4x4::new(0.0, 1.0, 2.0, 4.0,
                               1.0, 2.0, 4.0, 8.0,
                               2.0, 4.0, 8.0, 16.0,
                               4.0, 8.0, 16.0, 32.0);

        assert_eq!(a * Matrix4x4::IDENTITY,
                   Matrix4x4::new(0.0, 1.0, 2.0, 4.0,
                                  1.0, 2.0, 4.0, 8.0,
                                  2.0, 4.0, 8.0, 16.0,
                                  4.0, 8.0, 16.0, 32.0));
    }

    #[test]
    fn transpose() {
        let a = Matrix4x4::new(0.0, 9.0, 3.0, 0.0,
                               9.0, 8.0, 0.0, 8.0,
                               1.0, 8.0, 5.0, 3.0,
                               0.0, 0.0, 5.0, 8.0);
        assert_eq!(a.transpose(),
                   Matrix4x4::new(0.0, 9.0, 1.0, 0.0,
                                  9.0, 8.0, 8.0, 0.0,
                                  3.0, 0.0, 5.0, 5.0,
                                  0.0, 8.0, 3.0, 8.0));
        assert_eq!(Matrix4x4::IDENTITY.transpose(),
                   Matrix4x4::IDENTITY);
    }
}
