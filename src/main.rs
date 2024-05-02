use vector::Vector;
use speedy2d::color::Color;
use speedy2d::window::{WindowHandler, WindowHelper};
use speedy2d::{Graphics2D, Window};

fn main() {
    let window = Window::new_centered("Title", (800, 480)).unwrap();
    let win = MyWindowHandler{
        p: Pendulum::new(400.0, 0.0, 200.0),
        p2: Pendulum::new(400.0, 0.0, 400.0),
    };

    window.run_loop(win);
}

struct MyWindowHandler {
    p: Pendulum,
    p2: Pendulum,
}

impl WindowHandler for MyWindowHandler {
    fn on_draw(
            &mut self,
            helper: &mut WindowHelper<()>,
            graphics: &mut Graphics2D
        ) {
        graphics.clear_screen(Color::from_rgb(0.8, 0.9, 1.0));

        self.p.update();
        self.p.draw(graphics);

        self.p2.update();
        self.p2.draw(graphics);

        helper.request_redraw();
    }
}

struct Pendulum {
    origin: Vector,

    position: Vector,
    // this is the angle of the pendulum.
    angle: f32,

    angle_velocity: f32,
    angle_acceleration: f32,

    r: f32, // the length of the pendulum.
    m: f32, // the mass of the ball.
    g: f32, // the gravity.
}

impl Pendulum {
    fn new(x: f32, y: f32, r: f32) -> Pendulum {
        Pendulum {
            origin: Vector::new(x, y),
            position: vector::Vector::new(0.0, 0.0),
            angle: 1.0,
            angle_velocity: 0.0,
            angle_acceleration: 0.0,
            r: r,
            m: 1.0,
            g: 1.5,
        }
    }
    fn update(&mut self) {
        self.angle_acceleration = -1.0 * self.g * self.angle.sin() / self.r;
        self.angle_velocity += self.angle_acceleration;
        self.angle += self.angle_velocity;
        self.position
        .set(self.r * self.angle.sin(),
             self.r *self.angle.cos());

        self.position.add(&self.origin);
    }
    fn draw(&self, grafics: &mut Graphics2D) {
        grafics.draw_line(
            (self.origin.x, self.origin.y),
            (self.position.x, self.position.y),
            3.0,
            Color::RED,
        );
        grafics.draw_circle((self.position.x, self.position.y), 30.0, Color::BLUE);
    }
}

mod vector {

    pub struct Vector {
        pub x: f32,
        pub y: f32,
    }

    impl Vector {
        pub fn new(x: f32, y: f32) -> Vector {
            Vector { x, y }
        }
        pub fn add(&mut self, other: &Vector) -> &Vector {
            self.x += other.x;
            self.y += other.y;
            self
        }
        pub fn set(&mut self, x: f32, y: f32) -> &Vector {
            self.x = x;
            self.y = y;
            self
        }
    }
}
