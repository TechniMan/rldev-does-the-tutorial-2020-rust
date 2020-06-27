use rltk::{ RGB };

// PICO-8 colour palette: https://pico-8.fandom.com/wiki/Palette
const BLACK : RGB       = RGB { r: 0.000, g: 0.000, b: 0.000 }; // 0 0 0
const DARK_BLUE : RGB   = RGB { r: 0.114, g: 0.169, b: 0.325 }; // 29 43 83
const DARK_PURPLE : RGB = RGB { r: 0.494, g: 0.145, b: 0.325 }; // 126 37 83
const DARK_GREEN : RGB  = RGB { r: 0.000, g: 0.529, b: 0.318 }; // 0 135 81
const BROWN : RGB       = RGB { r: 0.671, g: 0.322, b: 0.212 }; // 171 82 54
const DARK_GREY : RGB   = RGB { r: 0.373, g: 0.341, b: 0.310 }; // 95 87 79
const LIGHT_GREY : RGB  = RGB { r: 0.761, g: 0.765, b: 0.780 }; // 194 195 199
const WHITE : RGB       = RGB { r: 1.000, g: 0.945, b: 0.910 }; // 255 241 232
const RED : RGB         = RGB { r: 1.000, g: 0.000, b: 0.302 }; // 255 0 77
const ORANGE : RGB      = RGB { r: 1.000, g: 0.639, b: 0.000 }; // 255 163 0
const YELLOW : RGB      = RGB { r: 1.000, g: 0.925, b: 0.153 }; // 255 236 39
const GREEN : RGB       = RGB { r: 0.000, g: 0.894, b: 0.212 }; // 0 228 54
const BLUE : RGB        = RGB { r: 0.161, g: 0.678, b: 1.000 }; // 41 173 255
const LAVENDER : RGB    = RGB { r: 0.514, g: 0.463, b: 0.612 }; // 131 118 156
const PINK : RGB        = RGB { r: 1.000, g: 0.467, b: 0.659 }; // 255 119 168
const PEACH : RGB       = RGB { r: 1.000, g: 0.800, b: 0.667 }; // 255 204 170

pub const COLOURS : [RGB; 16] = [
    BLACK,
    DARK_BLUE,
    DARK_PURPLE,
    DARK_GREEN,
    BROWN,
    DARK_GREY,
    LIGHT_GREY,
    WHITE,
    RED,
    ORANGE,
    YELLOW,
    GREEN,
    BLUE,
    LAVENDER,
    PINK,
    PEACH
];
