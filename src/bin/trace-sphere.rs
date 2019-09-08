use raytra::canvas::Canvas;
use raytra::tup::Point;
use raytra::sphere::Sphere;
use raytra::rays::Ray;
use raytra::color;

use std::fs;

fn main() {
    const WALL_SIZE: usize = 7;
    const CANVAS_SIZE: usize = 100;

    let pixel_size = WALL_SIZE as f32 / CANVAS_SIZE as f32;
    let half = WALL_SIZE as f32 / 2.0;

    let light = Point::new(0.0, 0.0, -5.0);
    let wall_z = 10.0;

    let mut canvas = Canvas::new(CANVAS_SIZE, CANVAS_SIZE);
    let shape = Sphere::unit();

    for y in 0..CANVAS_SIZE {
        let world_y = half - pixel_size * (y as f32);

        for x in 0..CANVAS_SIZE {
            let world_x = -half + pixel_size * (x as f32);

            let pos = Point::new(world_x as f32, world_y as f32, wall_z);

            let r = Ray::new(light, (pos - light).norm());
            let xs = shape.intersect(r);

            if let Some(_) = xs.hit() {
                canvas.write_at(x, y, color::RED);
            }
        }
    }

    fs::write("sphere.ppm", canvas.to_ppm());
}
