use coffee::graphics::{Color, Frame, Window, WindowSettings};
use coffee::load::Task;
use coffee::{Game, Result, Timer};
use coffee::graphics::{Batch, Rectangle, Image, Quad, Point};
use std::path::Path;
use std::time::Instant;
use rand::Rng;
use shapekit::{vector::Vector, world::{ShapeHandle, WorldHandle}};
use rayon::prelude::*;

const SPEED: f32 = 50.0;

fn main() -> Result<()> {
    MyGame::run(WindowSettings {
        title: String::from("A caffeinated game"),
        size: (1280, 1024),
        resizable: true,
        fullscreen: false,
    })
}

struct MyGame {
    timer: Instant,
    world: WorldHandle,
    bouncers: Vec<Bouncer>,
    batch: Batch,
}

impl Game for MyGame {
    type Input = (); // No input data
    type LoadingScreen = (); // No loading screen

    fn load(_window: &Window) -> Task<MyGame> {
        Task::using_gpu(|gpu| Image::new(gpu, Path::new("white_square.png"))).map(|palette| {
                let mut rng = rand::thread_rng();
                let w = WorldHandle::new();
                let mut bs = Vec::new();
                for _ in 0..1000 {
                    let x = rng.gen_range(0.0, 1280.0);
                    let y = rng.gen_range(0.0, 1024.0);
                    bs.push(Bouncer{
                        hitbox: w.add_shape(vec![(0.0, 0.0), (10.0, 0.0), (10.0, 10.0), (0.0, 10.0)], (x, y)),
                        velocity: Vector::new(SPEED, SPEED),
                    });
                }
                MyGame{ timer: Instant::now(), batch: Batch::new(palette), world: w, bouncers: bs }
            }
        )
    }

    fn update(&mut self, window: &Window) {
        let delta = self.timer.elapsed().as_micros() as f32 / 1000000.0;
        self.timer = Instant::now();
        let width = window.width();
        let height = window.height();
        self.bouncers.par_iter_mut().for_each(|me|{
            let movement = me.velocity * delta;
            for collision in me.hitbox.collisions() {
                let res = collision.resolution;
                if res.x > 0.0 {
                    me.velocity.x = SPEED;
                } else if res.x < 0.0 {
                    me.velocity.x = -SPEED;
                }
                if res.y > 0.0 {
                    me.velocity.y = SPEED;
                } else if res.y < 0.0 {
                    me.velocity.y = -SPEED;
                }
            }
            let (x, y) = me.hitbox.center().to_tuple();
            if x < 0.0 {
                me.velocity.x = SPEED;
            } else if x > width {
                me.velocity.x = -SPEED;
            }
            if y < 0.0 {
                me.velocity.y = SPEED;
            } else if y > height {
                me.velocity.y = -SPEED;
            }
            me.hitbox.move_by(movement);
        });
    }

    fn draw(&mut self, frame: &mut Frame, _timer: &Timer) {
        // Clear the current frame
        frame.clear(Color::BLACK);

        // Draw your game here. Check out the `graphics` module!
        self.batch.clear();
        for me in self.bouncers.iter() {
            let (x, y) = me.hitbox.center().to_tuple();
            self.batch.add(rect_on_pt(x, y));
        }
        self.batch.draw(&mut frame.as_target());
    }
}

fn rect_on_pt(x: f32, y: f32) -> Quad {
    Quad {
        source: Rectangle {
            x: 0.0,
            y: 0.0,
            width: 1.0,
            height: 1.0,
        },
        position: Point::new(x - 10.0, y - 10.0),
        size: (10.0, 10.0),
    }
}

struct Bouncer {
    hitbox: ShapeHandle,
    velocity: Vector,
}