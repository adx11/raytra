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

    fn ppm_header(&self) -> String {
        format!("P3\n{} {}\n255\n", self.width, self.height)
    }

    fn ppm_pixel_data(&self) -> String {
        let mut data = Vec::new();

        // TODO: iterators?
        for row in &self.pixels {
            for color in row {
                let p = color.to_pixel();
                data.push(format!("{} {} {}", p[0], p[1], p[2]));
            }
        }

        data.join(" ")
    }

    pub fn to_ppm(&self) -> String {
        format!("{}\n{}\n", self.ppm_header(), self.ppm_pixel_data())
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

    #[test]
    fn ppm_header() {
        let c = Canvas::new(5, 3);
        assert_eq!(c.ppm_header(), String::from("P3\n5 3\n255\n"));
    }

    #[test]
    fn ppm_pixel_data() {
        let mut c = Canvas::new(5, 3);
        c.write_at(0, 0, Color::new(1.5, 0.0, 0.0));
        c.write_at(2, 1, Color::new(0.0, 0.5, 0.0));
        c.write_at(4, 2, Color::new(-0.5, 0.0, 1.0));

        let expected = String::from(
            "255 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 128 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 255");
        assert_eq!(c.ppm_pixel_data(), expected);
    }
}
