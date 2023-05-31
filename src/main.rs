extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate rand;

use piston::event_loop::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::*;
use piston::window::WindowSettings;

use std::collections::LinkedList;
use rand::Rng;

const SCREEN_SIZE: [u32;2] = [400,400];
const TILE_SIZE: u32 = 25;
const ROWS: u32 = SCREEN_SIZE[0] / TILE_SIZE;
const COLS: u32 = SCREEN_SIZE[1] / TILE_SIZE;

const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
const RED: [f32;4] = [1.0,0.0,0.0,1.0];
const BLUE: [f32;4] = [0.0,0.0,1.0,1.0];

pub struct App {
    gl: GlGraphics,
    snake: Snake,
    food: Food
}

struct Snake{
    gl: GlGraphics,
    snake_parts: LinkedList<Coordinates>,
    direction: Coordinates,
    new_direction: Coordinates,
    pos: Coordinates
}

struct Food{
    gl: GlGraphics,
    pos: Coordinates,
}

impl Food{
    fn render(&mut self, args: &RenderArgs){
        use graphics::*;

        let square = rectangle::square((self.pos.x * TILE_SIZE as i32) as f64,(self.pos.y * TILE_SIZE as i32) as f64, TILE_SIZE as f64);
        self.gl.draw(args.viewport(), | c, gl| {
            let transform = c.transform;

            rectangle(BLUE, square, transform, gl);
        })
    }

    fn new_pos(&mut self){
        let mut rng = rand::thread_rng();
        self.pos = Coordinates{x: rng.gen_range(0..ROWS as i32),y:rng.gen_range(0..COLS as i32)};
    }
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

    fn update(&mut self){
        self.direction = match self.new_direction{
            Coordinates{x:1,y:0} if self.direction.x != -1 => Coordinates{x:1,y:0},
            Coordinates{x:-1,y:0} if self.direction.x != 1 => Coordinates{x:-1,y:0},
            Coordinates{x:0,y:1} if self.direction.y != -1 => Coordinates{x:0,y:1},
            Coordinates{x:0,y:-1} if self.direction.y != 1 => Coordinates{x:0,y:-1},
            _ => self.direction
        };

        self.pos.x += self.direction.x;
        self.pos.y += self.direction.y; 
        
        let head_pos: &Coordinates = &Coordinates { x: self.pos.x, y: self.pos.y };

        let mut iter = self.snake_parts.iter_mut();
        if let Some(mut prev_node) = iter.next() {
            while let Some(node) = iter.next() {
                *node = std::mem::replace(&mut prev_node, *node);
                if is_colliding(&node, &head_pos){
                    close_window()
                }
            }
        }

        if let Some(node) = self.snake_parts.front_mut() {
            *node = *head_pos
        }

    }

    fn pressed(&mut self, btn: &Button){
        self.new_direction = match btn {
            &Button::Keyboard(Key::Up) => Coordinates{x:0,y:-1},
            &Button::Keyboard(Key::Down)  => Coordinates{x:0,y:1},
            &Button::Keyboard(Key::Left)  => Coordinates{x:-1,y:0},
            &Button::Keyboard(Key::Right)  => Coordinates{x:1,y:0},
            _ => self.direction
        };
    }

    fn is_colliding(&mut self, pos: Coordinates) -> bool{
        let mut iter = self.snake_parts.iter_mut();
        while let Some(node) = iter.next() {
            if is_colliding(&node,&pos) {
                return true
            }
        }
        return false
    }
}

fn is_colliding(pos1: &Coordinates, pos2: &Coordinates) -> bool{
    if pos1.x == pos2.x && pos1.y == pos2.y {
        return true
    }
    return false
}

fn close_window(){
    std::process::exit(0);
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        
        self.gl.draw(args.viewport(), |_c, gl| {
            graphics::clear(GREEN, gl);
        });

        self.snake.render(args);
        self.food.render(args)
    }

    fn update(&mut self){
        self.snake.update();
        if self.snake.is_colliding(self.food.pos) {
            self.snake.snake_parts.push_back(Coordinates{x:-1,y:-1});
            self.food.new_pos();
        }
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

    let mut rng = rand::thread_rng();
    
    let mut app = App {
        gl: GlGraphics::new(opengl),
        snake: Snake{
            gl: GlGraphics::new(opengl),
            pos: Coordinates{x:4,y:4},
            snake_parts: LinkedList::from([Coordinates{x:4,y:4},Coordinates{x:3,y:4},Coordinates{x:2,y:4}]),
            direction: Coordinates{x:1,y:0},
            new_direction: Coordinates{x:1,y:0},
        },
        food: Food{
            gl: GlGraphics::new(opengl),
            pos: Coordinates{x: rng.gen_range(0..ROWS as i32),y:rng.gen_range(0..COLS as i32)}
        }
    };

    let mut events = Events::new(EventSettings::new().ups(8));
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            app.render(&args);
        }

        if e.update_args().is_some() {
            app.update()
        }

        if let Some(args) = e.button_args() {
            if args.state == ButtonState::Press {
                app.pressed(&args.button);
            }
        }
    }
}