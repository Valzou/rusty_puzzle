
extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL };

use crate::bsp;

pub struct App{
    gl: GlGraphics
}

static WHITE: [f32; 4] = [255.0, 255.0, 255.0, 1.0];
static BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
static RED: [f32; 4] = [255.0, 0.0, 0.0, 1.0];
static BLUE: [f32; 4] = [0.0, 0.0, 255.0, 1.0];

impl App {  
    
    fn render(&mut self, args: &RenderArgs){
        use graphics::*;

          self.gl.draw(args.viewport(), |c, gl| {
            clear(WHITE, gl);
        });
    }

    fn draws_lines(&mut self, args:&RenderArgs, lines: &Vec<bsp::Line>){
        use graphics::*;

        self.gl.draw(args.viewport(), |c, gl| {
            let transform = c
                .transform;
            
            for l in lines {
                let color = match &l.color{
                    None => BLACK,
                    Some(c) => match c {
                        bsp::Color::BLACK => BLACK,
                        bsp::Color::WHITE => WHITE,
                        bsp::Color::RED => RED,
                        bsp::Color::BLUE => BLUE
                    }                       
                };
                
                let (sx, sy) = l.start;
                let (ex, ey) = l.end;
                line(color, 1.0, [sx as f64, sy as f64, ex as f64, ey as f64], transform, gl);
            }
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

pub fn create_application(width: u32, height:u32, lines: &Vec<bsp::Line>){
    let opengl = OpenGL::V3_2;
    let mut window = create_window(opengl, "Rusty Puzzle".to_string(), width, height);
    let mut app = App { 
        gl: GlGraphics::new(opengl)
    };

    let mut events = Events::new(EventSettings::new());
    
    while let Some(e) = events.next(&mut window){
        if let Some(args) = e.render_args() {
            app.render(&args);
            app.draws_lines(&args, lines);
        } 
    }
}
