use specs::{
    Builder, Component, DenseVecStorage, DispatcherBuilder, Join, ReadStorage, System, World,
    WorldExt, WriteStorage,
};

struct HelloWorld;

impl<'a> System<'a> for HelloWorld {
    type SystemData = (ReadStorage<'a, Position>, ReadStorage<'a, Velocity>);

    fn run(&mut self, (pos, vel): Self::SystemData) {
        let mut energy = 0;
        for (pos, vel) in (&pos, &vel).join() {
            println!("Hello pos={:?}, vel={:?}", pos, vel);
            let pot = pos.x.abs() + pos.y.abs() + pos.z.abs();
            let kin = vel.x.abs() + vel.y.abs() + vel.z.abs();
            println!("Energy: {} * {} = {}", pot, kin, pot*kin);
            energy += pot * kin;
        }
        println!("Energy: {}", energy);
    }
}

/// Normalizes to 1, 0 or -1
fn norm(x: i64) -> i64 {
    if x == 0 {
        0
    } else if x > 0 {
        1
    } else {
        -1
    }
}

struct UpdateVel;
impl<'a> System<'a> for UpdateVel {
    type SystemData = (WriteStorage<'a, Velocity>, ReadStorage<'a, Position>);

    fn run(&mut self, (mut vel, pos): Self::SystemData) {
        for (vel, this) in (&mut vel, &pos).join() {
            for other in (pos).join() {
                vel.x += norm(other.x - this.x);
                vel.y += norm(other.y - this.y);
                vel.z += norm(other.z - this.z);
            }
        }
    }
}

struct UpdatePos;

impl<'a> System<'a> for UpdatePos {
    type SystemData = (ReadStorage<'a, Velocity>, WriteStorage<'a, Position>);

    fn run(&mut self, (vel, mut pos): Self::SystemData) {
        for (vel, pos) in (&vel, &mut pos).join() {
            pos.x += vel.x;
            pos.y += vel.y;
            pos.z += vel.z;
        }
    }
}

#[derive(Component, Debug, PartialEq)]
struct Position {
    x: i64,
    y: i64,
    z: i64,
}

#[derive(Component, Debug)]
struct Velocity {
    x: i64,
    y: i64,
    z: i64,
}

fn create_moon(world: &mut World, position: (i64, i64, i64)) {
    world
        .create_entity()
        .with(Position {
            x: position.0,
            y: position.1,
            z: position.2,
        })
        .with(Velocity { x: 0, y: 0, z: 0 })
        .build();
}

fn create_input(world: &mut World) {
    create_moon(world, (15, -2, -6));
    create_moon(world, (-5, -4, -11));
    create_moon(world, (0, -6, 0));
    create_moon(world, (5, 9, 6));
}

fn create_input_ex_a(world: &mut World) {
    create_moon(world, (-1, 0, 2));
    create_moon(world, (2, -10, -7));
    create_moon(world, (4, -8, 8));
    create_moon(world, (3, 5, -1));
}

fn create_input_ex_b(world: &mut World) {
    create_moon(world, (-8, -10, 0));
    create_moon(world, (5, 5, 10));
    create_moon(world, (2, -7, 3));
    create_moon(world, (9, -8, -3));
}

type Vector = Vec<(i64, i64)>;

fn run(xs: Vector) -> Vector {
    let mut out = xs.clone();
    for (i, x) in xs.iter().enumerate() {
        let mut new_x = x.1;
        for y in &xs {
            if x == y {
                continue;
            }
            new_x += norm(y.0 - x.0);
        }
        out[i] = (x.0, new_x);
    }
    let size = out.len();
    for i in 0..size {
        out[i].0 += out[i].1;
    }
    out
}

fn period(xs: (i64, i64, i64, i64)) -> i64 {
    let mut v = vec![];
    v.push((xs.0, 0));
    v.push((xs.1, 0));
    v.push((xs.2, 0));
    v.push((xs.3, 0));
    let first = v.clone();
    let mut i = 0;
    loop {
        i += 1;
        println!("V={:?}", v);
        v = run(v);
        if v == first {
            return i;
        }
    }
}

use std::cmp::{max, min};
 
fn gcd(a: i64, b: i64) -> i64 {
    match ((a, b), (a & 1, b & 1)) {
        ((x, y), _) if x == y => y,
        ((0, x), _) | ((x, 0), _) => x,
        ((x, y), (0, 1)) | ((y, x), (1, 0)) => gcd(x >> 1, y),
        ((x, y), (0, 0)) => gcd(x >> 1, y >> 1) << 1,
        ((x, y), (1, 1)) => {
            let (x, y) = (min(x, y), max(x, y));
            gcd((y - x) >> 1, x)
        }
        _ => unreachable!(),
    }
}
 
fn lcm(a: i64, b: i64) -> i64 {
    a * b / gcd(a, b)
}

fn main() {
    let mut world = World::new();
    world.register::<Position>();
    world.register::<Velocity>();
    create_input(&mut world);

    let mut dispatcher = DispatcherBuilder::new()
        .with(UpdateVel, "update_vel", &[])
        .with(UpdatePos, "update_pos", &["update_vel"])
        .with(HelloWorld, "hello_world", &["update_pos"])
        .build();

    for i in 0..1000 {
        println!("Running");
        if i % 10 == 9 {
            println!("After {} steps:", i + 1);
        }
        dispatcher.dispatch(&mut world);
        world.maintain();
    }

    let xs = (15, -5, 0, 5);
    let ys = (-2, -4, -6, 9);
    let zs = (-6, -11, 0, 6);
    let p1 = period(xs);
    let p2 = period(ys);
    let p3 = period(zs);
    println!("Period of {:?} is {}", xs, p1);
    println!("Period of {:?} is {}", ys, p2);
    println!("Period of {:?} is {}", zs, p3);
    let x = lcm(p1, p2);
    println!("lcm(1,2) = {}", x);
    let result = lcm(x, p3);
    println!("lcm(x, 3) = {}", result);

    println!("Hello, world!");
}
