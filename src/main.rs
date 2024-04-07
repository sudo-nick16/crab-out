use rand::prelude::*;
use raylib::{ffi::Vector2, prelude::*};

static WIDTH: f32 = 640.;
static HEIGHT: f32 = 480.;

enum Direction {
    Left(f32),
    Right(f32),
}

struct Paddle {
    pos: Vector2,
    size: Vector2,
    velocity: f32,
}

struct Ball {
    center: Vector2,
    radius: f32,
    velocity: Vector2,
}

impl Paddle {
    fn new(x: f32, y: f32, w: f32, h: f32, delta: f32) -> Paddle {
        let nx = x - (w / 2.);
        let ny = y - (h / 2.);
        return Paddle {
            pos: Vector2 { x: nx, y: ny },
            size: Vector2 { x: w, y: h },
            velocity: delta,
        };
    }

    fn slide(&mut self, dir: Direction) {
        match dir {
            Direction::Left(frametime) => {
                if self.pos.x - self.velocity > 0. {
                    self.pos.x -= self.velocity * frametime;
                }
            }
            Direction::Right(frametime) => {
                if self.pos.x + self.velocity + self.size.x < WIDTH {
                    self.pos.x += self.velocity * frametime;
                }
            }
        }
    }
}

impl Ball {
    fn new(x: f32, y: f32, r: f32, vx: f32, vy: f32) -> Ball {
        return Ball {
            center: Vector2 { x, y },
            radius: r,
            velocity: Vector2 { x: vx, y: vy },
        };
    }
    fn update(&mut self, frametime: f32) {
        if self.center.x + self.radius >= WIDTH || self.center.x - self.radius <= 0. {
            self.velocity.x *= -1.;
        }
        if self.center.y + self.radius >= HEIGHT || self.center.y - self.radius <= 0. {
            self.velocity.y *= -1.;
        }
        self.center.x += self.velocity.x * frametime;
        self.center.y += self.velocity.y * frametime;
    }
    fn collides_with(&mut self, pos: Vector2, size: Vector2) -> bool {
        if self.center.y + self.radius < pos.y || self.center.y - self.radius > pos.y + size.y {
            return false;
        }
        if self.center.x - self.radius > pos.x + size.x || self.center.x < pos.x {
            return false;
        }
        return true;
    }
}

#[derive(Debug, Clone, Copy)]
struct Obstacle {
    pos: Vector2,
    size: Vector2,
    hit: bool,
}

fn main() {
    let mut paddle = Paddle::new(WIDTH / 2., HEIGHT - 20., 100., 10., 10.);
    let mut ball = Ball::new(WIDTH / 2., HEIGHT - 35., 10., 5., 5.);
    let oheight = 25.0;
    let owidth = 150.0;
    let orowoffset = 100.0;
    let mut rng = rand::thread_rng();
    let mut obstacles: Vec<Obstacle> = Vec::new();
    let orows = 5;

    for i in 0..orows {
        let mut max_x = 0.;
        while max_x < WIDTH {
            let mut xl: f32 = rng.gen();
            xl *= owidth;
            max_x += 1.;
            obstacles.push(Obstacle {
                pos: Vector2 {
                    x: max_x,
                    y: i as f32 + orowoffset + i as f32 * oheight,
                },
                size: Vector2 { x: xl, y: oheight },
                hit: false,
            });
            max_x += xl;
        }
    }

    let (mut rl, thread) = raylib::init()
        .size(WIDTH as i32, HEIGHT as i32)
        .title("Crabout")
        .build();

    rl.set_target_fps(60);

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        d.draw_fps(10, 10);

        if d.is_key_down(KeyboardKey::KEY_LEFT) {
            paddle.slide(Direction::Left(d.get_frame_time() * 60.));
        }

        if d.is_key_down(KeyboardKey::KEY_RIGHT) {
            paddle.slide(Direction::Right(d.get_frame_time() * 60.));
        }

        d.clear_background(Color::BLANK);
        d.draw_rectangle_v(paddle.pos, paddle.size, Color::WHITE);

        d.draw_circle_v(ball.center, ball.radius, Color::RED);
        if ball.collides_with(paddle.pos, paddle.size) {
            ball.velocity.y *= -1.;
        }

        for obs in obstacles.iter_mut() {
            if !obs.hit {
                if ball.collides_with(obs.pos, obs.size) {
                    ball.velocity.y *= -1.;
                    obs.hit = true;
                    continue;
                }
                d.draw_rectangle_v(obs.pos, obs.size, Color::WHITE);
            }
        }

        ball.update(d.get_frame_time() * 60.);
        d.draw_text("Hello, world!", 12, 12, 20, Color::BLACK);
    }
}
