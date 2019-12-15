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

enum Bsp {
    R (Option<Color>),
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
