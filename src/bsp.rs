pub enum Color{
    BLACK,
    WHITE,
    RED,
    BLUE
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
