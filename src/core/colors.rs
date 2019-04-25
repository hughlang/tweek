/// This is a helper struct to provide easy access to a lot of RGB colors
/// It may be removed in the future.

/// Color chart from here: https://blogs.msdn.microsoft.com/smallbasic/2015/06/20/the-hex-colors-in-small-basic/
pub struct HexColors {}

#[allow(dead_code)]
#[allow(non_upper_case_globals)]
impl HexColors {
    // Red Colors
    pub const IndianRed: u32 = 0xCD5C5C;
    pub const LightCoral: u32 = 0xF08080;
    pub const Salmon: u32 = 0xFA8072;
    pub const DarkSalmon: u32 = 0xE9967A;
    pub const LightSalmon: u32 = 0xFFA07A;
    pub const Crimson: u32 = 0xDC143C;
    pub const Red: u32 = 0xFF0000;
    pub const FireBrick: u32 = 0xB22222;
    pub const DarkRed: u32 = 0x8B0000;
    // Pink Colors
    pub const Pink: u32 = 0xFFC0CB;
    pub const LightPink: u32 = 0xFFB6C1;
    pub const HotPink: u32 = 0xFF69B4;
    pub const DeepPink: u32 = 0xFF1493;
    pub const MediumVioletRed: u32 = 0xC71585;
    pub const PaleVioletRed: u32 = 0xDB7093;
    // Orange Colors
    pub const Coral: u32 = 0xFF7F50;
    pub const Tomato: u32 = 0xFF6347;
    pub const OrangeRed: u32 = 0xFF4500;
    pub const DarkOrange: u32 = 0xFF8C00;
    pub const Orange: u32 = 0xFFA500;
    // Yellow Colors;
    pub const Gold: u32 = 0xFFD700;
    pub const Yellow: u32 = 0xFFFF00;
    pub const LightYellow: u32 = 0xFFFFE0;
    pub const LemonChiffon: u32 = 0xFFFACD;
    pub const LightGoldenrodYellow: u32 = 0xFAFAD2;
    pub const PapayaWhip: u32 = 0xFFEFD5;
    pub const Moccasin: u32 = 0xFFE4B5;
    pub const PeachPuff: u32 = 0xFFDAB9;
    pub const PaleGoldenrod: u32 = 0xEEE8AA;
    pub const Khaki: u32 = 0xF0E68C;
    pub const DarkKhaki: u32 = 0xBDB76B;
    // Purple Colors;
    pub const Lavender: u32 = 0xE6E6FA;
    pub const Thistle: u32 = 0xD8BFD8;
    pub const Plum: u32 = 0xDDA0DD;
    pub const Violet: u32 = 0xEE82EE;
    pub const Orchid: u32 = 0xDA70D6;
    pub const Fuchsia: u32 = 0xFF00FF;
    pub const Magenta: u32 = 0xFF00FF;
    pub const MediumOrchid: u32 = 0xBA55D3;
    pub const MediumPurple: u32 = 0x9370DB;
    pub const BlueViolet: u32 = 0x8A2BE2;
    pub const DarkViolet: u32 = 0x9400D3;
    pub const DarkOrchid: u32 = 0x9932CC;
    pub const DarkMagenta: u32 = 0x8B008B;
    pub const Purple: u32 = 0x800080;
    pub const Indigo: u32 = 0x4B0082;
    pub const SlateBlue: u32 = 0x6A5ACD;
    pub const MediumSlateBlue: u32 = 0x7B68EE;
    pub const DarkSlateBlue: u32 = 0x483D8B;
    // Green Colors;
    pub const GreenYellow: u32 = 0xADFF2F;
    pub const Chartreuse: u32 = 0x7FFF00;
    pub const LawnGreen: u32 = 0x7CFC00;
    pub const Lime: u32 = 0x00FF00;
    pub const LimeGreen: u32 = 0x32CD32;
    pub const PaleGreen: u32 = 0x98FB98;
    pub const LightGreen: u32 = 0x90EE90;
    pub const MediumSpringGreen: u32 = 0x00FA9A;
    pub const SpringGreen: u32 = 0x00FF7F;
    pub const MediumSeaGreen: u32 = 0x3CB371;
    pub const SeaGreen: u32 = 0x2E8B57;
    pub const ForestGreen: u32 = 0x228B22;
    pub const Green: u32 = 0x008000;
    pub const DarkGreen: u32 = 0x006400;
    pub const YellowGreen: u32 = 0x9ACD32;
    pub const OliveDrab: u32 = 0x6B8E23;
    pub const Olive: u32 = 0x808000;
    pub const DarkOliveGreen: u32 = 0x556B2F;
    pub const MediumAquamarine: u32 = 0x66CDAA;
    pub const DarkSeaGreen: u32 = 0x8FBC8F;
    pub const LightSeaGreen: u32 = 0x20B2AA;
    pub const DarkCyan: u32 = 0x008B8B;
    pub const Teal: u32 = 0x008080;
    // Blue Colors;
    pub const Aqua: u32 = 0x00FFFF;
    pub const Cyan: u32 = 0x00FFFF;
    pub const LightCyan: u32 = 0xE0FFFF;
    pub const PaleTurquoise: u32 = 0xAFEEEE;
    pub const Aquamarine: u32 = 0x7FFFD4;
    pub const Turquoise: u32 = 0x40E0D0;
    pub const MediumTurquoise: u32 = 0x48D1CC;
    pub const DarkTurquoise: u32 = 0x00CED1;
    pub const CadetBlue: u32 = 0x5F9EA0;
    pub const SteelBlue: u32 = 0x4682B4;
    pub const LightSteelBlue: u32 = 0xB0C4DE;
    pub const PowderBlue: u32 = 0xB0E0E6;
    pub const LightBlue: u32 = 0xADD8E6;
    pub const SkyBlue: u32 = 0x87CEEB;
    pub const LightSkyBlue: u32 = 0x87CEFA;
    pub const DeepSkyBlue: u32 = 0x00BFFF;
    pub const DodgerBlue: u32 = 0x1E90FF;
    pub const CornflowerBlue: u32 = 0x6495ED;
    pub const RoyalBlue: u32 = 0x4169E1;
    pub const Blue: u32 = 0x0000FF;
    pub const MediumBlue: u32 = 0x0000CD;
    pub const DarkBlue: u32 = 0x00008B;
    pub const Navy: u32 = 0x000080;
    pub const MidnightBlue: u32 = 0x191970;
    // Brown Colors;
    pub const Cornsilk: u32 = 0xFFF8DC;
    pub const BlanchedAlmond: u32 = 0xFFEBCD;
    pub const Bisque: u32 = 0xFFE4C4;
    pub const NavajoWhite: u32 = 0xFFDEAD;
    pub const Wheat: u32 = 0xF5DEB3;
    pub const BurlyWood: u32 = 0xDEB887;
    pub const Tan: u32 = 0xD2B48C;
    pub const RosyBrown: u32 = 0xBC8F8F;
    pub const SandyBrown: u32 = 0xF4A460;
    pub const Goldenrod: u32 = 0xDAA520;
    pub const DarkGoldenrod: u32 = 0xB8860B;
    pub const Peru: u32 = 0xCD853F;
    pub const Chocolate: u32 = 0xD2691E;
    pub const SaddleBrown: u32 = 0x8B4513;
    pub const Sienna: u32 = 0xA0522D;
    pub const Brown: u32 = 0xA52A2A;
    pub const Maroon: u32 = 0x800000;
    // White Colors;
    pub const White: u32 = 0xFFFFFF;
    pub const Snow: u32 = 0xFFFAFA;
    pub const Honeydew: u32 = 0xF0FFF0;
    pub const MintCream: u32 = 0xF5FFFA;
    pub const Azure: u32 = 0xF0FFFF;
    pub const AliceBlue: u32 = 0xF0F8FF;
    pub const GhostWhite: u32 = 0xF8F8FF;
    pub const WhiteSmoke: u32 = 0xF5F5F5;
    pub const Seashell: u32 = 0xFFF5EE;
    pub const Beige: u32 = 0xF5F5DC;
    pub const OldLace: u32 = 0xFDF5E6;
    pub const FloralWhite: u32 = 0xFFFAF0;
    pub const Ivory: u32 = 0xFFFFF0;
    pub const AntiqueWhite: u32 = 0xFAEBD7;
    pub const Linen: u32 = 0xFAF0E6;
    pub const LavenderBlush: u32 = 0xFFF0F5;
    pub const MistyRose: u32 = 0xFFE4E1;
    // Gray Colors;
    pub const Gainsboro: u32 = 0xDCDCDC;
    pub const LightGray: u32 = 0xD3D3D3;
    pub const Silver: u32 = 0xC0C0C0;
    pub const DarkGray: u32 = 0xA9A9A9;
    pub const Gray: u32 = 0x808080;
    pub const DimGray: u32 = 0x696969;
    pub const LightSlateGray: u32 = 0x778899;
    pub const SlateGray: u32 = 0x708090;
    pub const DarkSlateGray: u32 = 0x2F4F4F;
    pub const Black: u32 = 0x000000;
}

pub fn hex_to_rgb(c: u32) -> (u8, u8, u8) {
    let rp = ((c & 0x00FF_0000u32) >> 16) as u8;
    let gp = ((c & 0x0000_FF00u32) >> 8) as u8;
    let bp = (c & 0x0000_00FFu32) as u8;
    (rp, gp, bp)
}
