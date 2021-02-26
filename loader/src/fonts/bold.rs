// This file is autogenerated by xous-core/loader/src/generate_fonts.py. Do not edit.
#[allow(dead_code)]
#[link_section=".fontdata"]
#[no_mangle]
/// Packed glyph pattern data.
/// Record format:
///  [offset+0]: ((w as u8) << 16) | ((h as u8) << 8) | (yOffset as u8)
///  [offset+1..=ceil(w*h/32)]: packed 1-bit pixels; 0=clear, 1=set
/// Pixels are packed in top to bottom, left to right order with MSB of first
/// pixel word containing the top left pixel.
///  w: Width of pattern in pixels
///  h: Height of pattern in pixels
///  yOffset: Vertical offset (pixels downward from top of line) to position
///     glyph pattern properly relative to text baseline
pub static DATA_BOLD: [u32; 1501] = [
    // [0]: 20 " "
    0x0004020e, 0x00000000,
    // [2]: 21 "!"
    0x00041206, 0xffffffff, 0xffff00ff, 0xff000000,
    // [6]: 22 "\""
    0x00060606, 0xcf3cf3cf, 0x30000000,
    // [9]: 23 "#"
    0x00101004, 0x30c030c0, 0x30c030c0, 0xfffcfffc, 0x0c300c30, 0x0c300c30, 0x3fff3fff, 0x030c030c,
    0x030c030c,
    // [18]: 24 "$"
    0x000a1604, 0x0c0303f0, 0xfcccf330, 0xfc3f0fc3, 0xf3f0fcfc, 0x3f0fc3f0, 0xccf333f0, 0xfc0c0300,
    // [26]: 25 "%"
    0x00121404, 0x0fcf03f3, 0xc0c30c30, 0xc30330c0, 0xcc3030f0, 0x0c3c00c0, 0x0030000c, 0x0003003c,
    0x300f0c0c, 0x33030cc0, 0xc30c30c3, 0x03c0c0f0, 0x30000000,
    // [39]: 26 "&"
    0x00101206, 0x00fc00fc, 0x03cf03cf, 0x03cf03cf, 0x03ff03ff, 0xf0fcf0fc, 0x3fcf3fcf, 0x0f0f0f0f,
    0x3fcf3fcf, 0xf0fcf0fc,
    // [49]: 27 "'"
    0x00020606, 0xfff00000,
    // [51]: 28 "("
    0x00061604, 0xc3030c3c, 0xf3cf3cf3, 0xcf3cf3cf, 0x3cf30cc3, 0x00000000,
    // [57]: 29 ")"
    0x00061604, 0x0c330cf3, 0xcf3cf3cf, 0x3cf3cf3c, 0xf3c30c0c, 0x30000000,
    // [63]: 2A "*"
    0x000a0a06, 0x0c030ccf, 0x333f0fcc, 0xcf330c03, 0x00000000,
    // [68]: 2B "+"
    0x000a0a0a, 0x0c0300c0, 0x30fffff0, 0xc0300c03, 0x00000000,
    // [73]: 2C ","
    0x00040814, 0xffffcc33,
    // [75]: 2D "-"
    0x000a020e, 0xfffff000,
    // [77]: 2E "."
    0x00040414, 0xffff0000,
    // [79]: 2F "/"
    0x000a1404, 0xc0300c03, 0x00300c03, 0x00c00c03, 0x00c03003, 0x00c0300c, 0x00c0300c, 0x03000000,
    // [87]: 30 "0"
    0x000c1206, 0x3fc3fcf0, 0xff0ff0ff, 0x0ffcffcf, 0xfffffff3, 0xff3ff0ff, 0x0ff0ff0f, 0x3fc3fc00,
    // [95]: 31 "1"
    0x00081206, 0xf0f0ffff, 0xf0f0f0f0, 0xf0f0f0f0, 0xf0f0f0f0, 0xf0f00000,
    // [101]: 32 "2"
    0x000c1206, 0x3fc3fcf0, 0x3f03f00f, 0x00f00f00, 0x3c03c00f, 0x00f003c0, 0x3cffffff, 0xffffff00,
    // [109]: 33 "3"
    0x000c1206, 0xffffff3c, 0x03c00f00, 0xf03fc3fc, 0xf00f00f0, 0x0f00f00f, 0x00f03f03, 0x3fc3fc00,
    // [117]: 34 "4"
    0x000e1206, 0x3c00f003, 0xf00fc03c, 0xc0f303c3, 0x0f0c3c0c, 0xf03fffff, 0xff3c00f0, 0x03c00f00,
    0x3c00f000,
    // [126]: 35 "5"
    0x000c1206, 0xffffff00, 0xf00f00f0, 0x0f3ff3ff, 0xf00f00f0, 0x0f00f00f, 0x00f03f03, 0x3fc3fc00,
    // [134]: 36 "6"
    0x000c1206, 0x3f03f003, 0xc03c00f0, 0x0f3ff3ff, 0xf0ff0ff0, 0xff0ff0ff, 0x0ff0ff0f, 0x3fc3fc00,
    // [142]: 37 "7"
    0x000c1206, 0xfffffff0, 0x0f00f00f, 0x00f00f00, 0x3c03c00f, 0x00f00f00, 0xf00f00f0, 0x0f00f000,
    // [150]: 38 "8"
    0x000c1206, 0x3fc3fcf0, 0xff0ff0ff, 0x0f3fc3fc, 0xf0ff0ff0, 0xff0ff0ff, 0x0ff0ff0f, 0x3fc3fc00,
    // [158]: 39 "9"
    0x000c1206, 0x3fc3fcf0, 0xff0ff0ff, 0x0ff0ff0f, 0xf0ff0fff, 0xcffcf00f, 0x003c03c0, 0x0fc0fc00,
    // [166]: 3A ":"
    0x00040e0a, 0xffff0000, 0x00ffff00,
    // [169]: 3B ";"
    0x0004120a, 0xffff0000, 0x00ffffcc, 0x33000000,
    // [173]: 3C "<"
    0x000a0e08, 0xf03c03c0, 0xf00f03c0, 0x3c0f0f03, 0xc3c0f0f0, 0x3c000000,
    // [179]: 3D "="
    0x000c060c, 0xffffff00, 0x0000ffff, 0xff000000,
    // [183]: 3E ">"
    0x000a0e08, 0x03c0f0f0, 0x3c3c0f0f, 0x03c03c0f, 0x00f03c03, 0xc0f00000,
    // [189]: 3F "?"
    0x000c1206, 0x3fc3fcf0, 0x3f03f00f, 0x003c03c0, 0x0f00f00f, 0x00f00000, 0x000f00f0, 0x0f00f000,
    // [197]: 40 "@"
    0x00121008, 0x0ffc03ff, 0x030030c0, 0x0cc3f0f0, 0xfc3c30cf, 0x0c33c30c, 0xf0c333cf, 0x0cf3c300,
    0x030000c0, 0xffc03ff0,
    // [207]: 41 "A"
    0x000c1206, 0x3fc3fcf0, 0xff0ff0ff, 0x0ff0ff0f, 0xfffffff0, 0xff0ff0ff, 0x0ff0ff0f, 0xf0ff0f00,
    // [215]: 42 "B"
    0x000c1206, 0x3ff3fff0, 0xff0ff0ff, 0x0ff0ff0f, 0x3ff3fff0, 0xff0ff0ff, 0x0ff0ff0f, 0x3ff3ff00,
    // [223]: 43 "C"
    0x000c1206, 0x3fc3fcc0, 0xfc0f00f0, 0x0f00f00f, 0x00f00f00, 0xf00f00f0, 0x0fc0fc0f, 0x3fc3fc00,
    // [231]: 44 "D"
    0x000c1206, 0x3ff3fff0, 0xff0ff0ff, 0x0ff0ff0f, 0xf0ff0ff0, 0xff0ff0ff, 0x0ff0ff0f, 0x3ff3ff00,
    // [239]: 45 "E"
    0x000a1206, 0xfffff03c, 0x0f03c0f0, 0x3c0f3fcf, 0xf03c0f03, 0xc0f03c0f, 0xfffff000,
    // [246]: 46 "F"
    0x000a1206, 0xfffff03c, 0x0f03c0f0, 0x3c0f3fcf, 0xf03c0f03, 0xc0f03c0f, 0x03c0f000,
    // [253]: 47 "G"
    0x000c1206, 0x3fc3fcc0, 0xfc0f00f0, 0x0f00f00f, 0xfcffcff0, 0xff0ff0ff, 0x0ff0ff0f, 0x3fc3fc00,
    // [261]: 48 "H"
    0x000c1206, 0xf0ff0ff0, 0xff0ff0ff, 0x0ff0ff0f, 0xfffffff0, 0xff0ff0ff, 0x0ff0ff0f, 0xf0ff0f00,
    // [269]: 49 "I"
    0x00081206, 0xffff3c3c, 0x3c3c3c3c, 0x3c3c3c3c, 0x3c3c3c3c, 0xffff0000,
    // [275]: 4A "J"
    0x000c1206, 0xf00f00f0, 0x0f00f00f, 0x00f00f00, 0xf00f00f0, 0xff0ff0ff, 0x0ff0ff0f, 0x3fc3fc00,
    // [283]: 4B "K"
    0x000e1206, 0xf03fc0f3, 0xc3cf0f0f, 0x3c3cf03f, 0xc0ff00fc, 0x03f03fc0, 0xff0f3c3c, 0xf3c3cf0f,
    0xf03fc0f0,
    // [292]: 4C "L"
    0x000a1206, 0x03c0f03c, 0x0f03c0f0, 0x3c0f03c0, 0xf03c0f03, 0xc0f03c0f, 0xfffff000,
    // [299]: 4D "M"
    0x00141206, 0xc0003c00, 0x03f000ff, 0x000ffc03, 0xffc03fff, 0x0ffff0ff, 0xf3ff3f3f, 0xf3f0fc3f,
    0x0fc3f030, 0x3f0303f0, 0x003f0003, 0xf0003f00, 0x03000000,
    // [312]: 4E "N"
    0x000e1206, 0xc00f003c, 0x03f00fc0, 0xff03fc3f, 0xf0ffcfcf, 0x3f3ff0ff, 0xc3fc0ff0, 0x3f00fc03,
    0xc00f0030,
    // [321]: 4F "O"
    0x000c1206, 0x3fc3fcf0, 0xff0ff0ff, 0x0ff0ff0f, 0xf0ff0ff0, 0xff0ff0ff, 0x0ff0ff0f, 0x3fc3fc00,
    // [329]: 50 "P"
    0x000c1206, 0x3ff3fff0, 0xff0ff0ff, 0x0ff0ff0f, 0x3ff3ff00, 0xf00f00f0, 0x0f00f00f, 0x00f00f00,
    // [337]: 51 "Q"
    0x000c1606, 0x3fc3fcf0, 0xff0ff0ff, 0x0ff0ff0f, 0xf0ff0ff0, 0xff0ff0ff, 0x0ff0ff0f, 0x3fc3fc3c,
    0x03c0f00f, 0x00000000,
    // [347]: 52 "R"
    0x000c1206, 0x3ff3fff0, 0xff0ff0ff, 0x0ff0ff0f, 0x3ff3fff0, 0xff0ff0ff, 0x0ff0ff0f, 0xf0ff0f00,
    // [355]: 53 "S"
    0x000a1206, 0x3f0fcc3f, 0x0f03c0f0, 0xfc3f3f0f, 0xcfc3f0f0, 0x3c0f0fc3, 0x3f0fc000,
    // [362]: 54 "T"
    0x000c1206, 0xffffff0f, 0x00f00f00, 0xf00f00f0, 0x0f00f00f, 0x00f00f00, 0xf00f00f0, 0x0f00f000,
    // [370]: 55 "U"
    0x000c1206, 0xf0ff0ff0, 0xff0ff0ff, 0x0ff0ff0f, 0xf0ff0ff0, 0xff0ff0ff, 0x0ff0ff0f, 0x3fc3fc00,
    // [378]: 56 "V"
    0x000c1206, 0xf0ff0ff0, 0xff0ff0ff, 0x0ff0ff0f, 0xf0ff0ff0, 0xff0ff0ff, 0x0f3fc3fc, 0x0f00f000,
    // [386]: 57 "W"
    0x00141206, 0xf0f0ff0f, 0x0ff0f0ff, 0x0f0ff0f0, 0xff0f0ff0, 0xf0ff0f0f, 0xf0f0ff0f, 0x0ff0f0ff,
    0x0f0ff0f0, 0xff0f0f30, 0xf0c30f0c, 0x0f0f00f0, 0xf0000000,
    // [399]: 58 "X"
    0x000c1206, 0xf0ff0ff0, 0xff0ff0ff, 0x0f3fc3fc, 0x0f00f03f, 0xc3fcf0ff, 0x0ff0ff0f, 0xf0ff0f00,
    // [407]: 59 "Y"
    0x000c1206, 0xf0ff0ff0, 0xff0ff0ff, 0x0ff0ff0f, 0x3fc3fc0f, 0x00f00f00, 0xf00f00f0, 0x0f00f000,
    // [415]: 5A "Z"
    0x000c1206, 0xfffffff0, 0x0f00f00f, 0x003c03c0, 0x0f00f003, 0xc03c00f0, 0x0f00f00f, 0xffffff00,
    // [423]: 5B "["
    0x00061604, 0xfff3cf3c, 0xf3cf3cf3, 0xcf3cf3cf, 0x3cf3cfff, 0xf0000000,
    // [429]: 5C "\\"
    0x000a1404, 0x00c0300c, 0x030300c0, 0x300c0c03, 0x00c03030, 0x0c0300c0, 0xc0300c03, 0x00000000,
    // [437]: 5D "]"
    0x00061604, 0xffff3cf3, 0xcf3cf3cf, 0x3cf3cf3c, 0xf3cf3cff, 0xf0000000,
    // [443]: 5E "^"
    0x000a0606, 0x0c030330, 0xccc0f030,
    // [446]: 5F "_"
    0x00100216, 0xffffffff,
    // [448]: 60 "`"
    0x00060604, 0x0c330cc3, 0x00000000,
    // [451]: 61 "a"
    0x000c0e0a, 0x3fc3fcf0, 0x3f03ffcf, 0xfcf0ff0f, 0xf0ff0ff0, 0xff0fffcf, 0xfc000000,
    // [458]: 62 "b"
    0x000c1206, 0x00f00f00, 0xf00f3ff3, 0xfff0ff0f, 0xf0ff0ff0, 0xff0ff0ff, 0x0ff0ff0f, 0x3ff3ff00,
    // [466]: 63 "c"
    0x000a0e0a, 0x3f0fcc3f, 0x0f03c0f0, 0x3c0f03c0, 0xfc3f0f3f, 0x0fc00000,
    // [472]: 64 "d"
    0x000c1206, 0xf00f00f0, 0x0f00ffcf, 0xfcf0ff0f, 0xf0ff0ff0, 0xff0ff0ff, 0x0ff0ff0f, 0xffcffc00,
    // [480]: 65 "e"
    0x000c0e0a, 0x3fc3fcf0, 0xff0ff0ff, 0x0fffffff, 0x00f00fc0, 0xfc0f3fc3, 0xfc000000,
    // [487]: 66 "f"
    0x000a1206, 0xfc3f00f0, 0x3c3fcff0, 0xf03c0f03, 0xc0f03c0f, 0x03c0f03c, 0x0f03c000,
    // [494]: 67 "g"
    0x000c140a, 0xffcffcf0, 0xff0ff0ff, 0x0ff0ff0f, 0xf0ff0ff0, 0xff0fffcf, 0xfcf00f00, 0xf03f033f,
    0xc3fc0000,
    // [503]: 68 "h"
    0x000c1206, 0x00f00f00, 0xf00f3ff3, 0xfff0ff0f, 0xf0ff0ff0, 0xff0ff0ff, 0x0ff0ff0f, 0xf0ff0f00,
    // [511]: 69 "i"
    0x00041206, 0xff00ffff, 0xffffffff, 0xff000000,
    // [515]: 6A "j"
    0x000a1806, 0xf03c0000, 0x00f03c0f, 0x03c0f03c, 0x0f03c0f0, 0x3c0f03c0, 0xf03c0f03, 0xc0f0fc33,
    0xf0fc0000,
    // [524]: 6B "k"
    0x000c1206, 0x00f00f00, 0xf00ff0ff, 0x0f3cf3cf, 0x0ff0ff03, 0xf03f0ff0, 0xff3cf3cf, 0xf0ff0f00,
    // [532]: 6C "l"
    0x00041206, 0xffffffff, 0xffffffff, 0xff000000,
    // [536]: 6D "m"
    0x00140e0a, 0x3ffff3ff, 0xfff0f0ff, 0x0f0ff0f0, 0xff0f0ff0, 0xf0ff0f0f, 0xf0f0ff0f, 0x0ff0f0ff,
    0x0f0ff0f0, 0xff0f0f00,
    // [546]: 6E "n"
    0x000c0e0a, 0x3ff3fff0, 0xff0ff0ff, 0x0ff0ff0f, 0xf0ff0ff0, 0xff0ff0ff, 0x0f000000,
    // [553]: 6F "o"
    0x000c0e0a, 0x3fc3fcf0, 0xff0ff0ff, 0x0ff0ff0f, 0xf0ff0ff0, 0xff0f3fc3, 0xfc000000,
    // [560]: 70 "p"
    0x000c120a, 0x3ff3fff0, 0xff0ff0ff, 0x0ff0ff0f, 0xf0ff0ff0, 0xff0f3ff3, 0xff00f00f, 0x00f00f00,
    // [568]: 71 "q"
    0x000c120a, 0xffcffcf0, 0xff0ff0ff, 0x0ff0ff0f, 0xf0ff0ff0, 0xff0fffcf, 0xfcf00f00, 0xf00f0000,
    // [576]: 72 "r"
    0x000a0e0a, 0xf3fcf0fc, 0x3f03c0f0, 0x3c0f03c0, 0xf03c0f03, 0xc0f00000,
    // [582]: 73 "s"
    0x000a0e0a, 0x3f0fcc3f, 0x0f0fc3f3, 0xf0fcfc3f, 0x0f0fc33f, 0x0fc00000,
    // [588]: 74 "t"
    0x00081206, 0x3c3c3c3c, 0xffff3c3c, 0x3c3c3c3c, 0x3c3c3c3c, 0xf0f00000,
    // [594]: 75 "u"
    0x000c0e0a, 0xf0ff0ff0, 0xff0ff0ff, 0x0ff0ff0f, 0xf0ff0ffc, 0xffcff3cf, 0x3c000000,
    // [601]: 76 "v"
    0x000c0e0a, 0xf0ff0ff0, 0xff0ff0ff, 0x0ff0ff0f, 0xf0ff0f3f, 0xc3fc0f00, 0xf0000000,
    // [608]: 77 "w"
    0x00140e0a, 0xf0f0ff0f, 0x0ff0f0ff, 0x0f0ff0f0, 0xff0f0ff0, 0xf0ff0f0f, 0xf0f0ff0f, 0x0f30f0c3,
    0x0f0c0f0f, 0x00f0f000,
    // [618]: 78 "x"
    0x000c0e0a, 0xf0ff0ff0, 0xff0f3fc3, 0xfc0f00f0, 0x3fc3fcf0, 0xff0ff0ff, 0x0f000000,
    // [625]: 79 "y"
    0x000c140a, 0xf0ff0ff0, 0xff0ff0ff, 0x0ff0ff0f, 0xf0ff0ff0, 0xff0fffcf, 0xfcf00f00, 0xf03f033f,
    0xc3fc0000,
    // [634]: 7A "z"
    0x000c0e0a, 0xfffffff0, 0x0f003c03, 0xc00f00f0, 0x03c03c00, 0xf00fffff, 0xff000000,
    // [641]: 7B "{"
    0x00061604, 0xc3030c30, 0xc30c30c0, 0xc330c30c, 0x30c30cc3, 0x00000000,
    // [647]: 7C "|"
    0x00021604, 0xffffffff, 0xfff00000,
    // [650]: 7D "}"
    0x00061604, 0x0c330c30, 0xc30c30cc, 0x3030c30c, 0x30c30c0c, 0x30000000,
    // [656]: 7E "~"
    0x000c040a, 0xc3cc3c3c, 0x33c30000,
    // [659]: A0 "\u00A0" No-Break Space
    0x0004020e, 0x00000000,
    // [661]: A1 "¡"
    0x00041206, 0xffff00ff, 0xffffffff, 0xff000000,
    // [665]: A2 "¢"
    0x000a1004, 0x0c0303f0, 0xfcccf330, 0xcc330cc3, 0x3ccf333f, 0x0fc0c030,
    // [671]: A3 "£"
    0x000e1206, 0x0fc03f03, 0x0f0c3c00, 0xf003c00f, 0x003c03fc, 0x0ff00f00, 0x3c00f003, 0xcc0f303c,
    0x3ffcfff0,
    // [680]: A4 "¤"
    0x000e0e08, 0x400bbf77, 0xff8e1c70, 0x39806601, 0x98066019, 0xc0e3871f, 0xfeefdd00, 0x20000000,
    // [688]: A5 "¥"
    0x00101206, 0xf00ff00f, 0x3c3c3c3c, 0xffffffff, 0x03c003c0, 0xffffffff, 0x03c003c0, 0x03c003c0,
    0x03c003c0, 0x03c003c0,
    // [698]: A6 "¦"
    0x00021604, 0xfffff0ff, 0xfff00000,
    // [701]: A7 "§"
    0x000a1804, 0x3f0fcc0f, 0x0303c0f0, 0xf03c3ccf, 0x3f0fc3c3, 0xf0fcf33c, 0x3c0f0f03, 0xc0c0f033,
    0xf0fc0000,
    // [710]: A8 "¨"
    0x00080206, 0xc3c30000,
    // [712]: A9 "©"
    0x00101206, 0x0ff00ff0, 0x300c300c, 0xc3c3c3c3, 0xcc33cc33, 0xc033c033, 0xcc33cc33, 0xc3c3c3c3,
    0x300c300c, 0x0ff00ff0,
    // [722]: AA "ª"
    0x000a1006, 0x3f0fcf0f, 0xc3ff3fcf, 0x3fcff3fc, 0xfff3fc00, 0x000fffff,
    // [728]: AB "«"
    0x000e0e0a, 0xc3030c03, 0x0c0c300c, 0x3030c030, 0xc0c30c30, 0x30c30c0c, 0x30c3030c, 0x00000000,
    // [736]: AC "¬"
    0x000a060e, 0xfffffc03, 0x00c03000,
    // [739]: AD "\u00AD" Soft Hyphen
    0x000a020e, 0xfffff000,
    // [741]: AE "®"
    0x00101206, 0x0ff00ff0, 0x300c300c, 0xc3f3c3f3, 0xcc33cc33, 0xc3f3c3f3, 0xcc33cc33, 0xcc33cc33,
    0x300c300c, 0x0ff00ff0,
    // [751]: AF "¯"
    0x00080204, 0xffff0000,
    // [753]: B0 "°"
    0x00080806, 0x3c3cc3c3, 0xc3c33c3c,
    // [756]: B1 "±"
    0x000a0e0a, 0x0c0300c0, 0x30fffff0, 0xc0300c03, 0x000000ff, 0xfff00000,
    // [762]: B2 "²"
    0x00060a02, 0xfffc30ff, 0xf0c3fff0,
    // [765]: B3 "³"
    0x00060a02, 0xfffc30ff, 0xfc30fff0,
    // [768]: B4 "´"
    0x00060606, 0xc3030c0c, 0x30000000,
    // [771]: B5 "µ"
    0x0010120a, 0x3c3c3c3c, 0x3c3c3c3c, 0x3c3c3c3c, 0x3c3c3c3c, 0x3c3c3c3c, 0x3c3c3c3c, 0xcffccffc,
    0x000c000c, 0x00030003,
    // [781]: B6 "¶"
    0x000e1206, 0xfff3ffcc, 0xc3f30fcc, 0x3f30fcc3, 0xf30fcff3, 0x3fccc033, 0x00cc0330, 0x0cc03300,
    0xcc033000,
    // [790]: B7 "·"
    0x0004040e, 0xffff0000,
    // [792]: B8 "¸"
    0x00040618, 0xffcc3300,
    // [794]: B9 "¹"
    0x00060a02, 0x30c3cf30, 0xc30cfff0,
    // [797]: BA "º"
    0x000a1006, 0x3f0fcf3f, 0xcff3fcff, 0x3fcff3fc, 0xf3f0fc00, 0x000fffff,
    // [803]: BB "»"
    0x000e0e0a, 0x030c0c30, 0xc3030c30, 0xc0c30c30, 0x30c030c0, 0xc300c303, 0x0c030c0c, 0x30000000,
    // [811]: BC "¼"
    0x00121402, 0x0c030300, 0xc0c03c30, 0x0f030300, 0xc0c03030, 0x0c0c00cf, 0xc033fccc, 0x033300cc,
    0x30330c0f, 0xc303f0c0, 0xc00c3003, 0x0c00c300, 0x30000000,
    // [824]: BD "½"
    0x00121402, 0x0c030300, 0xc0c03c30, 0x0f030300, 0xc0c03030, 0x0c0c00cf, 0xc033ffcc, 0x03f300c0,
    0x30300c0f, 0xc303f0c0, 0x0c0c0303, 0x0fc0c3f0, 0x30000000,
    // [837]: BE "¾"
    0x00121402, 0x0c0fc303, 0xf0c0c030, 0x30030fc0, 0xc3f030c0, 0x0c3000cf, 0xc033fccc, 0x033300cc,
    0x30330c0f, 0xc303f0c0, 0xc00c3003, 0x0c00c300, 0x30000000,
    // [850]: BF "¿"
    0x000c1206, 0x0f00f00f, 0x00f00000, 0x000f00f0, 0x0f00f003, 0xc03c00f0, 0x0fc0fc0f, 0x3fc3fc00,
    // [858]: C0 "À"
    0x000c1800, 0x0300300c, 0x00c00000, 0x003fc3fc, 0xf0ff0ff0, 0xff0ff0ff, 0x0fffffff, 0xf0ff0ff0,
    0xff0ff0ff, 0x0ff0ff0f,
    // [868]: C1 "Á"
    0x000c1800, 0x0c00c003, 0x00300000, 0x003fc3fc, 0xf0ff0ff0, 0xff0ff0ff, 0x0fffffff, 0xf0ff0ff0,
    0xff0ff0ff, 0x0ff0ff0f,
    // [878]: C2 "Â"
    0x000c1800, 0x0f00f030, 0xc30c0000, 0x003fc3fc, 0xf0ff0ff0, 0xff0ff0ff, 0x0fffffff, 0xf0ff0ff0,
    0xff0ff0ff, 0x0ff0ff0f,
    // [888]: C3 "Ã"
    0x000c1800, 0xc3cc3c3c, 0x33c30000, 0x003fc3fc, 0xf0ff0ff0, 0xff0ff0ff, 0x0fffffff, 0xf0ff0ff0,
    0xff0ff0ff, 0x0ff0ff0f,
    // [898]: C4 "Ä"
    0x000c1602, 0x30c30c00, 0x00003fc3, 0xfcf0ff0f, 0xf0ff0ff0, 0xff0fffff, 0xfff0ff0f, 0xf0ff0ff0,
    0xff0ff0ff, 0x0f000000,
    // [908]: C5 "Å"
    0x000c1800, 0x0f00f030, 0xc30c30c3, 0x0c3fc3fc, 0xf0ff0ff0, 0xff0ff0ff, 0x0fffffff, 0xf0ff0ff0,
    0xff0ff0ff, 0x0ff0ff0f,
    // [918]: C6 "Æ"
    0x00121206, 0xffff3fff, 0xc03c3c0f, 0x0f03c3c0, 0xf0f03c3c, 0x0f0f3fff, 0xcffff03c, 0x3c0f0f03,
    0xc3c0f0f0, 0x3c3c0f0f, 0xffc3fff0, 0xf0000000,
    // [930]: C7 "Ç"
    0x000c1806, 0x3fc3fcc0, 0xfc0f00f0, 0x0f00f00f, 0x00f00f00, 0xf00f00f0, 0x0fc0fc0f, 0x3fc3fc0f,
    0x00f00c00, 0xc0030030,
    // [940]: C8 "È"
    0x000a1800, 0x0300c0c0, 0x3000000f, 0xffff03c0, 0xf03c0f03, 0xc0f3fcff, 0x03c0f03c, 0x0f03c0ff,
    0xffff0000,
    // [949]: C9 "É"
    0x000a1800, 0x300c00c0, 0x3000000f, 0xffff03c0, 0xf03c0f03, 0xc0f3fcff, 0x03c0f03c, 0x0f03c0ff,
    0xffff0000,
    // [958]: CA "Ê"
    0x000a1800, 0x0f03c30c, 0xc300000f, 0xffff03c0, 0xf03c0f03, 0xc0f3fcff, 0x03c0f03c, 0x0f03c0ff,
    0xffff0000,
    // [967]: CB "Ë"
    0x000a1602, 0x330cc000, 0x00fffff0, 0x3c0f03c0, 0xf03c0f3f, 0xcff03c0f, 0x03c0f03c, 0x0ffffff0,
    // [975]: CC "Ì"
    0x00041800, 0x33cc00ff, 0xffffffff, 0xffffffff,
    // [979]: CD "Í"
    0x00041800, 0xcc3300ff, 0xffffffff, 0xffffffff,
    // [983]: CE "Î"
    0x00081800, 0x3c3cc3c3, 0x00003c3c, 0x3c3c3c3c, 0x3c3c3c3c, 0x3c3c3c3c, 0x3c3c3c3c,
    // [990]: CF "Ï"
    0x00081602, 0xc3c30000, 0x3c3c3c3c, 0x3c3c3c3c, 0x3c3c3c3c, 0x3c3c3c3c, 0x3c3c0000,
    // [997]: D0 "Ð"
    0x000e1206, 0x3ff0ffcf, 0x0f3c3cf0, 0xf3c3cf0f, 0x3c3cf3ff, 0xcfff0f3c, 0x3cf0f3c3, 0xcf0f3c3c,
    0x3ff0ffc0,
    // [1006]: D1 "Ñ"
    0x000e1800, 0x30f0c3c0, 0xf0c3c300, 0x00000c00, 0xf003c03f, 0x00fc0ff0, 0x3fc3ff0f, 0xfcfcf3f3,
    0xff0ffc3f, 0xc0ff03f0, 0x0fc03c00, 0xf0030000,
    // [1018]: D2 "Ò"
    0x000c1800, 0x0300300c, 0x00c00000, 0x003fc3fc, 0xf0ff0ff0, 0xff0ff0ff, 0x0ff0ff0f, 0xf0ff0ff0,
    0xff0ff0ff, 0x0f3fc3fc,
    // [1028]: D3 "Ó"
    0x000c1800, 0x0c00c003, 0x00300000, 0x003fc3fc, 0xf0ff0ff0, 0xff0ff0ff, 0x0ff0ff0f, 0xf0ff0ff0,
    0xff0ff0ff, 0x0f3fc3fc,
    // [1038]: D4 "Ô"
    0x000c1800, 0x0f00f030, 0xc30c0000, 0x003fc3fc, 0xf0ff0ff0, 0xff0ff0ff, 0x0ff0ff0f, 0xf0ff0ff0,
    0xff0ff0ff, 0x0f3fc3fc,
    // [1048]: D5 "Õ"
    0x000c1800, 0xc3cc3c3c, 0x33c30000, 0x003fc3fc, 0xf0ff0ff0, 0xff0ff0ff, 0x0ff0ff0f, 0xf0ff0ff0,
    0xff0ff0ff, 0x0f3fc3fc,
    // [1058]: D6 "Ö"
    0x000c1602, 0x30c30c00, 0x00003fc3, 0xfcf0ff0f, 0xf0ff0ff0, 0xff0ff0ff, 0x0ff0ff0f, 0xf0ff0ff0,
    0xff0f3fc3, 0xfc000000,
    // [1068]: D7 "×"
    0x000a0a0e, 0xc0f03330, 0xcc0c0303, 0x30ccc0f0, 0x30000000,
    // [1073]: D8 "Ø"
    0x00121206, 0xcff033fc, 0x03c3c0f0, 0xf03c3c0f, 0x0f03f3c0, 0xfcf03cfc, 0x0f3f03c3, 0xc0f0f03c,
    0x3c0f0f03, 0xc3f0f0fc, 0x0ff0c3fc, 0x30000000,
    // [1085]: D9 "Ù"
    0x000c1800, 0x0300300c, 0x00c00000, 0x00f0ff0f, 0xf0ff0ff0, 0xff0ff0ff, 0x0ff0ff0f, 0xf0ff0ff0,
    0xff0ff0ff, 0x0f3fc3fc,
    // [1095]: DA "Ú"
    0x000c1800, 0x0c00c003, 0x00300000, 0x00f0ff0f, 0xf0ff0ff0, 0xff0ff0ff, 0x0ff0ff0f, 0xf0ff0ff0,
    0xff0ff0ff, 0x0f3fc3fc,
    // [1105]: DB "Û"
    0x000c1800, 0x0f00f030, 0xc30c0000, 0x00f0ff0f, 0xf0ff0ff0, 0xff0ff0ff, 0x0ff0ff0f, 0xf0ff0ff0,
    0xff0ff0ff, 0x0f3fc3fc,
    // [1115]: DC "Ü"
    0x000c1602, 0x30c30c00, 0x0000f0ff, 0x0ff0ff0f, 0xf0ff0ff0, 0xff0ff0ff, 0x0ff0ff0f, 0xf0ff0ff0,
    0xff0f3fc3, 0xfc000000,
    // [1125]: DD "Ý"
    0x000c1800, 0x0c00c003, 0x00300000, 0x00f0ff0f, 0xf0ff0ff0, 0xff0ff0ff, 0x0f3fc3fc, 0x0f00f00f,
    0x00f00f00, 0xf00f00f0,
    // [1135]: DE "Þ"
    0x000e1206, 0x03fc0ff0, 0x0f003c3f, 0xf0ffcf0f, 0x3c3cf0f3, 0xc3cf0f3c, 0x3c3ff0ff, 0xc00f003c,
    0x03fc0ff0,
    // [1144]: DF "ß"
    0x000e1206, 0x0ff03fc3, 0xc3cf0f3c, 0x3cf0f0f3, 0xc3cf3c3c, 0xf0ff03fc, 0x0ff03fc0, 0xff03fc0f,
    0x3f3cfcf0,
    // [1153]: E0 "à"
    0x000c1404, 0x0300300c, 0x00c00000, 0x003fc3fc, 0xf03f03ff, 0xcffcf0ff, 0x0ff0ff0f, 0xf0ff0fff,
    0xcffc0000,
    // [1162]: E1 "á"
    0x000c1404, 0x0c00c003, 0x00300000, 0x003fc3fc, 0xf03f03ff, 0xcffcf0ff, 0x0ff0ff0f, 0xf0ff0fff,
    0xcffc0000,
    // [1171]: E2 "â"
    0x000c1404, 0x0f00f030, 0xc30c0000, 0x003fc3fc, 0xf03f03ff, 0xcffcf0ff, 0x0ff0ff0f, 0xf0ff0fff,
    0xcffc0000,
    // [1180]: E3 "ã"
    0x000c1404, 0xc3cc3c3c, 0x33c30000, 0x003fc3fc, 0xf03f03ff, 0xcffcf0ff, 0x0ff0ff0f, 0xf0ff0fff,
    0xcffc0000,
    // [1189]: E4 "ä"
    0x000c1206, 0x30c30c00, 0x00003fc3, 0xfcf03f03, 0xffcffcf0, 0xff0ff0ff, 0x0ff0ff0f, 0xffcffc00,
    // [1197]: E5 "å"
    0x000c1800, 0x0f00f030, 0xc30c30c3, 0x0c0f00f0, 0x0000003f, 0xc3fcf03f, 0x03ffcffc, 0xf0ff0ff0,
    0xff0ff0ff, 0x0fffcffc,
    // [1207]: E6 "æ"
    0x00140e0a, 0x3fffc3ff, 0xfcf0f03f, 0x0f03f0ff, 0xcf0ffcff, 0xf0ffff0f, 0x00f0f00f, 0x0fc0f0fc,
    0x0f0f3fff, 0xc3fffc00,
    // [1217]: E7 "ç"
    0x000c140a, 0x3fc3fcc0, 0xfc0f00f0, 0x0f00f00f, 0x00f00fc0, 0xfc0f3fc3, 0xfc0f00f0, 0x0c00c003,
    0x00300000,
    // [1226]: E8 "è"
    0x000c1404, 0x0300300c, 0x00c00000, 0x003fc3fc, 0xf0ff0ff0, 0xff0fffff, 0xff00f00f, 0xc0fc0f3f,
    0xc3fc0000,
    // [1235]: E9 "é"
    0x000c1404, 0x0c00c003, 0x00300000, 0x003fc3fc, 0xf0ff0ff0, 0xff0fffff, 0xff00f00f, 0xc0fc0f3f,
    0xc3fc0000,
    // [1244]: EA "ê"
    0x000c1404, 0x0f00f030, 0xc30c0000, 0x003fc3fc, 0xf0ff0ff0, 0xff0fffff, 0xff00f00f, 0xc0fc0f3f,
    0xc3fc0000,
    // [1253]: EB "ë"
    0x000c1206, 0x30c30c00, 0x00003fc3, 0xfcf0ff0f, 0xf0ff0fff, 0xffff00f0, 0x0fc0fc0f, 0x3fc3fc00,
    // [1261]: EC "ì"
    0x00041404, 0x33cc00ff, 0xffffffff, 0xffff0000,
    // [1265]: ED "í"
    0x00041404, 0xcc3300ff, 0xffffffff, 0xffff0000,
    // [1269]: EE "î"
    0x00081404, 0x3c3cc3c3, 0x00003c3c, 0x3c3c3c3c, 0x3c3c3c3c, 0x3c3c3c3c,
    // [1275]: EF "ï"
    0x00081206, 0xc3c30000, 0x3c3c3c3c, 0x3c3c3c3c, 0x3c3c3c3c, 0x3c3c0000,
    // [1281]: F0 "ð"
    0x000c1206, 0x03f73f3f, 0x00fc3ce3, 0xc0ff0ff0, 0xf3cf3cf0, 0xff0ff0ff, 0x0f30f30f, 0x0fc0fc00,
    // [1289]: F1 "ñ"
    0x000c1404, 0xc3cc3c3c, 0x33c30000, 0x003ff3ff, 0xf0ff0ff0, 0xff0ff0ff, 0x0ff0ff0f, 0xf0ff0ff0,
    0xff0f0000,
    // [1298]: F2 "ò"
    0x000c1404, 0x0300300c, 0x00c00000, 0x003fc3fc, 0xf0ff0ff0, 0xff0ff0ff, 0x0ff0ff0f, 0xf0ff0f3f,
    0xc3fc0000,
    // [1307]: F3 "ó"
    0x000c1404, 0x0c00c003, 0x00300000, 0x003fc3fc, 0xf0ff0ff0, 0xff0ff0ff, 0x0ff0ff0f, 0xf0ff0f3f,
    0xc3fc0000,
    // [1316]: F4 "ô"
    0x000c1404, 0x0f00f030, 0xc30c0000, 0x003fc3fc, 0xf0ff0ff0, 0xff0ff0ff, 0x0ff0ff0f, 0xf0ff0f3f,
    0xc3fc0000,
    // [1325]: F5 "õ"
    0x000c1404, 0xc3cc3c3c, 0x33c30000, 0x003fc3fc, 0xf0ff0ff0, 0xff0ff0ff, 0x0ff0ff0f, 0xf0ff0f3f,
    0xc3fc0000,
    // [1334]: F6 "ö"
    0x000c1206, 0x30c30c00, 0x00003fc3, 0xfcf0ff0f, 0xf0ff0ff0, 0xff0ff0ff, 0x0ff0ff0f, 0x3fc3fc00,
    // [1342]: F7 "÷"
    0x000a0a0a, 0x0c030000, 0x00fffff0, 0x00000c03, 0x00000000,
    // [1347]: F8 "ø"
    0x00100e0a, 0xcff0cff0, 0x3c3c3c3c, 0x3c3c3c3c, 0x3f3c3f3c, 0x3cfc3cfc, 0x3c3c3c3c, 0x0ff30ff3,
    // [1355]: F9 "ù"
    0x000c1404, 0x0300300c, 0x00c00000, 0x00f0ff0f, 0xf0ff0ff0, 0xff0ff0ff, 0x0ff0ff0f, 0xfcffcff3,
    0xcf3c0000,
    // [1364]: FA "ú"
    0x000c1404, 0x0c00c003, 0x00300000, 0x00f0ff0f, 0xf0ff0ff0, 0xff0ff0ff, 0x0ff0ff0f, 0xfcffcff3,
    0xcf3c0000,
    // [1373]: FB "û"
    0x000c1404, 0x0f00f030, 0xc30c0000, 0x00f0ff0f, 0xf0ff0ff0, 0xff0ff0ff, 0x0ff0ff0f, 0xfcffcff3,
    0xcf3c0000,
    // [1382]: FC "ü"
    0x000c1206, 0x30c30c00, 0x0000f0ff, 0x0ff0ff0f, 0xf0ff0ff0, 0xff0ff0ff, 0x0ffcffcf, 0xf3cf3c00,
    // [1390]: FD "ý"
    0x000c1a04, 0x0c00c003, 0x00300000, 0x00f0ff0f, 0xf0ff0ff0, 0xff0ff0ff, 0x0ff0ff0f, 0xf0ff0fff,
    0xcffcf00f, 0x00f03f03, 0x3fc3fc00,
    // [1401]: FE "þ"
    0x000c1606, 0x00f00f00, 0xf00f3ff3, 0xfff0ff0f, 0xf0ff0ff0, 0xff0ff0ff, 0x0ff0ff0f, 0x3ff3ff00,
    0xf00f00f0, 0x0f000000,
    // [1411]: FF "ÿ"
    0x000c1806, 0x30c30c00, 0x0000f0ff, 0x0ff0ff0f, 0xf0ff0ff0, 0xff0ff0ff, 0x0ff0ff0f, 0xffcffcf0,
    0x0f00f03f, 0x033fc3fc,
    // [1421]: 152 "Œ"
    0x00121206, 0xffff3fff, 0xc03c3c0f, 0x0f03c3c0, 0xf0f03c3c, 0x0f0f3fc3, 0xcff0f03c, 0x3c0f0f03,
    0xc3c0f0f0, 0x3c3c0f0f, 0xffff3fff, 0xc0000000,
    // [1433]: 153 "œ"
    0x00140e0a, 0x3fffc3ff, 0xfcf0f0ff, 0x0f0ff0f0, 0xff0f0fff, 0xf0ffff0f, 0x00f0f00f, 0x0fc0f0fc,
    0x0f0f3fff, 0xc3fffc00,
    // [1443]: 2018 "‘"
    0x00040806, 0xcc33ffff,
    // [1445]: 2019 "’"
    0x00040806, 0xffffcc33,
    // [1447]: 201A "‚"
    0x00040814, 0xffffcc33,
    // [1449]: 201B "‛"
    0x00040806, 0xffff33cc,
    // [1451]: 201C "“"
    0x000a0806, 0xc330c30c, 0xc3f3fcff, 0x3fcf0000,
    // [1455]: 201D "”"
    0x000a0806, 0xf3fcff3f, 0xcfc330c3, 0x0cc30000,
    // [1459]: 201E "„"
    0x000a0814, 0xf3fcff3f, 0xcfc330c3, 0x0cc30000,
    // [1463]: 201F "‟"
    0x000a0806, 0xf3fcff3f, 0xcf30cc3c, 0x330c0000,
    // [1467]: 2020 "†"
    0x00060a06, 0x30cfff30, 0xc30c30c0,
    // [1470]: 2021 "‡"
    0x00060c06, 0x30cfff30, 0xc30cfff3, 0x0c000000,
    // [1474]: 2022 "•"
    0x000a0a0a, 0x3f0fcfff, 0xffffffff, 0xffff3f0f, 0xc0000000,
    // [1479]: 20AC "€"
    0x00101008, 0x3fc03fc0, 0xc030c030, 0x000c000c, 0x3fff3fff, 0x000c000c, 0x0fff0fff, 0xc030c030,
    0x3fc03fc0,
    // [1488]: FFFD "�"
    0x00121404, 0x00c00030, 0x003f000f, 0xc00f3c03, 0xcf03ccf0, 0xf33cfcff, 0xff3ffff3, 0xfffcff3f,
    0xff0fffc0, 0xf3c03cf0, 0x03f000fc, 0x000c0003, 0x00000000,
];
