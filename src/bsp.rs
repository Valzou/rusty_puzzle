pub enum Color{
    BLACK
}

struct Label {
    coord : u32,
    colored : bool
}

struct Rect {
    couleur : Option<Color>,
    xd : u32,
    xf : u32,
    yd : u32,
    yf : u32
}

struct Line {
    deb : (u32, u32),
    fin : (u32, u32),
    color : Option<Color>
}

enum Bsp {
    R (Option<Color>),
    L (Label, Box<Bsp>, Box<Bsp>)
}
