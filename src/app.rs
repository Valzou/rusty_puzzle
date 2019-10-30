extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL };

pub struct App{
    gl: GlGraphics
}

impl App {
    fn render(&mut self, args: &RenderArgs){
        use graphics::*;

        const WHITE: [f32; 4] = [255.0, 255.0, 255.0, 1.0];

        self.gl.draw(args.viewport(), |c, gl| {
            clear(WHITE, gl);
        });
    }
}

fn create_window(opengl: OpenGL, name:String, width:u32, height:u32) -> Window{
    let mut window: Window = WindowSettings::new(
        name,
        [width, height]
    )
    .graphics_api(opengl)
    .exit_on_esc(true)
    .build()
    .unwrap();

    return window;
}

pub fn create_application(){
    let opengl = OpenGL::V3_2;
    let mut window = create_window(opengl, "Rusty Puzzle".to_string(), 400, 400);
    let mut app = App { 
        gl: GlGraphics::new(opengl)
    };

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window){
        if let Some(r) = e.render_args() {
            app.render(&r);
        }
    }
}