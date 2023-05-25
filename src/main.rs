extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;

const TILE_SIZE: f64 = 20.0;

pub struct App {
    gl: GlGraphics,
    snake: Snake
}

struct Snake{
    gl: GlGraphics,
    pos_x: i32,
    pos_y: i32
}

impl Snake{
    fn render(&mut self, args: &RenderArgs){
        use graphics::*;


        const RED: [f32;4] = [1.0,0.0,0.0,1.0];

        let square = rectangle::square(self.pos_x as f64 * TILE_SIZE, self.pos_y as f64 * TILE_SIZE, TILE_SIZE);

        self.gl.draw(args.viewport(), | c, gl| {
            let transform = c.transform;

            rectangle(RED, square, transform, gl);
        })
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

        self.snake.render(args)
    }
}

fn main() {
    let opengl = OpenGL::V3_2;

    let mut window: Window = WindowSettings::new("snake", [200, 200])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

        let mut app = App {
            gl: GlGraphics::new(opengl),
            snake: Snake{
                gl: GlGraphics::new(opengl),
                pos_x: 4,
                pos_y: 4
            }
        };


        let mut events = Events::new(EventSettings::new());
        while let Some(e) = events.next(&mut window) {
            if let Some(args) = e.render_args() {
                app.render(&args);
            }
        }
}