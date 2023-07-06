extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::RenderArgs;
use piston::{RenderEvent, UpdateEvent};
// use piston::input::{RenderArgs, UpdateArgs};
// use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;

pub trait Logic {
    fn update(&mut self) -> ();
    fn get_gen(&mut self) -> (usize, Vec<bool>);
}

pub struct Graphics<T: Logic> {
    logic: T,
    gl: GlGraphics, // OpenGL drawing backend.
    pub window: Window,
}

impl<T: Logic> Graphics<T> {
    pub fn start(logic: T) -> Self {
        // Change this to OpenGL::V2_1 if not working.
        let opengl = OpenGL::V3_2;

        // Create a Glutin window.
        let window: Window = WindowSettings::new("spinning-square", [800, 800])
            .graphics_api(opengl)
            .exit_on_esc(true)
            .build()
            .unwrap();

        // Create a new game and run it.
        Self {
            logic,
            window,
            gl: GlGraphics::new(opengl),
        }
    }

    pub fn run(&mut self) {
        let mut events = Events::new(EventSettings::new());
        while let Some(e) = events.next(&mut self.window) {
            if let Some(args) = e.render_args() {
                self.render(&args);
            }

            if let Some(_args) = e.update_args() {
                // app.update(&args);
                self.logic.update();
            }
        }
    }

    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const GRAY: [f32; 4] = [0.1, 0.1, 0.1, 1.0];
        const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

        let width = 4.0;
        let gap_x = 2.0;
        let gap_y = 4.0;
        let max_h = 800.0;
        let square = rectangle::square(0.0, 0.0, width);

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(GRAY, gl);

            let (count, gen) = self.logic.get_gen();
            let count = count as f64;
            for (i, gen) in gen.iter().enumerate() {
                if *gen == true {
                    let x = gap_x + i as f64 * (width + gap_x);
                    let y = (gap_y + gap_y * count) % max_h;
                    let transform = c.transform.trans(x, y);

                    rectangle(RED, square, transform, gl);
                }
            }
        });
    }
}
