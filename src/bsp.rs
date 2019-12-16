pub enum Color{
    LRED,
    LBLUE,
    RRED,
    RBLUE,
    WHITE
}

pub struct Rect {
    pub start : (u32, u32),
    pub end : (u32, u32),
    pub color : Option<Color>,
}

pub struct Line {
    pub start : (u32, u32),
    pub end : (u32, u32),
    pub color : Option<Color>
}

#[derive(Clone)]
pub enum Axis {
    HORIZONTAL,
    VERTICAL,
}

pub enum Bsp {
    R,
    L (Line, Box<Bsp>, Box<Bsp>)
}

fn search_rect(rects: &mut Vec<Rect>, x: u32, y: u32) -> Option<&mut Rect>{
    for r in rects.iter_mut() {
        let (sx, sy) = r.start;
        let (ex, ey) = r.end;

        if x >= sx && x <= ex && y >= sy && y <= ey {
            return Some(r)
        }
    }
    None
}

fn swap_rect_color(rect: &mut Rect){
    rect.color = match rect.color {
        Some(Color::RBLUE) => Some(Color::RRED),
        Some(Color::RRED) => Some(Color::WHITE),
        _ => Some(Color::RBLUE)
    }
}

pub fn change_rect_color(rects: &mut Vec<Rect>, x: u32, y: u32){
    let r: &mut Rect = match search_rect(rects, x, y) {
        None => return,
        Some (r) => r
    };
    swap_rect_color(r)    
}


pub fn bsp_to_lines(bsp: &Bsp) -> Vec<&Line> {
    match bsp {
        Bsp::R => Vec::new(),
        Bsp::L(l, left, right) => {
            let mut left = bsp_to_lines(&*left);
            let mut right = bsp_to_lines(&*right);
            left.append(&mut right);
            left.push(l);
            left
        }
    }
    
}

fn bsp_to_rectangles_aux(bsp: &Bsp, startx: u32, endx: u32, starty: u32, endy: u32) -> Vec<Rect> {
    match bsp {
        Bsp::R => {
            let mut v = Vec::new();
            let r = Rect {
                start: (startx, starty),
                end: (endx, endy),
                color: None
            };
            v.push(r);
            v
        },
        Bsp::L (Line{start:(sx, sy), end:(ex, _), color:_}, l, r) => {
            let is_vertical = sx==ex;
            let mut left;
            let mut right;
            if is_vertical {
                left = bsp_to_rectangles_aux(&*l, startx, sx-1, starty, endy);
                right = bsp_to_rectangles_aux(&*r, sx+1, endx, starty, endy);
            } else {
                left = bsp_to_rectangles_aux(&*l, startx, endx, starty, sy-1);
                right = bsp_to_rectangles_aux(&*r, startx, endx, sy+1, endy);
            }
            left.append(&mut right);
            left
        }
    }
}

pub fn bsp_to_rectangles(bsp: &Bsp, width: u32, height: u32) -> Vec<Rect> {
    return bsp_to_rectangles_aux(bsp, 0, width, 0, height);
}

fn generate_bsp_aux(n: u32, startx: u32, endx: u32, starty: u32, endy: u32, axis: Axis) -> Bsp{
    if n <= 0 {
        return Bsp::R;
    }

    use rand::{Rng};
    let mut rng = rand::thread_rng();
    
    let coord: u32 = match axis {
        Axis::VERTICAL => {
            let offset = 0.25 * ( (endx - startx) as f32);
            rng.gen_range(startx+(offset as u32), endx-(offset as u32))
            },
        Axis::HORIZONTAL => {
            let offset = 0.25 * ( (endy - starty) as f32);
            rng.gen_range(starty+(offset as u32), endy-(offset as u32))
        }
    };

    let (start, end) = match axis {
        Axis::VERTICAL => ( (coord, starty), (coord, endy) ),
        Axis::HORIZONTAL => ( (startx, coord), (endx, coord) )
    };
    
    let mut color = Color::LRED;
    if rng.gen_range(0, 2) == 0 {
        color = Color::LBLUE;
    };

    let line = Line {
        start: start,
        end: end,
        color: Some(color),
    };

    let next_axis = match axis {
        Axis::VERTICAL => Axis::HORIZONTAL,
        Axis::HORIZONTAL => Axis::VERTICAL
    };

    let ( (next_left_startx, next_left_endx), (next_right_startx, next_right_endx) ) = match axis {
        Axis::VERTICAL => (
            (startx,  coord-1),
            (coord+1, endx)
        ),
        Axis::HORIZONTAL => (
            (startx, endx),
            (startx, endx)
        )
    };

    let ( (next_left_starty, next_left_endy), (next_right_starty, next_right_endy) ) = match axis {
        Axis::VERTICAL => (
            (starty, endy),
            (starty, endy)
        ),
        Axis::HORIZONTAL => (
            (starty, coord-1),
            (coord+1, endy)
        )
    };
    
    let left = Box::new(generate_bsp_aux(n-1, next_left_startx, next_left_endx, next_left_starty, next_left_endy, next_axis.clone()));
    let right = Box::new(generate_bsp_aux(n-1, next_right_startx, next_right_endx, next_right_starty, next_right_endy, next_axis.clone()));
                 
    return Bsp::L (line, left, right);
}

pub fn generate_bsp(n: u32, width: u32, height: u32) -> Bsp {
    generate_bsp_aux(n, 0, width, 0, height, Axis::VERTICAL)
}
