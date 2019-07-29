use std::cmp::PartialEq;
use std::ops::{Add, Sub, Mul};

#[derive(Debug, Clone, Copy)]
pub struct Color {
    red: f32,
    green: f32,
    blue: f32,
}

impl Color {
    pub fn new(red: f32, green: f32, blue: f32) -> Color {
        Color{red, green, blue}
    }
}

impl Add for Color {
    type Output = Color;

    fn add(self, other: Self) -> Self {
        Color::new(self.red + other.red,
                   self.green + other.green,
                   self.blue + other.blue)
    }
}

impl Sub for Color {
    type Output = Color;

    fn sub(self, other: Self) -> Self {
        Color::new(self.red - other.red,
                   self.green - other.green,
                   self.blue - other.blue)
    }
}

impl Mul for Color {
    type Output = Color;

    fn mul(self, other: Self) -> Self {
        Color::new(self.red * other.red,
                   self.green * other.green,
                   self.blue * other.blue)
    }
}

impl Mul<f32> for Color {
    type Output = Color;

    fn mul(self, other: f32) -> Self {
        Color::new(self.red * other,
                   self.green * other,
                   self.blue * other)
    }
}

impl PartialEq for Color {
    fn eq(&self, other: &Color) -> bool {
        abs_diff_eq!(self.red, other.red) &&
            abs_diff_eq!(self.green, other.green) &&
            abs_diff_eq!(self.blue, other.blue)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add() {
        let a = Color::new(0.9, 0.6, 0.75);
        let b = Color::new(0.7, 0.1, 0.25);
        assert_eq!(a + b, Color::new(1.6, 0.7, 1.0));
    }

    #[test]
    fn sub() {
        let a = Color::new(0.9, 0.6, 0.75);
        let b = Color::new(0.7, 0.1, 0.25);
        assert_eq!(a - b, Color::new(0.2, 0.5, 0.5));
    }

    #[test]
    fn mul() {
        let a = Color::new(0.2, 0.3, 0.4);
        assert_eq!(a * 2.0, Color::new(0.4, 0.6, 0.8));

        let a = Color::new(1.0, 0.2, 0.4);
        let b = Color::new(0.9, 1.0, 0.1);
        assert_eq!(a * b, Color::new(0.9, 0.2, 0.04));

    }
}
