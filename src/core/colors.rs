/// This is a helper struct to provide easy access to a lot of RGB colors.
/// Really, I'm not sure if this whole thing is loaded into memory, which makes this a bad idea.
/// It may be removed in the future.
/// Originally, this codebase used u32 values for the colors in the format 0xFFFFFF which has a lesser
/// impact. However, the Quicksilver code seems to prefer string hex values and VSCode has extensions
/// that show the color.

/// Color chart from here: https://blogs.msdn.microsoft.com/smallbasic/2015/06/20/the-hex-colors-in-small-basic/

#[allow(dead_code)]
#[allow(non_upper_case_globals)]
pub struct HexColors {}

#[allow(dead_code)]
#[allow(missing_docs)]
#[allow(non_upper_case_globals)]
impl HexColors {
    // Red Colors
    pub const IndianRed: &'static str = "#CD5C5C";
    pub const LightCoral: &'static str = "#F08080";
    pub const Salmon: &'static str = "#FA8072";
    pub const DarkSalmon: &'static str = "#E9967A";
    pub const LightSalmon: &'static str = "#FFA07A";
    pub const Crimson: &'static str = "#DC143C";
    pub const Red: &'static str = "#FF0000";
    pub const FireBrick: &'static str = "#B22222";
    pub const DarkRed: &'static str = "#8B0000";
    // Pink Colors
    pub const Pink: &'static str = "#FFC0CB";
    pub const LightPink: &'static str = "#FFB6C1";
    pub const HotPink: &'static str = "#FF69B4";
    pub const DeepPink: &'static str = "#FF1493";
    pub const MediumVioletRed: &'static str = "#C71585";
    pub const PaleVioletRed: &'static str = "#DB7093";
    // Orange Colors
    pub const Coral: &'static str = "#FF7F50";
    pub const Tomato: &'static str = "#FF6347";
    pub const OrangeRed: &'static str = "#FF4500";
    pub const DarkOrange: &'static str = "#FF8C00";
    pub const Orange: &'static str = "#FFA500";
    // Yellow Colors;
    pub const Gold: &'static str = "#FFD700";
    pub const Yellow: &'static str = "#FFFF00";
    pub const LightYellow: &'static str = "#FFFFE0";
    pub const LemonChiffon: &'static str = "#FFFACD";
    pub const LightGoldenrodYellow: &'static str = "#FAFAD2";
    pub const PapayaWhip: &'static str = "#FFEFD5";
    pub const Moccasin: &'static str = "#FFE4B5";
    pub const PeachPuff: &'static str = "#FFDAB9";
    pub const PaleGoldenrod: &'static str = "#EEE8AA";
    pub const Khaki: &'static str = "#F0E68C";
    pub const DarkKhaki: &'static str = "#BDB76B";
    // Purple Colors;
    pub const Lavender: &'static str = "#E6E6FA";
    pub const Thistle: &'static str = "#D8BFD8";
    pub const Plum: &'static str = "#DDA0DD";
    pub const Violet: &'static str = "#EE82EE";
    pub const Orchid: &'static str = "#DA70D6";
    pub const Fuchsia: &'static str = "#FF00FF";
    pub const Magenta: &'static str = "#FF00FF";
    pub const MediumOrchid: &'static str = "#BA55D3";
    pub const MediumPurple: &'static str = "#9370DB";
    pub const BlueViolet: &'static str = "#8A2BE2";
    pub const DarkViolet: &'static str = "#9400D3";
    pub const DarkOrchid: &'static str = "#9932CC";
    pub const DarkMagenta: &'static str = "#8B008B";
    pub const Purple: &'static str = "#800080";
    pub const Indigo: &'static str = "#4B0082";
    pub const SlateBlue: &'static str = "#6A5ACD";
    pub const MediumSlateBlue: &'static str = "#7B68EE";
    pub const DarkSlateBlue: &'static str = "#483D8B";
    // Green Colors;
    pub const GreenYellow: &'static str = "#ADFF2F";
    pub const Chartreuse: &'static str = "#7FFF00";
    pub const LawnGreen: &'static str = "#7CFC00";
    pub const Lime: &'static str = "#00FF00";
    pub const LimeGreen: &'static str = "#32CD32";
    pub const PaleGreen: &'static str = "#98FB98";
    pub const LightGreen: &'static str = "#90EE90";
    pub const MediumSpringGreen: &'static str = "#00FA9A";
    pub const SpringGreen: &'static str = "#00FF7F";
    pub const MediumSeaGreen: &'static str = "#3CB371";
    pub const SeaGreen: &'static str = "#2E8B57";
    pub const ForestGreen: &'static str = "#228B22";
    pub const Green: &'static str = "#008000";
    pub const DarkGreen: &'static str = "#006400";
    pub const YellowGreen: &'static str = "#9ACD32";
    pub const OliveDrab: &'static str = "#6B8E23";
    pub const Olive: &'static str = "#808000";
    pub const DarkOliveGreen: &'static str = "#556B2F";
    pub const MediumAquamarine: &'static str = "#66CDAA";
    pub const DarkSeaGreen: &'static str = "#8FBC8F";
    pub const LightSeaGreen: &'static str = "#20B2AA";
    pub const DarkCyan: &'static str = "#008B8B";
    pub const Teal: &'static str = "#008080";
    // Blue Colors;
    pub const Aqua: &'static str = "#00FFFF";
    pub const Cyan: &'static str = "#00FFFF";
    pub const LightCyan: &'static str = "#E0FFFF";
    pub const PaleTurquoise: &'static str = "#AFEEEE";
    pub const Aquamarine: &'static str = "#7FFFD4";
    pub const Turquoise: &'static str = "#40E0D0";
    pub const MediumTurquoise: &'static str = "#48D1CC";
    pub const DarkTurquoise: &'static str = "#00CED1";
    pub const CadetBlue: &'static str = "#5F9EA0";
    pub const SteelBlue: &'static str = "#4682B4";
    pub const LightSteelBlue: &'static str = "#B0C4DE";
    pub const PowderBlue: &'static str = "#B0E0E6";
    pub const LightBlue: &'static str = "#ADD8E6";
    pub const SkyBlue: &'static str = "#87CEEB";
    pub const LightSkyBlue: &'static str = "#87CEFA";
    pub const DeepSkyBlue: &'static str = "#00BFFF";
    pub const DodgerBlue: &'static str = "#1E90FF";
    pub const CornflowerBlue: &'static str = "#6495ED";
    pub const RoyalBlue: &'static str = "#4169E1";
    pub const Blue: &'static str = "#0000FF";
    pub const MediumBlue: &'static str = "#0000CD";
    pub const DarkBlue: &'static str = "#00008B";
    pub const Navy: &'static str = "#000080";
    pub const MidnightBlue: &'static str = "#191970";
    // Brown Colors;
    pub const Cornsilk: &'static str = "#FFF8DC";
    pub const BlanchedAlmond: &'static str = "#FFEBCD";
    pub const Bisque: &'static str = "#FFE4C4";
    pub const NavajoWhite: &'static str = "#FFDEAD";
    pub const Wheat: &'static str = "#F5DEB3";
    pub const BurlyWood: &'static str = "#DEB887";
    pub const Tan: &'static str = "#D2B48C";
    pub const RosyBrown: &'static str = "#BC8F8F";
    pub const SandyBrown: &'static str = "#F4A460";
    pub const Goldenrod: &'static str = "#DAA520";
    pub const DarkGoldenrod: &'static str = "#B8860B";
    pub const Peru: &'static str = "#CD853F";
    pub const Chocolate: &'static str = "#D2691E";
    pub const SaddleBrown: &'static str = "#8B4513";
    pub const Sienna: &'static str = "#A0522D";
    pub const Brown: &'static str = "#A52A2A";
    pub const Maroon: &'static str = "#800000";
    // White Colors;
    pub const White: &'static str = "#FFFFFF";
    pub const Snow: &'static str = "#FFFAFA";
    pub const Honeydew: &'static str = "#F0FFF0";
    pub const MintCream: &'static str = "#F5FFFA";
    pub const Azure: &'static str = "#F0FFFF";
    pub const AliceBlue: &'static str = "#F0F8FF";
    pub const GhostWhite: &'static str = "#F8F8FF";
    pub const WhiteSmoke: &'static str = "#F5F5F5";
    pub const Seashell: &'static str = "#FFF5EE";
    pub const Beige: &'static str = "#F5F5DC";
    pub const OldLace: &'static str = "#FDF5E6";
    pub const FloralWhite: &'static str = "#FFFAF0";
    pub const Ivory: &'static str = "#FFFFF0";
    pub const AntiqueWhite: &'static str = "#FAEBD7";
    pub const Linen: &'static str = "#FAF0E6";
    pub const LavenderBlush: &'static str = "#FFF0F5";
    pub const MistyRose: &'static str = "#FFE4E1";
    // Gray Colors;
    pub const Gainsboro: &'static str = "#DCDCDC";
    pub const LightGray: &'static str = "#D3D3D3";
    pub const Silver: &'static str = "#C0C0C0";
    pub const DarkGray: &'static str = "#A9A9A9";
    pub const Gray: &'static str = "#808080";
    pub const DimGray: &'static str = "#696969";
    pub const LightSlateGray: &'static str = "#778899";
    pub const SlateGray: &'static str = "#708090";
    pub const DarkSlateGray: &'static str = "#2F4F4F";
    pub const Black: &'static str = "#000000";
}
