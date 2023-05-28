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

const SCREEN_SIZE: [u32;2] = [300,300];
const TILE_SIZE: u32 = 25;
const ROWS: u32 = SCREEN_SIZE[0] / TILE_SIZE;
const COLS: u32 = SCREEN_SIZE[1] / TILE_SIZE;

const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
const RED: [f32;4] = [1.0,0.0,0.0,1.0];
const BLUE: [f32;4] = [0.0,0.0,1.0,1.0];

pub struct App {
    gl: GlGraphics,
    snake: Snake
}

struct Snake{
    gl: GlGraphics,
    snake_parts: LinkedList<Coordinates>,
    direction: Coordinates,
    pos: Coordinates
}

#[derive(Clone,Copy)]
struct Coordinates{
    x: i32,
    y: i32
}

impl Snake{
    fn render(&mut self, args: &RenderArgs){
        use graphics::*;

        let mut iter = self.snake_parts.iter_mut();
        while let Some(node) = iter.next() {

            let square = rectangle::square((node.x * TILE_SIZE as i32) as f64,(node.y * TILE_SIZE as i32) as f64, TILE_SIZE as f64);
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
            *node = Coordinates{x:self.pos.x,y:self.pos.y};
        }

    }

    fn pressed(&mut self, btn: &Button){
        self.direction = match btn {
            &Button::Keyboard(Key::Up) if self.direction.y != 1  => Coordinates{x:0,y:-1},
            &Button::Keyboard(Key::Down) if self.direction.y != -1 => Coordinates{x:0,y:1},
            &Button::Keyboard(Key::Left) if self.direction.x != 1 => Coordinates{x:-1,y:0},
            &Button::Keyboard(Key::Right) if self.direction.x != -1 => Coordinates{x:1,y:0},
            _ => self.direction
        };
    }
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

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

    let mut window: Window = WindowSettings::new("snake", SCREEN_SIZE)
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut app = App {
        gl: GlGraphics::new(opengl),
        snake: Snake{
            gl: GlGraphics::new(opengl),
            pos: Coordinates{x:4,y:4},
            snake_parts: LinkedList::from([Coordinates{x:4,y:4},Coordinates{x:3,y:4},Coordinates{x:2,y:4}]),
            direction: Coordinates{x:1,y:0}
        }
    };

    let mut events = Events::new(EventSettings::new().ups(12));
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