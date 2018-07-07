extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

use std::iter::FromIterator;
use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow;
use opengl_graphics::{ GlGraphics, OpenGL };
use std::collections::LinkedList;

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
    }

    fn update(&mut self) {
        self.snake.update();
    }

    fn key_pressed(&mut self, button: &Button) {
        let last_direction = self.snake.direction.clone();

        self.snake.direction = match button {
            &Button::Keyboard(Key::Up)
                if last_direction != Direction::Up => Direction::Up,
            &Button::Keyboard(Key::Down)
                if last_direction != Direction::Down => Direction::Down,
            &Button::Keyboard(Key::Left)
                if last_direction != Direction::Left => Direction::Left,
            &Button::Keyboard(Key::Right)
                if last_direction != Direction::Right => Direction::Right,
            _ => last_direction
        };
    }
}

struct Snake {
    body: LinkedList<(i32, i32)>,
    direction: Direction,
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

        let squares: Vec<graphics::types::Rectangle> = 
            self.body
                .iter()
                .map(|&(square_x, square_y)| {
                    graphics::rectangle::square(
                        (square_x * 20) as f64,
                        (square_y * 20) as f64, 
                        20_f64)
                })
                .collect();
        
        gl.draw(arg.viewport(), |_c, gl| {
            let transform = _c.transform;
            squares
                .into_iter()
                .for_each(|square| {
                    graphics::rectangle(color_red, square, transform, gl);
                })
                
        });
    }

    fn update(&mut self) {
        let mut new_head = (*self.body.front().expect("No Body")).clone();

        match self.direction {
            Direction::Left => new_head.0 -= 1,
            Direction::Right => new_head.0 += 1,
            Direction::Up => new_head.1 -= 1,
            Direction::Down => new_head.1 += 1,
        }

        self.body.push_front(new_head);
        self.body.pop_back().unwrap();
    }
}

#[derive(Clone, PartialEq)]
enum Direction {
    Up, Down, Left, Right
}

fn main() {

   let opengl_version = OpenGL::V3_2;

    let window_dimensions = [640, 480];
    let mut window: GlutinWindow = 
                WindowSettings::new("Play Snek", window_dimensions)
                .opengl(opengl_version)
                    .exit_on_esc(true)
                    .srgb(false)
                    .build()
                    .unwrap();

    let gl_graphics: GlGraphics = GlGraphics::new(opengl_version);
    let snake_body = LinkedList::from_iter((vec![(0,0), (0,1)]).into_iter());
    let snake_guy: Snake = Snake 
    {
        body: snake_body,
        direction: Direction::Right
    };

    let mut game = Game 
    {
        gl: gl_graphics,
        snake: snake_guy
    };

    let mut events = Events::new(EventSettings::new()).ups(10);
    while let Some(event) = events.next(&mut window) {

        if let Some(rend_args) = event.render_args() {
            game.render(&rend_args);
        }

        if let Some(_) = event.update_args() {
            game.update();
        }

        if let Some(rend_args) = event.button_args() {
            if rend_args.state == ButtonState::Press {
                game.key_pressed(&rend_args.button);
            }
        }
    }
}
