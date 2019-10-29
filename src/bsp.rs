use crate::graphics;

struct Label {
    coord : u32,
    colored : bool
}

struct Rect {
    couleur : Option<graphics::Color>,
    xd : u32,
    xf : u32,
    yd : u32,
    yf : u32
}

struct Line {
    deb : (u32, u32),
    fin : (u32, u32),
    color : Option<graphics::Color>
}

enum Bsp {
    R (Option<graphics::Color>),
    L (Label, Box<Bsp>, Box<Bsp>)
}
