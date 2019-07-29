use raytra::tup::*;

#[derive(Debug)]
struct Proj {
    pos: Point,
    vel: Vector,
}

impl Proj {
    fn new(pos: Point, vel: Vector) -> Proj {
        Proj{pos, vel}
    }
}

#[derive(Debug)]
struct Env {
    grav: Vector,
    wind: Vector,
}

impl Env {
    fn new(grav: Vector, wind: Vector) -> Env {
        Env{grav, wind}
    }
}

fn tick(env: &Env, proj: &Proj) -> Proj {
    let pos: Point = proj.pos + proj.vel;
    let vel: Vector  = proj.vel + env.grav + env.wind;
    Proj::new(pos, vel)
}

fn main() {
    let e = Env::new(Vector::new(0.0, -0.1, 0.0),
                     Vector::new(-0.01, 0.0, 0.0));
    let p0 = Proj::new(Point::new(0.0, 1.0, 0.0),
                      Vector::new(1.0, 1.0, 0.0).norm());

    println!("Environment: {:?}", e);
    println!(" Projectile: {:?}", p0);

    let mut p = tick(&e, &p0);

    while p.pos.y() > 0.0 {
        println!(" Projectile: {:?}", p);
        p = tick(&e, &p);
    }
}
