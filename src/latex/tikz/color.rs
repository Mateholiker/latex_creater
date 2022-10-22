#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub fn name(&self) -> String {
        format!("0x{:0>2x}{:0>2x}{:0>2x}", self.r, self.g, self.b)
    }
}

//Main KIT Colors

const KIT_GREEN: Color = Color {
    r: 0,
    g: 150,
    b: 130,
}; //#009682

const KIT_BLUE: Color = Color {
    r: 70,
    g: 100,
    b: 170,
}; //#4664AA

const KIT_BLACK: Color = Color { r: 0, g: 0, b: 0 }; //#000000

const KIT_BLACK_70: Color = Color {
    r: 64,
    g: 64,
    b: 64,
}; //#404040

//More KIT Colors

const KIT_YELLOW: Color = Color {
    r: 252,
    g: 229,
    b: 0,
}; //#FCE500

const KIT_ORANGE: Color = Color {
    r: 223,
    g: 155,
    b: 27,
}; //#DF9B1B

const KIT_MAY_GREEN: Color = Color {
    r: 140,
    g: 182,
    b: 60,
}; //#8CB63C

const KIT_RED: Color = Color {
    r: 162,
    g: 34,
    b: 35,
}; //#A22223

const KIT_PURPLE: Color = Color {
    r: 163,
    g: 16,
    b: 124,
}; //#A3107C

const KIT_BROWN: Color = Color {
    r: 167,
    g: 130,
    b: 46,
}; //#A7822E

const KIT_CYAN: Color = Color {
    r: 35,
    g: 161,
    b: 224,
}; //#23A1E0
