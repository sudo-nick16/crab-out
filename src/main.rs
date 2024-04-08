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
    fn new(x: f32, y: f32, w: f32, h: f32, v: f32) -> Paddle {
        let nx = x - (w / 2.);
        let ny = y - (h / 2.);
        return Paddle {
            pos: Vector2 { x: nx, y: ny },
            size: Vector2 { x: w, y: h },
            velocity: v,
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
        let mut distx = self.center.x - pos.x - size.x / 2.;
        distx = distx.abs();
        let mut disty = self.center.y - pos.y - size.y / 2.;
        disty = disty.abs();

        if distx > (size.x / 2. + self.radius) {
            return false;
        }

        if disty > (size.y / 2. + self.radius) {
            return false;
        }

        if distx <= (size.x / 2.) {
            return true;
        }

        if disty <= (size.y / 2.) {
            return true;
        }

        let dx = distx - size.x / 2.;
        let dy = disty - size.y / 2.;
        return dx * dx + dy * dy <= self.radius * self.radius;
    }
}

#[derive(Debug, Clone, Copy)]
struct Obstacle {
    pos: Vector2,
    size: Vector2,
    hit: bool,
}

fn generate_obstacles(obstacles: &mut Vec<Obstacle>) {
    let mut rng = rand::thread_rng();
    let owidth = 150.0;
    let oheight = 25.0;
    let orowoffset = 50.0;
    let orows = 5;
    let minw = 70.;
    for i in 0..orows {
        let mut max_x = 0.;
        while max_x < WIDTH {
            let mut xl: f32 = rng.gen();
            xl *= owidth;
            if xl < minw {
                xl = minw;
            }
            max_x += 2.;
            if max_x + xl + minw > WIDTH {
                xl = WIDTH - max_x;
            }
            obstacles.push(Obstacle {
                pos: Vector2 {
                    x: max_x,
                    y: i as f32 * 2. + orowoffset + i as f32 * oheight,
                },
                size: Vector2 { x: xl, y: oheight },
                hit: false,
            });
            max_x += xl;
        }
    }
}

fn main() {
    let ground_offset = 100.;
    let mut paddle = Paddle::new(WIDTH / 2., HEIGHT - ground_offset, 100., 10., 12.);
    let mut ball = Ball::new(WIDTH / 2., HEIGHT - 35., 10., 5., 5.);
    let mut obstacles: Vec<Obstacle> = Vec::new();

    generate_obstacles(&mut obstacles);

    let (mut rl, thread) = raylib::init()
        .size(WIDTH as i32, HEIGHT as i32)
        .title("Crabout")
        .build();

    rl.set_target_fps(60);

    let brick_texture = rl
        .load_texture(&thread, "assets/brick_img.png")
        .expect("Could not load brick texture");

    let bg_texture = rl
        .load_texture(&thread, "assets/bg.png")
        .expect("Could not load background texture");

    let ball_texture = rl
        .load_texture(&thread, "assets/ball_20.png")
        .expect("Could not load ball texture");

    let mut frame_rec = Rectangle {
        x: 0.,
        y: 0.,
        width: WIDTH,
        height: HEIGHT,
    };

    let mut score = 0;
    let per_hit_gain = 10;

    let mut pause = false;

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        d.draw_fps(10, 10);
        d.clear_background(Color::BLANK);

        frame_rec.x += 1.;
        frame_rec.x %= bg_texture.width() as f32;

        d.draw_texture_rec(
            &bg_texture,
            frame_rec,
            Vector2 { x: 0., y: 0. },
            Color::DARKGRAY,
        );

        d.draw_text(score.to_string().as_str(), 10, 10, 20, Color::WHITE);

        if d.is_key_pressed(KeyboardKey::KEY_SPACE) {
            pause = !pause;
        }

        if d.is_key_down(KeyboardKey::KEY_LEFT) {
            paddle.slide(Direction::Left(d.get_frame_time() * 60.));
        }

        if d.is_key_down(KeyboardKey::KEY_RIGHT) {
            paddle.slide(Direction::Right(d.get_frame_time() * 60.));
        }

        d.draw_rectangle_v(paddle.pos, paddle.size, Color::WHITE);

        d.draw_texture_pro(
            &ball_texture,
            Rectangle {
                x: 0.,
                y: 0.,
                width: ball_texture.width() as f32,
                height: ball_texture.height() as f32,
            },
            Rectangle {
                x: ball.center.x - ball.radius,
                y: ball.center.y - ball.radius,
                width: ball.radius * 2.,
                height: ball.radius * 2.,
            },
            Vector2 { x: 0., y: 0. },
            0.,
            Color::YELLOW,
        );
        // d.draw_circle_v(ball.center, ball.radius, Color::YELLOW);

        for obs in obstacles.iter_mut() {
            if !obs.hit {
                if ball.collides_with(obs.pos, obs.size) {
                    score += per_hit_gain;
                    ball.velocity.y *= -1.;
                    obs.hit = true;
                    break;
                }
                // d.draw_rectangle_v(obs.pos, obs.size, Color::RED);
            }
        }

        for obs in obstacles.iter() {
            if !obs.hit {
                d.draw_texture_pro(
                    &brick_texture,
                    Rectangle {
                        x: 0.,
                        y: 0.,
                        width: brick_texture.width() as f32,
                        height: brick_texture.height() as f32,
                    },
                    Rectangle {
                        x: obs.pos.x,
                        y: obs.pos.y,
                        width: obs.size.x,
                        height: obs.size.y,
                    },
                    Vector2 { x: 0., y: 0. },
                    0.,
                    Color::from_hex("FF5733").expect("Could not get color from hex"),
                );
                // d.draw_rectangle_rounded(
                //     Rectangle {
                //         x: obs.pos.x,
                //         y: obs.pos.y,
                //         width: obs.size.x,
                //         height: obs.size.y,
                //     },
                //     0.3,
                //     15,
                //     Color::from_hex("FF5733").expect("Could not get color from hex"),
                // );
            }
        }

        if ball.collides_with(paddle.pos, paddle.size) {
            ball.velocity.y *= -1.;
        }

        if !pause {
            ball.update(d.get_frame_time() * 60.);
        }
    }
}
