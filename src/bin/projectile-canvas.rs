use raytra::tup::*;
use raytra::canvas::Canvas;
use raytra::color::Color;

use std::fs;

#[derive(Debug)]
struct Projectile {
    pos: Point,
    vel: Vector,
}

#[derive(Debug)]
struct Env {
    g: Vector,
    w: Vector,
}

fn tick(env: &Env, proj: &Projectile) -> Projectile {
    Projectile {
        pos: proj.pos + proj.vel,
        vel: proj.vel + env.g + env.w
    }
}

fn main() {
    let e = Env {
        g: Vector::new(0.0, -0.1, 0.0),
        w: Vector::new(-0.01, 0.0, 0.0)
    };

    let mut p = Projectile {
        pos: Point::new(0.0, 1.0, 0.0),
        vel: Vector::new(1.0, 1.8, 0.0).norm() * 11.25
    };


    let mut canvas = Canvas::new(900, 550);
    let red = Color::new(1.0, 0.0, 0.0);

    while p.pos.y() > 0.0 {
        let x = p.pos.x().round() as usize;
        let y = p.pos.y().round() as usize;
        canvas.write_at(x, 549-y, red);
        p = tick(&e, &p);
    }

    fs::write("proj.ppm",
              canvas.to_ppm().as_bytes()).expect("Could not write PPM file");
}
