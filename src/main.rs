extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow;
use opengl_graphics::{ GlGraphics, OpenGL };

struct Game {
    gl: GlGraphics,
    snake: Snake
}

impl Game {
    fn render(&mut self, arg: &RenderArgs) {
        use graphics;

        let color_green: [f32; 4] = 
            [
                0.42352941176,
                0.47843137254,
                0.53725490196,
                1.0
            ];
        self.gl.draw(arg.viewport(), |_c, gl| {
            graphics::clear(color_green, gl);
        });

        self.snake.render(&mut self.gl, arg);
        self.snake.pos_x += self.snake.vel_x;
        self.snake.pos_y += self.snake.vel_y;

        if self.snake.pos_x < 0 || self.snake.pos_x > 620 {
            self.snake.vel_x = -self.snake.vel_x;
        }

        if self.snake.pos_y < 0 || self.snake.pos_y > 460 {
            self.snake.vel_y = -self.snake.vel_y;
        }
    }
}

struct Snake {
    pos_x: i32,
    pos_y: i32,
    vel_x: i32,
    vel_y: i32,
}

impl Snake {
    fn render(&self, gl: &mut GlGraphics, arg: &RenderArgs) {
        use graphics;
        //rgba(244,67,54,1)
        let color_red: [f32; 4] = 
            [
                0.956862745,
                0.262745098,
                0.211764706,
                1.0
            ];
        
        let square = 
            graphics::rectangle::square(self.pos_x as f64,
                                        self.pos_y as f64, 
                                        20_f64);
        gl.draw(arg.viewport(), |_c, gl| {
            let transform = _c.transform;
            graphics::rectangle(color_red, square, transform, gl);
        });
    }
}

fn main() {

    let opengl_version = OpenGL::V3_1;

    let window_dimensions = [640, 480];
    let mut window: GlutinWindow = 
                WindowSettings::new("Play Snek", window_dimensions)
                .opengl(opengl_version)
                    .exit_on_esc(true)
                    .srgb(false)
                    .build()
                    .unwrap();

    let gl_graphics: GlGraphics = GlGraphics::new(opengl_version);
    let snake_guy: Snake = Snake 
    {
        pos_x: 10, 
        pos_y: 10,
        vel_x: 5,
        vel_y: 5
    };

    let mut game = Game 
    {
        gl: gl_graphics,
        snake: snake_guy
    };

    let mut events = Events::new(EventSettings::new());
    while let Some(event) = events.next(&mut window) {
        if let Some(rend_args) = event.render_args() {
            game.render(&rend_args);
        }
    }
}
