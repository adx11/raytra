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
}
