extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;


fn main() {
    let opengl = OpenGL::V3_2;

    let mut window: Window = WindowSettings::new("snake", [200, 200])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

        let mut events = Events::new(EventSettings::new());
        while let Some(e) = events.next(&mut window) {
            
        }
}