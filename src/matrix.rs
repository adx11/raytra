use std::cmp::PartialEq;
use std::ops::Mul;

use crate::tup::{Tup};

#[derive(Debug, Clone, Copy)]
pub struct Matrix4x4 {
    elem: [[f32; 4]; 4]
}

#[derive(Debug, Clone, Copy)]
pub struct Matrix3x3 {
    elem: [[f32; 3]; 3]
}

#[derive(Debug, Clone, Copy)]
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

    pub fn submatrix(&self, row: usize, col: usize) -> Matrix3x3 {
        let mut elem = [[0.0f32; 3]; 3];
        let mut row_off = 0;
        let mut col_off = 0;

        for i in 0..4 {
            if i == row {
                row_off = 1;
                continue;
            }
            for j in 0..4 {
                if j == col {
                    col_off = 1;
                    continue;
                }
                elem[i-row_off][j-col_off] = self.at(i,j);
            }
            col_off = 0;
        }

        Matrix3x3{elem}
    }

    pub fn minor(self, row: usize, col: usize) -> f32 {
        self.submatrix(row, col).determinant()
    }

    pub fn cofactor(self, row: usize, col: usize) -> f32 {
        let mut m = self.minor(row, col);

        if (row + col) % 2 == 1 {
            m = -m
        }

        m
    }

    pub fn determinant(self) -> f32 {
        let mut det: f32 = 0.0;

        for i in 0..4 {
            det += self.elem[0][i] * self.cofactor(0, i);
        }

        det
    }

    pub fn inverse(&self) -> Option<Matrix4x4> {
        let det = self.determinant();

        if det == 0.0 {
            return None;
        }

        let mut elem = [[0.0_f32; 4]; 4];

        for row in 0..4 {
            for col in 0..4 {
                elem[col][row] = self.cofactor(row, col) / det;
            }
        }
        Some(Matrix4x4{elem})
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

    pub fn submatrix(&self, row: usize, col: usize) -> Matrix2x2 {
        let mut elem = [[0.0f32; 2]; 2];
        let mut row_off = 0;
        let mut col_off = 0;

        for i in 0..3 {
            if i == row {
                row_off = 1;
                continue;
            }
            for j in 0..3 {
                if j == col {
                    col_off = 1;
                    continue;
                }
                elem[i-row_off][j-col_off] = self.at(i,j);
            }
            col_off = 0;
        }

        Matrix2x2{elem}
    }

    pub fn minor(&self, row: usize, col: usize) -> f32 {
        let s = self.submatrix(row, col);
        s.determinant()
    }

    pub fn cofactor(&self, row: usize, col: usize) -> f32 {
        let mut m = self.minor(row, col);

        if (row + col) % 2 == 1 {
            m = -m
        }

        m
    }

    pub fn determinant(&self) -> f32 {
        let mut det: f32 = 0.0;

        for i in 0..3 {
            det += self.elem[0][i] * self.cofactor(0, i);
        }

        det
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

    pub fn determinant(&self) -> f32 {
        self.at(0,0) * self.at(1,1) - self.at(0,1) * self.at(1,0)
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
        for r in 0..4 {
            for c in 0..4 {
                if abs_diff_ne!(self.elem[r][c], rhs.elem[r][c], epsilon=0.00001) {
                    return false;
                }
            }
        }
        true
    }
}

impl PartialEq for Matrix3x3 {
    fn eq(&self, rhs: &Matrix3x3) -> bool {
        for r in 0..3 {
            for c in 0..3 {
                if abs_diff_ne!(self.elem[r][c], rhs.elem[r][c], epsilon=0.00001) {
                    return false;
                }
            }
        }
        true
    }
}

impl PartialEq for Matrix2x2 {
    fn eq(&self, rhs: &Matrix2x2) -> bool {
        for r in 0..2 {
            for c in 0..2 {
                if abs_diff_ne!(self.elem[r][c], rhs.elem[r][c], epsilon=0.00001) {
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

    #[test]
    fn determinant() {
        let a = Matrix2x2::new(1.0, 5.0,
                              -3.0, 2.0);
        assert_eq!(a.determinant(), 17.0);
    }

    #[test]
    fn submatrix() {
        let a = Matrix3x3::new(1.0, 5.0, 0.0,
                               -3.0, 2.0, 7.0,
                               0.0, 6.0, -3.0);
        assert_eq!(a.submatrix(0, 2),
                   Matrix2x2::new(-3.0, 2.0,
                                  0.0, 6.0));

        let a = Matrix4x4::new(-6.0, 1.0, 1.0, 6.0,
                               -8.0, 5.0, 8.0, 6.0,
                               -1.0, 0.0, 8.0, 2.0,
                               -7.0, 1.0, -1.0, 1.0);
        assert_eq!(a.submatrix(2, 1),
                   Matrix3x3::new(-6.0, 1.0, 6.0,
                                  -8.0, 8.0, 6.0,
                                  -7.0, -1.0, 1.0));
    }

    #[test]
    fn minor() {
        let a = Matrix3x3::new(3.0, 5.0, 0.0,
                               2.0, -1.0, -7.0,
                               6.0, -1.0, 5.0);
        let b = a.submatrix(1, 0);

        assert_eq!(b.determinant(), 25.0);
        assert_eq!(a.minor(1, 0), 25.0);
    }

    #[test]
    fn cofactor() {
        let a = Matrix3x3::new(3.0, 5.0, 0.0,
                               2.0, -1.0, -7.0,
                               6.0, -1.0, 5.0);

        assert_eq!(a.cofactor(0,0), -12.0);
        assert_eq!(a.cofactor(1, 0), -25.0);
    }

    #[test]
    fn determinant3x3() {
        let a = Matrix3x3::new(1.0, 2.0, 6.0,
                               -5.0, 8.0, -4.0,
                               2.0, 6.0, 4.0);

        assert_eq!(a.cofactor(0, 0), 56.0);
        assert_eq!(a.cofactor(0, 1), 12.0);
        assert_eq!(a.cofactor(0, 2), -46.0);
        assert_eq!(a.determinant(), -196.0);
    }

    #[test]
    fn determinant4x4() {
        let a = Matrix4x4::new(-2.0, -8.0, 3.0, 5.0,
                               -3.0, 1.0, 7.0, 3.0,
                               1.0, 2.0, -9.0, 6.0,
                               -6.0, 7.0, 7.0, -9.0);

        assert_eq!(a.cofactor(0, 0), 690.0);
        assert_eq!(a.cofactor(0, 1), 447.0);
        assert_eq!(a.cofactor(0, 2), 210.0);
        assert_eq!(a.cofactor(0, 3), 51.0);
        assert_eq!(a.determinant(), -4071.0);
    }

    #[test]
    fn invertable() {
        let a = Matrix4x4::new(6.0, 4.0, 4.0, 4.0,
                               5.0, 5.0, 7.0, 6.0,
                               4.0, -9.0, 3.0, -7.0,
                               9.0, 1.0, 7.0, -6.0);
        assert_eq!(a.determinant(), -2120.0);
        assert_ne!(a.inverse(), None);

        let a = Matrix4x4::new(-4.0, 2.0, -2.0, -3.0,
                               9.0, 6.0, 2.0, 6.0,
                               0.0, -5.0, 1.0, -5.0,
                               0.0, 0.0, 0.0, 0.0);
        assert_eq!(a.determinant(), 0.0);
        assert_eq!(a.inverse(), None);
    }

    #[test]
    fn inverse() {
        let a = Matrix4x4::new(-5.0, 2.0, 6.0, -8.0,
                               1.0, -5.0, 1.0, 8.0,
                               7.0, 7.0, -6.0, -7.0,
                               1.0, -3.0, 7.0, 4.0);

        assert_eq!(a.inverse().unwrap(),
                   Matrix4x4::new(0.21805, 0.45113, 0.24060, -0.04511,
                                  -0.80827, -1.45677, -0.44361, 0.52068,
                                  -0.07895, -0.22368, -0.05263, 0.19737,
                                  -0.52256, -0.81391, -0.30075, 0.30639));

        let b = Matrix4x4::new(8.0, 2.0, 2.0, 2.0,
                               3.0, -1.0, 7.0, 0.0,
                               7.0, 0.0, 5.0, 4.0,
                               6.0, -2.0, 0.0, 5.0);
        let c = a * b;
        assert_ne!(b.inverse(), None);
        assert_eq!(c * b.inverse().unwrap(), a);
    }
}
