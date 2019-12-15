
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

static WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
static BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
static RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
static BLUE: [f32; 4] = [0.0, 0.0, 1.0, 1.0];
static DARK_RED: [f32; 4] = [0.6, 0.0, 0.0, 1.0];
static DARK_BLUE: [f32; 4] = [0.0, 0.0, 0.5, 1.0];

impl App {  
    
    fn render(&mut self, args: &RenderArgs){
        use graphics::*;

          self.gl.draw(args.viewport(), |_, gl| {
            clear(WHITE, gl);
        });
    }

    fn to_color(color: &bsp::Color) -> [f32; 4]{
        match color {
            bsp::Color::RRED => DARK_RED,
            bsp::Color::RBLUE => DARK_BLUE,
            bsp::Color::LRED => RED,
            bsp::Color::LBLUE => BLUE,
            bsp::Color::WHITE => WHITE
                
        }
    }

    fn from_bsp_color(color: &Option<bsp::Color>, default: [f32; 4]) -> [f32; 4]{
        let c = match color{
            None => default,
            Some(c) => App::to_color(c)      
        };
        c
    }
    
    fn draws_lines(&mut self, args:&RenderArgs, lines: &Vec<bsp::Line>){
        use graphics::*;

        self.gl.draw(args.viewport(), |c, gl| {
            let transform = c
                .transform;
            
            for l in lines {
                let color = App::from_bsp_color(&l.color, BLACK);
                
                let (sx, sy) = l.start;
                let (ex, ey) = l.end;
                line(color, 1.0, [sx as f64, sy as f64, ex as f64, ey as f64], transform, gl);
            }
        });    
          
    }

    fn draw_rects(&mut self, args:&RenderArgs, rects: &mut Vec<bsp::Rect>){
        use graphics::*;

        self.gl.draw(args.viewport(), |c, gl| {
            let transform = c
                .transform;

            for r in rects {
                let color = App::from_bsp_color(&r.color, WHITE);

                let (sx, sy) = r.start;
                let (ex, ey) = r.end;

                rectangle(color, [sx as f64, sy as f64, ex as f64, ey as f64], transform, gl);
                
            }
        });
        
    }
}

fn create_window(opengl: OpenGL, name:String, width:u32, height:u32) -> Window{
    let window: Window = WindowSettings::new(
        name,
        [width, height]
    )
    .graphics_api(opengl)
    .exit_on_esc(true)
    .build()
    .unwrap();

    return window;
}

pub fn create_application(width: u32, height:u32, lines: &Vec<bsp::Line>, rects: &mut Vec<bsp::Rect>){
    let opengl = OpenGL::V3_2;
    let mut window = create_window(opengl, "Rusty Puzzle".to_string(), width, height);
    let mut app = App { 
        gl: GlGraphics::new(opengl)
    };

    let mut events = Events::new(EventSettings::new());
g
    let mut cursor_pos: [f64; 2] = [-1.0; 2];
    
    while let Some(e) = events.next(&mut window){
        if let Some(args) = e.render_args() {
            app.render(&args);
            app.draw_rects(&args, rects);
            app.draws_lines(&args, lines)
        }

        if let Some(pos) = e.mouse_cursor_args(){
            cursor_pos = pos;
        }
        
        if let Some(Button::Mouse(MouseButton::Left)) = e.press_args() {
            let x = cursor_pos[0];
            let y = cursor_pos[1];

            if x >= 0.0 && x < width as f64 && y >= 0.0 && y < height as f64 {
                bsp::change_rect_color(rects, x as u32, y as u32);
            }
        }
    }
}
