use crate::color::Color;

struct Canvas {
    width: usize,
    height: usize,
    pixels: Vec<Vec<Color>>,
}

impl Canvas {
    fn new(width: usize, height: usize) -> Canvas {
        Canvas {width,
                height,
                pixels: vec![vec![Color::new(0.0, 0.0, 0.0); height]; width]}
    }

    fn at(&self, x: usize, y: usize) -> Color {
        self.pixels[x][y]
    }

    fn write_at(&mut self, x: usize, y: usize, color: Color) {
        self.pixels[x][y] = color;
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let w = 10;
        let h = 20;
        let c = Canvas::new(w, h);

        assert_eq!(c.width, w);
        assert_eq!(c.height, h);

        let black = Color::new(0.0, 0.0, 0.0);

        for i in 0..w {
            for j in 0..h {
                assert_eq!(c.pixels[i][j], black)
            }
        }
    }

    #[test]
    fn write_at() {
        let mut c = Canvas::new(10, 20);
        let red = Color::new(1.0, 0.0, 0.0);

        c.write_at(2, 3, red);
        assert_eq!(c.at(2,3), red);
    }
}
