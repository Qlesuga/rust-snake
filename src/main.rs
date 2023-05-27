extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use piston::event_loop::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::*;
use piston::window::WindowSettings;

use std::collections::LinkedList;

const TILE_SIZE: f64 = 20.0;

pub struct App {
    gl: GlGraphics,
    snake: Snake
}

struct Snake{
    gl: GlGraphics,
    snake_parts: LinkedList<Position>,
    direction: Direction,
    pos: Position
}

#[derive(Clone,Copy)]
struct Direction{
    x: i32,
    y: i32
}

#[derive(Clone,Copy)]
struct Position{
    x: i32,
    y: i32
}

impl Snake{
    fn render(&mut self, args: &RenderArgs){
        use graphics::*;


        const RED: [f32;4] = [1.0,0.0,0.0,1.0];


        let mut iter = self.snake_parts.iter_mut();
        while let Some(node) = iter.next() {

            let square = rectangle::square(node.x as f64 * TILE_SIZE,node.y as f64 * TILE_SIZE, TILE_SIZE);
            self.gl.draw(args.viewport(), | c, gl| {
                let transform = c.transform;

                rectangle(RED, square, transform, gl);
            })
        } 
    }

    fn update(&mut self, args: &UpdateArgs){
        self.pos.x += self.direction.x;
        self.pos.y += self.direction.y; 
        
        let mut iter = self.snake_parts.iter_mut();
        if let Some(mut prev_node) = iter.next() {
            while let Some(node) = iter.next() {
                *node = std::mem::replace(&mut prev_node, *node);
            }
        }

        if let Some(mut node) = self.snake_parts.front_mut() {
            *node = Position{x:self.pos.x,y:self.pos.y};
        }

    }

    fn pressed(&mut self, btn: &Button){
        self.direction = match btn {
            &Button::Keyboard(Key::Up) if self.direction.y != 1  => Direction{x:0,y:-1},
            &Button::Keyboard(Key::Down) if self.direction.y != -1 => Direction{x:0,y:1},
            &Button::Keyboard(Key::Left) if self.direction.x != 1 => Direction{x:-1,y:0},
            &Button::Keyboard(Key::Right) if self.direction.x != -1 => Direction{x:1,y:0},
            _ => self.direction
        };
    }
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];

        let square = rectangle::square(0.0, 0.0, 200.0);

        self.gl.draw(args.viewport(), |c, gl| {
            graphics::clear(GREEN, gl);
        });

        self.snake.render(args);
    }

    fn update(&mut self, args: &UpdateArgs){
        self.snake.update(args);
    }

    fn pressed(&mut self, btn: &Button){
        self.snake.pressed(btn);
    }
}

fn main() {
    let opengl = OpenGL::V3_2;

    let mut window: Window = WindowSettings::new("snake", [700, 700])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut app = App {
        gl: GlGraphics::new(opengl),
        snake: Snake{
            gl: GlGraphics::new(opengl),
            pos: Position{x:4,y:4},
            snake_parts: LinkedList::from([Position{x:4,y:4},Position{x:3,y:4},Position{x:2,y:4}]),
            direction: Direction{x:1,y:0}
        }
    };

    let mut events = Events::new(EventSettings::new().ups(8));
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            app.render(&args);
        }

        if let Some(args) = e.update_args() {
            app.update(&args)
        }

        if let Some(args) = e.button_args() {
            if args.state == ButtonState::Press {
                app.pressed(&args.button);
            }
        }
    }
}