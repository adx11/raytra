use raytra::canvas::Canvas;
use raytra::tup::Point;
use raytra::matrix::Matrix4x4;
use raytra::color;

use std::f32::consts::PI;

fn color(c:&mut Canvas, x: usize, y: usize) {
    c.write_at(x-1, y+1, color::RED);
    c.write_at(x, y+1, color::RED);
    c.write_at(x+1, y+1, color::RED);

    c.write_at(x-1, y, color::RED);
    c.write_at(x, y, color::RED);
    c.write_at(x+1, y, color::RED);

    c.write_at(x-1, y-1, color::RED);
    c.write_at(x, y-1, color::RED);
    c.write_at(x+1, y-1, color::RED);
}

fn main() {
    const CANVAS_SIZE: usize = 400;

    let mut c = Canvas::new(CANVAS_SIZE, CANVAS_SIZE);

    let rot = Matrix4x4::identity().rotate_z(PI / 6.0);
    let t = Matrix4x4::identity().scale(150.0, 150.0, 0.0)
        .translate(200.0, 200.0, 0.0);

    let mut p = Point::new(0.0, 1.0, 0.0);

    for _i in 0..12 {
        p = rot * p;

        let x = (t * p).x() as usize;
        let y = (t * p).y() as usize;
        color(&mut c, x, y);
    }

    println!("{}", c.to_ppm());
}
