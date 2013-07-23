/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use std::float::round;
use std::libc::types::os::arch::c95::c_double;
use std::cmp::Eq;

#[deriving(Eq)]
pub struct Color {
    red: u8,
    green: u8,
    blue: u8,
    alpha: float,
}

pub fn rgba(r : u8, g : u8, b : u8, a : float) -> Color {
    Color { red : r, green : g, blue : b, alpha : a}
}

pub fn rgb(r : u8, g : u8, b : u8) -> Color {
    return rgba(r, g, b, 1.0);
}

pub fn hsla(h : float, s : float, l : float, a : float) -> Color {
    // Algorithm for converting hsl to rbg taken from
    // http://www.w3.org/TR/2003/CR-css3-color-20030514/#hsl-color
    let m2 = if l <= 0.5 { l*(s + 1.0) } else { l + s - l*s };
    let m1 = l*2.0 - m2;
    let h = h / 360.0; 
    
    fn hue_to_rgb(m1 : float, m2 : float, h : float) -> float {
        let h = if h < 0.0 { h + 1.0 } else if h > 1.0 { h - 1.0 } else { h };

        // FIXME (Rust #7222) - Auugh. Patterns would be much better here
        if 0.0 <= h && h < 1.0/6.0 {
            m1 + (m2 - m1)*h*6.0
        } else if 1.0/6.0 <= h && h < 1.0/2.0 {
            m2
        } else if 1.0/2.0 <= h && h < 2.0/3.0 {
            m1 + (m2 - m1)*(4.0 - 6.0*h)
        } else if 2.0/3.0 <= h && h <= 1.0 {
            m1
        } else {
          fail!(~"unexpected hue value")
        }
    }

    let r = round(255.0*hue_to_rgb(m1, m2, h + 1.0/3.0) as c_double);;
    let g = round(255.0*hue_to_rgb(m1, m2, h) as c_double);
    let b = round(255.0*hue_to_rgb(m1, m2, h - 1.0/3.0) as c_double);

    return rgba(r as u8, g as u8, b as u8, a);
}

pub fn hsl(h : float, s : float, l : float) -> Color {
    return hsla(h, s, l, 1.0);
}

impl Color {
    fn print(&self) -> ~str {
        fmt!("rgba(%u,%u,%u,%f)", self.red as uint, self.green as uint,
             self.blue as uint, self.alpha)
    }
}

pub mod parsing {
    use super::{Color, rgb, rgba, hsl, hsla};
    use super::css_colors::{black, silver, gray, white, maroon, red, purple, fuchsia, green, lime, olive, yellow, navy, blue, teal, aqua};

    fn fail_unrecognized(col : &str) -> Option<Color> {
        warn!("Unrecognized color %s", col);
        return None;
    }

    /** Match an exact color keyword. */
    fn parse_by_name(color : &str) -> Option<Color> {
        let col = match color.to_ascii().to_lower().to_str_ascii() {
                ~"aliceblue" => aliceblue(),
                ~"antiquewhite" => antiquewhite(),
                ~"aqua" => aqua(),
                ~"aquamarine" => aquamarine(),
                ~"azure" => azure(),
                ~"beige" => beige(),
                ~"bisque" => bisque(),
                ~"black" => black(),
                ~"blanchedalmond" => blanchedalmond(),
                ~"blue" => blue(),
                ~"blueviolet" => blueviolet(),
                ~"brown" => brown(),
                ~"burlywood" => burlywood(),
                ~"cadetblue" => cadetblue(),
                ~"chartreuse" => chartreuse(),
                ~"chocolate" => chocolate(),
                ~"coral" => coral(),
                ~"cornflowerblue" => cornflowerblue(),
                ~"cornsilk" => cornsilk(),
                ~"crimson" => crimson(),
                ~"cyan" => cyan(),
                ~"darkblue" => darkblue(),
                ~"darkcyan" => darkcyan(),
                ~"darkgoldenrod" => darkgoldenrod(),
                ~"darkgray" => darkgray(),
                ~"darkgreen" => darkgreen(),
                ~"darkgrey" => darkgrey(),
                ~"darkkhaki" => darkkhaki(),
                ~"darkmagenta" => darkmagenta(),
                ~"darkolivegreen" => darkolivegreen(),
                ~"darkorange" => darkorange(),
                ~"darkorchid" => darkorchid(),
                ~"darkred" => darkred(),
                ~"darksalmon" => darksalmon(),
                ~"darkseagreen" => darkseagreen(),
                ~"darkslateblue" => darkslateblue(),
                ~"darkslategray" => darkslategray(),
                ~"darkslategrey" => darkslategrey(),
                ~"darkturquoise" => darkturquoise(),
                ~"darkviolet" => darkviolet(),
                ~"deeppink" => deeppink(),
                ~"deepskyblue" => deepskyblue(),
                ~"dimgray" => dimgray(),
                ~"dimgrey" => dimgrey(),
                ~"dodgerblue" => dodgerblue(),
                ~"firebrick" => firebrick(),
                ~"floralwhite" => floralwhite(),
                ~"forestgreen" => forestgreen(),
                ~"fuchsia" => fuchsia(),
                ~"gainsboro" => gainsboro(),
                ~"ghostwhite" => ghostwhite(),
                ~"gold" => gold(),
                ~"goldenrod" => goldenrod(),
                ~"gray" => gray(),
                ~"grey" => grey(),
                ~"green" => green(),
                ~"greenyellow" => greenyellow(),
                ~"honeydew" => honeydew(),
                ~"hotpink" => hotpink(),
                ~"indianred" => indianred(),
                ~"indigo" => indigo(),
                ~"ivory" => ivory(),
                ~"khaki" => khaki(),
                ~"lavender" => lavender(),
                ~"lavenderblush" => lavenderblush(),
                ~"lawngreen" => lawngreen(),
                ~"lemonchiffon" => lemonchiffon(),
                ~"lightblue" => lightblue(),
                ~"lightcoral" => lightcoral(),
                ~"lightcyan" => lightcyan(),
                ~"lightgoldenrodyellow" => lightgoldenrodyellow(),
                ~"lightgray" => lightgray(),
                ~"lightgreen" => lightgreen(),
                ~"lightgrey" => lightgrey(),
                ~"lightpink" => lightpink(),
                ~"lightsalmon" => lightsalmon(),
                ~"lightseagreen" => lightseagreen(),
                ~"lightskyblue" => lightskyblue(),
                ~"lightslategray" => lightslategray(),
                ~"lightslategrey" => lightslategrey(),
                ~"lightsteelblue" => lightsteelblue(),
                ~"lightyellow" => lightyellow(),
                ~"lime" => lime(),
                ~"limegreen" => limegreen(),
                ~"linen" => linen(),
                ~"magenta" => magenta(),
                ~"maroon" => maroon(),
                ~"mediumaquamarine" => mediumaquamarine(),
                ~"mediumblue" => mediumblue(),
                ~"mediumorchid" => mediumorchid(),
                ~"mediumpurple" => mediumpurple(),
                ~"mediumseagreen" => mediumseagreen(),
                ~"mediumslateblue" => mediumslateblue(),
                ~"mediumspringgreen" => mediumspringgreen(),
                ~"mediumturquoise" => mediumturquoise(),
                ~"mediumvioletred" => mediumvioletred(),
                ~"midnightblue" => midnightblue(),
                ~"mintcream" => mintcream(),
                ~"mistyrose" => mistyrose(),
                ~"moccasin" => moccasin(),
                ~"navajowhite" => navajowhite(),
                ~"navy" => navy(),
                ~"oldlace" => oldlace(),
                ~"olive" => olive(),
                ~"olivedrab" => olivedrab(),
                ~"orange" => orange(),
                ~"orangered" => orangered(),
                ~"orchid" => orchid(),
                ~"palegoldenrod" => palegoldenrod(),
                ~"palegreen" => palegreen(),
                ~"paleturquoise" => paleturquoise(),
                ~"palevioletred" => palevioletred(),
                ~"papayawhip" => papayawhip(),
                ~"peachpuff" => peachpuff(),
                ~"peru" => peru(),
                ~"pink" => pink(),
                ~"plum" => plum(),
                ~"powderblue" => powderblue(),
                ~"purple" => purple(),
                ~"red" => red(),
                ~"rosybrown" => rosybrown(),
                ~"royalblue" => royalblue(),
                ~"saddlebrown" => saddlebrown(),
                ~"salmon" => salmon(),
                ~"sandybrown" => sandybrown(),
                ~"seagreen" => seagreen(),
                ~"seashell" => seashell(),
                ~"sienna" => sienna(),
                ~"silver" => silver(),
                ~"skyblue" => skyblue(),
                ~"slateblue" => slateblue(),
                ~"slategray" => slategray(),
                ~"slategrey" => slategrey(),
                ~"snow" => snow(),
                ~"springgreen" => springgreen(),
                ~"steelblue" => steelblue(),
                ~"tan" => tan(),
                ~"teal" => teal(),
                ~"thistle" => thistle(),
                ~"tomato" => tomato(),
                ~"turquoise" => turquoise(),
                ~"violet" => violet(),
                ~"wheat" => wheat(),
                ~"white" => white(),
                ~"whitesmoke" => whitesmoke(),
                ~"yellow" => yellow(),
                ~"yellowgreen" => yellowgreen(),
                _ => return fail_unrecognized(color)
        };

        return Some(col);
    }
    
    /** Parses a color specification in the form rgb(foo,bar,baz) */
    fn parse_rgb(color : &str) -> Option<Color> {
        // Shave off the rgb( and the )
        let only_colors = color.slice(4u, color.len() - 1);

        // split up r, g, and b
        let mut cols = ~[];
        for only_colors.split_iter(',').advance |s| {
            cols.push(s);
        };

        if cols.len() != 3u { return fail_unrecognized(color); }

        match (FromStr::from_str(cols[0]), FromStr::from_str(cols[1]), 
               FromStr::from_str(cols[2])) {
          (Some(r), Some(g), Some(b)) => { Some(rgb(r, g, b)) }
          _ => { fail_unrecognized(color) }
        }
    }

    /** Parses a color specification in the form rgba(foo,bar,baz,qux) */
    fn parse_rgba(color : &str) -> Option<Color> {
        // Shave off the rgba( and the )
        let only_vals = color.slice(5u, color.len() - 1);

        // split up r, g, and b
        let mut cols = ~[];
        for only_vals.split_iter(',').advance |s| {
            cols.push(s);
        };

        if cols.len() != 4u { return fail_unrecognized(color); }

        match (FromStr::from_str(cols[0]), FromStr::from_str(cols[1]), 
               FromStr::from_str(cols[2]), FromStr::from_str(cols[3])) {
          (Some(r), Some(g), Some(b), Some(a)) => { Some(rgba(r, g, b, a)) }
          _ => { fail_unrecognized(color) }
        }
    }

    /** Parses a color specification in the form hsl(foo,bar,baz) */
    fn parse_hsl(color : &str) -> Option<Color> {
        // Shave off the hsl( and the )
        let only_vals = color.slice(4u, color.len() - 1);

        // split up h, s, and l
        let mut vals = ~[];
        for only_vals.split_iter(',').advance |s| {
            vals.push(s);
        };

        if vals.len() != 3u { return fail_unrecognized(color); }

        match (FromStr::from_str(vals[0]), FromStr::from_str(vals[1]), 
               FromStr::from_str(vals[2])) {
          (Some(h), Some(s), Some(l)) => { Some(hsl(h, s, l)) }
          _ => { fail_unrecognized(color) }
        }
    }

    /** Parses a color specification in the form hsla(foo,bar,baz,qux) */
    fn parse_hsla(color : &str) -> Option<Color> {
        // Shave off the hsla( and the )
        let only_vals = color.slice(5u, color.len() - 1);

        let mut vals = ~[];
        for only_vals.split_iter(',').advance |s| {
            vals.push(s);
        };

        if vals.len() != 4u { return fail_unrecognized(color); }

        match (FromStr::from_str(vals[0]), FromStr::from_str(vals[1]), 
               FromStr::from_str(vals[2]), FromStr::from_str(vals[3])) {
          (Some(h), Some(s), Some(l), Some(a)) => { Some(hsla(h, s, l, a)) }
          _ => { fail_unrecognized(color) }
        }
    }

    // Currently colors are supported in rgb(a,b,c) form and also by
    // keywords for several common colors.
    // TODO: extend this
    pub fn parse_color(color : &str) -> Option<Color> {
        match color {
          c if c.starts_with("rgb(") => parse_rgb(c),
          c if c.starts_with("rgba(") => parse_rgba(c),
          c if c.starts_with("hsl(") => parse_hsl(c),
          c if c.starts_with("hsla(") => parse_hsla(c),
          c => parse_by_name(c)
        }
    }
}

#[cfg(test)]
mod test {
    use super::{rgb, rgba};
    use super::css_colors::*;
    use super::parsing::parse_color;

    #[test]
    fn test_parse_by_name() {
        assert!(red().eq(&parse_color("red").unwrap()));
        assert!(lime().eq(&parse_color("Lime").unwrap()));
        assert!(blue().eq(&parse_color("BLUE").unwrap()));
        assert!(green().eq(&parse_color("GreEN").unwrap()));
        assert!(white().eq(&parse_color("white").unwrap()));
        assert!(black().eq(&parse_color("Black").unwrap()));
        assert!(gray().eq(&parse_color("Gray").unwrap()));
        println("silver");
        assert!(silver().eq(&parse_color("SiLvEr").unwrap()));
        assert!(maroon().eq(&parse_color("maroon").unwrap()));
        assert!(purple().eq(&parse_color("PURPLE").unwrap()));
        assert!(fuchsia().eq(&parse_color("FUCHSIA").unwrap()));
        assert!(olive().eq(&parse_color("oLiVe").unwrap()));
        assert!(yellow().eq(&parse_color("yellow").unwrap()));
        assert!(navy().eq(&parse_color("NAVY").unwrap()));
        assert!(teal().eq(&parse_color("Teal").unwrap()));
        assert!(aqua().eq(&parse_color("Aqua").unwrap()));
        assert!(None == parse_color("foobarbaz"));
    }

    #[test]
    fn test_parsing_rgb() {
        assert!(red().eq(&parse_color("rgb(255,0,0)").unwrap()));
        assert!(red().eq(&parse_color("rgba(255,0,0,1.0)").unwrap()));
        assert!(red().eq(&parse_color("rgba(255,0,0,1)").unwrap()));
        assert!(lime().eq(&parse_color("rgba(0,255,0,1.00)").unwrap()));
        assert!(rgb(1u8,2u8,3u8).eq(&parse_color("rgb(1,2,03)").unwrap()));
        assert!(rgba(15u8,250u8,3u8,0.5).eq(&parse_color("rgba(15,250,3,.5)").unwrap()));
        assert!(rgba(15u8,250u8,3u8,0.5).eq(&parse_color("rgba(15,250,3,0.5)").unwrap()));
        assert!(None == parse_color("rbga(1,2,3)"));
    }

    #[test]
    fn test_parsing_hsl() {
        assert!(red().eq(&parse_color("hsl(0,1,.5)").unwrap()));
        assert!(lime().eq(&parse_color("hsl(120.0,1.0,.5)").unwrap()));
        assert!(blue().eq(&parse_color("hsl(240.0,1.0,.5)").unwrap()));
        assert!(green().eq(&parse_color("hsl(120.0,1.0,.25)").unwrap()));
        assert!(white().eq(&parse_color("hsl(1.0,1.,1.0)").unwrap()));
        assert!(white().eq(&parse_color("hsl(129.0,0.3,1.0)").unwrap()));
        assert!(black().eq(&parse_color("hsl(231.2,0.75,0.0)").unwrap()));
        assert!(black().eq(&parse_color("hsl(11.2,0.0,0.0)").unwrap()));
        assert!(gray().eq(&parse_color("hsl(0.0,0.0,0.5)").unwrap()));
        assert!(maroon().eq(&parse_color("hsl(0.0,1.0,0.25)").unwrap()));
        assert!(purple().eq(&parse_color("hsl(300.0,1.0,0.25)").unwrap()));
        assert!(fuchsia().eq(&parse_color("hsl(300,1.0,0.5)").unwrap()));
        assert!(olive().eq(&parse_color("hsl(60.,1.0,0.25)").unwrap()));
        assert!(yellow().eq(&parse_color("hsl(60.,1.0,0.5)").unwrap()));
        assert!(navy().eq(&parse_color("hsl(240.0,1.0,.25)").unwrap()));
        assert!(teal().eq(&parse_color("hsl(180.0,1.0,.25)").unwrap()));
        assert!(aqua().eq(&parse_color("hsl(180.0,1.0,.5)").unwrap()));
        assert!(None == parse_color("hsl(1,2,3,.4)"));
    }

    #[test]
    fn test_parsing_rgb_all() {
        assert!(aliceblue().eq(&parse_color("rgb(240, 248, 255)").unwrap()));
        assert!(antiquewhite().eq(&parse_color("rgb(250, 235, 215)").unwrap()));
        assert!(aqua().eq(&parse_color("rgb(0, 255, 255)").unwrap()));
        assert!(aquamarine().eq(&parse_color("rgb(127, 255, 212)").unwrap()));
        assert!(azure().eq(&parse_color("rgb(240, 255, 255)").unwrap()));
        assert!(beige().eq(&parse_color("rgb(245, 245, 220)").unwrap()));
        assert!(bisque().eq(&parse_color("rgb(255, 228, 196)").unwrap()));
        assert!(black().eq(&parse_color("rgb(0, 0, 0)").unwrap()));
        assert!(blanchedalmond().eq(&parse_color("rgb(255, 235, 205)").unwrap()));
        assert!(blue().eq(&parse_color("rgb(0, 0, 255)").unwrap()));
        assert!(blueviolet().eq(&parse_color("rgb(138, 43, 226)").unwrap()));
        assert!(brown().eq(&parse_color("rgb(165, 42, 42)").unwrap()));
        assert!(burlywood().eq(&parse_color("rgb(222, 184, 135)").unwrap()));
        assert!(cadetblue().eq(&parse_color("rgb(95, 158, 160)").unwrap()));
        assert!(chartreuse().eq(&parse_color("rgb(127, 255, 0)").unwrap()));
        assert!(chocolate().eq(&parse_color("rgb(210, 105, 30)").unwrap()));
        assert!(coral().eq(&parse_color("rgb(255, 127, 80)").unwrap()));
        assert!(cornflowerblue().eq(&parse_color("rgb(100, 149, 237)").unwrap()));
        assert!(cornsilk().eq(&parse_color("rgb(255, 248, 220)").unwrap()));
        assert!(crimson().eq(&parse_color("rgb(220, 20, 60)").unwrap()));
        assert!(cyan().eq(&parse_color("rgb(0, 255, 255)").unwrap()));
        assert!(darkblue().eq(&parse_color("rgb(0, 0, 139)").unwrap()));
        assert!(darkcyan().eq(&parse_color("rgb(0, 139, 139)").unwrap()));
        assert!(darkgoldenrod().eq(&parse_color("rgb(184, 134, 11)").unwrap()));
        assert!(darkgray().eq(&parse_color("rgb(169, 169, 169)").unwrap()));
        assert!(darkgreen().eq(&parse_color("rgb(0, 100, 0)").unwrap()));
        assert!(darkgrey().eq(&parse_color("rgb(169, 169, 169)").unwrap()));
        assert!(darkkhaki().eq(&parse_color("rgb(189, 183, 107)").unwrap()));
        assert!(darkmagenta().eq(&parse_color("rgb(139, 0, 139)").unwrap()));
        assert!(darkolivegreen().eq(&parse_color("rgb(85, 107, 47)").unwrap()));
        assert!(darkorange().eq(&parse_color("rgb(255, 140, 0)").unwrap()));
        assert!(darkorchid().eq(&parse_color("rgb(153, 50, 204)").unwrap()));
        assert!(darkred().eq(&parse_color("rgb(139, 0, 0)").unwrap()));
        assert!(darksalmon().eq(&parse_color("rgb(233, 150, 122)").unwrap()));
        assert!(darkseagreen().eq(&parse_color("rgb(143, 188, 143)").unwrap()));
        assert!(darkslateblue().eq(&parse_color("rgb(72, 61, 139)").unwrap()));
        assert!(darkslategray().eq(&parse_color("rgb(47, 79, 79)").unwrap()));
        assert!(darkslategrey().eq(&parse_color("rgb(47, 79, 79)").unwrap()));
        assert!(darkturquoise().eq(&parse_color("rgb(0, 206, 209)").unwrap()));
        assert!(darkviolet().eq(&parse_color("rgb(148, 0, 211)").unwrap()));
        assert!(deeppink().eq(&parse_color("rgb(255, 20, 147)").unwrap()));
        assert!(deepskyblue().eq(&parse_color("rgb(0, 191, 255)").unwrap()));
        assert!(dimgray().eq(&parse_color("rgb(105, 105, 105)").unwrap()));
        assert!(dimgrey().eq(&parse_color("rgb(105, 105, 105)").unwrap()));
        assert!(dodgerblue().eq(&parse_color("rgb(30, 144, 255)").unwrap()));
        assert!(firebrick().eq(&parse_color("rgb(178, 34, 34)").unwrap()));
        assert!(floralwhite().eq(&parse_color("rgb(255, 250, 240)").unwrap()));
        assert!(forestgreen().eq(&parse_color("rgb(34, 139, 34)").unwrap()));
        assert!(fuchsia().eq(&parse_color("rgb(255, 0, 255)").unwrap()));
        assert!(gainsboro().eq(&parse_color("rgb(220, 220, 220)").unwrap()));
        assert!(ghostwhite().eq(&parse_color("rgb(248, 248, 255)").unwrap()));
        assert!(gold().eq(&parse_color("rgb(255, 215, 0)").unwrap()));
        assert!(goldenrod().eq(&parse_color("rgb(218, 165, 32)").unwrap()));
        assert!(gray().eq(&parse_color("rgb(128, 128, 128)").unwrap()));
        assert!(grey().eq(&parse_color("rgb(128, 128, 128)").unwrap()));
        assert!(green().eq(&parse_color("rgb(0, 128, 0)").unwrap()));
        assert!(greenyellow().eq(&parse_color("rgb(173, 255, 47)").unwrap()));
        assert!(honeydew().eq(&parse_color("rgb(240, 255, 240)").unwrap()));
        assert!(hotpink().eq(&parse_color("rgb(255, 105, 180)").unwrap()));
        assert!(indianred().eq(&parse_color("rgb(205, 92, 92)").unwrap()));
        assert!(indigo().eq(&parse_color("rgb(75, 0, 130)").unwrap()));
        assert!(ivory().eq(&parse_color("rgb(255, 255, 240)").unwrap()));
        assert!(khaki().eq(&parse_color("rgb(240, 230, 140)").unwrap()));
        assert!(lavender().eq(&parse_color("rgb(230, 230, 250)").unwrap()));
        assert!(lavenderblush().eq(&parse_color("rgb(255, 240, 245)").unwrap()));
        assert!(lawngreen().eq(&parse_color("rgb(124, 252, 0)").unwrap()));
        assert!(lemonchiffon().eq(&parse_color("rgb(255, 250, 205)").unwrap()));
        assert!(lightblue().eq(&parse_color("rgb(173, 216, 230)").unwrap()));
        assert!(lightcoral().eq(&parse_color("rgb(240, 128, 128)").unwrap()));
        assert!(lightcyan().eq(&parse_color("rgb(224, 255, 255)").unwrap()));
        assert!(lightgoldenrodyellow().eq(&parse_color("rgb(250, 250, 210)").unwrap()));
        assert!(lightgray().eq(&parse_color("rgb(211, 211, 211)").unwrap()));
        assert!(lightgreen().eq(&parse_color("rgb(144, 238, 144)").unwrap()));
        assert!(lightgrey().eq(&parse_color("rgb(211, 211, 211)").unwrap()));
        assert!(lightpink().eq(&parse_color("rgb(255, 182, 193)").unwrap()));
        assert!(lightsalmon().eq(&parse_color("rgb(255, 160, 122)").unwrap()));
        assert!(lightseagreen().eq(&parse_color("rgb(32, 178, 170)").unwrap()));
        assert!(lightskyblue().eq(&parse_color("rgb(135, 206, 250)").unwrap()));
        assert!(lightslategray().eq(&parse_color("rgb(119, 136, 153)").unwrap()));
        assert!(lightslategrey().eq(&parse_color("rgb(119, 136, 153)").unwrap()));
        assert!(lightsteelblue().eq(&parse_color("rgb(176, 196, 222)").unwrap()));
        assert!(lightyellow().eq(&parse_color("rgb(255, 255, 224)").unwrap()));
        assert!(lime().eq(&parse_color("rgb(0, 255, 0)").unwrap()));
        assert!(limegreen().eq(&parse_color("rgb(50, 205, 50)").unwrap()));
        assert!(linen().eq(&parse_color("rgb(250, 240, 230)").unwrap()));
        assert!(magenta().eq(&parse_color("rgb(255, 0, 255)").unwrap()));
        assert!(maroon().eq(&parse_color("rgb(128, 0, 0)").unwrap()));
        assert!(mediumaquamarine().eq(&parse_color("rgb(102, 205, 170)").unwrap()));
        assert!(mediumblue().eq(&parse_color("rgb(0, 0, 205)").unwrap()));
        assert!(mediumorchid().eq(&parse_color("rgb(186, 85, 211)").unwrap()));
        assert!(mediumpurple().eq(&parse_color("rgb(147, 112, 219)").unwrap()));
        assert!(mediumseagreen().eq(&parse_color("rgb(60, 179, 113)").unwrap()));
        assert!(mediumslateblue().eq(&parse_color("rgb(123, 104, 238)").unwrap()));
        assert!(mediumspringgreen().eq(&parse_color("rgb(0, 250, 154)").unwrap()));
        assert!(mediumturquoise().eq(&parse_color("rgb(72, 209, 204)").unwrap()));
        assert!(mediumvioletred().eq(&parse_color("rgb(199, 21, 133)").unwrap()));
        assert!(midnightblue().eq(&parse_color("rgb(25, 25, 112)").unwrap()));
        assert!(mintcream().eq(&parse_color("rgb(245, 255, 250)").unwrap()));
        assert!(mistyrose().eq(&parse_color("rgb(255, 228, 225)").unwrap()));
        assert!(moccasin().eq(&parse_color("rgb(255, 228, 181)").unwrap()));
        assert!(navajowhite().eq(&parse_color("rgb(255, 222, 173)").unwrap()));
        assert!(navy().eq(&parse_color("rgb(0, 0, 128)").unwrap()));
        assert!(oldlace().eq(&parse_color("rgb(253, 245, 230)").unwrap()));
        assert!(olive().eq(&parse_color("rgb(128, 128, 0)").unwrap()));
        assert!(olivedrab().eq(&parse_color("rgb(107, 142, 35)").unwrap()));
        assert!(orange().eq(&parse_color("rgb(255, 165, 0)").unwrap()));
        assert!(orangered().eq(&parse_color("rgb(255, 69, 0)").unwrap()));
        assert!(orchid().eq(&parse_color("rgb(218, 112, 214)").unwrap()));
        assert!(palegoldenrod().eq(&parse_color("rgb(238, 232, 170)").unwrap()));
        assert!(palegreen().eq(&parse_color("rgb(152, 251, 152)").unwrap()));
        assert!(paleturquoise().eq(&parse_color("rgb(175, 238, 238)").unwrap()));
        assert!(palevioletred().eq(&parse_color("rgb(219, 112, 147)").unwrap()));
        assert!(papayawhip().eq(&parse_color("rgb(255, 239, 213)").unwrap()));
        assert!(peachpuff().eq(&parse_color("rgb(255, 218, 185)").unwrap()));
        assert!(peru().eq(&parse_color("rgb(205, 133, 63)").unwrap()));
        assert!(pink().eq(&parse_color("rgb(255, 192, 203)").unwrap()));
        assert!(plum().eq(&parse_color("rgb(221, 160, 221)").unwrap()));
        assert!(powderblue().eq(&parse_color("rgb(176, 224, 230)").unwrap()));
        assert!(purple().eq(&parse_color("rgb(128, 0, 128)").unwrap()));
        assert!(red().eq(&parse_color("rgb(255, 0, 0)").unwrap()));
        assert!(rosybrown().eq(&parse_color("rgb(188, 143, 143)").unwrap()));
        assert!(royalblue().eq(&parse_color("rgb(65, 105, 225)").unwrap()));
        assert!(saddlebrown().eq(&parse_color("rgb(139, 69, 19)").unwrap()));
        assert!(salmon().eq(&parse_color("rgb(250, 128, 114)").unwrap()));
        assert!(sandybrown().eq(&parse_color("rgb(244, 164, 96)").unwrap()));
        assert!(seagreen().eq(&parse_color("rgb(46, 139, 87)").unwrap()));
        assert!(seashell().eq(&parse_color("rgb(255, 245, 238)").unwrap()));
        assert!(sienna().eq(&parse_color("rgb(160, 82, 45)").unwrap()));
        assert!(silver().eq(&parse_color("rgb(192, 192, 192)").unwrap()));
        assert!(skyblue().eq(&parse_color("rgb(135, 206, 235)").unwrap()));
        assert!(slateblue().eq(&parse_color("rgb(106, 90, 205)").unwrap()));
        assert!(slategray().eq(&parse_color("rgb(112, 128, 144)").unwrap()));
        assert!(slategrey().eq(&parse_color("rgb(112, 128, 144)").unwrap()));
        assert!(snow().eq(&parse_color("rgb(255, 250, 250)").unwrap()));
        assert!(springgreen().eq(&parse_color("rgb(0, 255, 127)").unwrap()));
        assert!(steelblue().eq(&parse_color("rgb(70, 130, 180)").unwrap()));
        assert!(tan().eq(&parse_color("rgb(210, 180, 140)").unwrap()));
        assert!(teal().eq(&parse_color("rgb(0, 128, 128)").unwrap()));
        assert!(thistle().eq(&parse_color("rgb(216, 191, 216)").unwrap()));
        assert!(tomato().eq(&parse_color("rgb(255, 99, 71)").unwrap()));
        assert!(turquoise().eq(&parse_color("rgb(64, 224, 208)").unwrap()));
        assert!(violet().eq(&parse_color("rgb(238, 130, 238)").unwrap()));
        assert!(wheat().eq(&parse_color("rgb(245, 222, 179)").unwrap()));
        assert!(white().eq(&parse_color("rgb(255, 255, 255)").unwrap()));
        assert!(whitesmoke().eq(&parse_color("rgb(245, 245, 245)").unwrap()));
        assert!(yellow().eq(&parse_color("rgb(255, 255, 0)").unwrap()));
        assert!(yellowgreen().eq(&parse_color("rgb(154, 205, 50)").unwrap()));
    }
}


/** Define the colors specified by css */
pub mod css_colors {
    use super::Color;

    // The 147 css colors
    pub fn aliceblue() -> Color {
        Color {red: 240u8, green: 248u8, blue: 255u8, alpha: 1.0}
    }
    pub fn antiquewhite() -> Color {
        Color {red: 250u8, green: 235u8, blue: 215u8, alpha: 1.0}
    }
    pub fn aqua() -> Color {
        Color {red: 0u8, green: 255u8, blue: 255u8, alpha: 1.0}
    }
    pub fn aquamarine() -> Color {
        Color {red: 127u8, green: 255u8, blue: 212u8, alpha: 1.0}
    }
    pub fn azure() -> Color {
        Color {red: 240u8, green: 255u8, blue: 255u8, alpha: 1.0}
    }
    pub fn beige() -> Color {
        Color {red: 245u8, green: 245u8, blue: 220u8, alpha: 1.0}
    }
    pub fn bisque() -> Color {
        Color {red: 255u8, green: 228u8, blue: 196u8, alpha: 1.0}
    }
    pub fn black() -> Color {
        Color {red: 0u8, green: 0u8, blue: 0u8, alpha: 1.0}
    }
    pub fn blanchedalmond() -> Color {
        Color {red: 255u8, green: 235u8, blue: 205u8, alpha: 1.0}
    }
    pub fn blue() -> Color {
        Color {red: 0u8, green: 0u8, blue: 255u8, alpha: 1.0}
    }
    pub fn blueviolet() -> Color {
        Color {red: 138u8, green: 43u8, blue: 226u8, alpha: 1.0}
    }
    pub fn brown() -> Color {
        Color {red: 165u8, green: 42u8, blue: 42u8, alpha: 1.0}
    }
    pub fn burlywood() -> Color {
        Color {red: 222u8, green: 184u8, blue: 135u8, alpha: 1.0}
    }
    pub fn cadetblue() -> Color {
        Color {red: 95u8, green: 158u8, blue: 160u8, alpha: 1.0}
    }
    pub fn chartreuse() -> Color {
        Color {red: 127u8, green: 255u8, blue: 0u8, alpha: 1.0}
    }
    pub fn chocolate() -> Color {
        Color {red: 210u8, green: 105u8, blue: 30u8, alpha: 1.0}
    }
    pub fn coral() -> Color {
        Color {red: 255u8, green: 127u8, blue: 80u8, alpha: 1.0}
    }
    pub fn cornflowerblue() -> Color {
        Color {red: 100u8, green: 149u8, blue: 237u8, alpha: 1.0}
    }
    pub fn cornsilk() -> Color {
        Color {red: 255u8, green: 248u8, blue: 220u8, alpha: 1.0}
    }
    pub fn crimson() -> Color {
        Color {red: 220u8, green: 20u8, blue: 60u8, alpha: 1.0}
    }
    pub fn cyan() -> Color {
        Color {red: 0u8, green: 255u8, blue: 255u8, alpha: 1.0}
    }
    pub fn darkblue() -> Color {
        Color {red: 0u8, green: 0u8, blue: 139u8, alpha: 1.0}
    }
    pub fn darkcyan() -> Color {
        Color {red: 0u8, green: 139u8, blue: 139u8, alpha: 1.0}
    }
    pub fn darkgoldenrod() -> Color {
        Color {red: 184u8, green: 134u8, blue: 11u8, alpha: 1.0}
    }
    pub fn darkgray() -> Color {
        Color {red: 169u8, green: 169u8, blue: 169u8, alpha: 1.0}
    }
    pub fn darkgreen() -> Color {
        Color {red: 0u8, green: 100u8, blue: 0u8, alpha: 1.0}
    }
    pub fn darkgrey() -> Color {
        Color {red: 169u8, green: 169u8, blue: 169u8, alpha: 1.0}
    }
    pub fn darkkhaki() -> Color {
        Color {red: 189u8, green: 183u8, blue: 107u8, alpha: 1.0}
    }
    pub fn darkmagenta() -> Color {
        Color {red: 139u8, green: 0u8, blue: 139u8, alpha: 1.0}
    }
    pub fn darkolivegreen() -> Color {
        Color {red: 85u8, green: 107u8, blue: 47u8, alpha: 1.0}
    }
    pub fn darkorange() -> Color {
        Color {red: 255u8, green: 140u8, blue: 0u8, alpha: 1.0}
    }
    pub fn darkorchid() -> Color {
        Color {red: 153u8, green: 50u8, blue: 204u8, alpha: 1.0}
    }
    pub fn darkred() -> Color {
        Color {red: 139u8, green: 0u8, blue: 0u8, alpha: 1.0}
    }
    pub fn darksalmon() -> Color {
        Color {red: 233u8, green: 150u8, blue: 122u8, alpha: 1.0}
    }
    pub fn darkseagreen() -> Color {
        Color {red: 143u8, green: 188u8, blue: 143u8, alpha: 1.0}
    }
    pub fn darkslateblue() -> Color {
        Color {red: 72u8, green: 61u8, blue: 139u8, alpha: 1.0}
    }
    pub fn darkslategray() -> Color {
        Color {red: 47u8, green: 79u8, blue: 79u8, alpha: 1.0}
    }
    pub fn darkslategrey() -> Color {
        Color {red: 47u8, green: 79u8, blue: 79u8, alpha: 1.0}
    }
    pub fn darkturquoise() -> Color {
        Color {red: 0u8, green: 206u8, blue: 209u8, alpha: 1.0}
    }
    pub fn darkviolet() -> Color {
        Color {red: 148u8, green: 0u8, blue: 211u8, alpha: 1.0}
    }
    pub fn deeppink() -> Color {
        Color {red: 255u8, green: 20u8, blue: 147u8, alpha: 1.0}
    }
    pub fn deepskyblue() -> Color {
        Color {red: 0u8, green: 191u8, blue: 255u8, alpha: 1.0}
    }
    pub fn dimgray() -> Color {
        Color {red: 105u8, green: 105u8, blue: 105u8, alpha: 1.0}
    }
    pub fn dimgrey() -> Color {
        Color {red: 105u8, green: 105u8, blue: 105u8, alpha: 1.0}
    }
    pub fn dodgerblue() -> Color {
        Color {red: 30u8, green: 144u8, blue: 255u8, alpha: 1.0}
    }
    pub fn firebrick() -> Color {
        Color {red: 178u8, green: 34u8, blue: 34u8, alpha: 1.0}
    }
    pub fn floralwhite() -> Color {
        Color {red: 255u8, green: 250u8, blue: 240u8, alpha: 1.0}
    }
    pub fn forestgreen() -> Color {
        Color {red: 34u8, green: 139u8, blue: 34u8, alpha: 1.0}
    }
    pub fn fuchsia() -> Color {
        Color {red: 255u8, green: 0u8, blue: 255u8, alpha: 1.0}
    }
    pub fn gainsboro() -> Color {
        Color {red: 220u8, green: 220u8, blue: 220u8, alpha: 1.0}
    }
    pub fn ghostwhite() -> Color {
        Color {red: 248u8, green: 248u8, blue: 255u8, alpha: 1.0}
    }
    pub fn gold() -> Color {
        Color {red: 255u8, green: 215u8, blue: 0u8, alpha: 1.0}
    }
    pub fn goldenrod() -> Color {
        Color {red: 218u8, green: 165u8, blue: 32u8, alpha: 1.0}
    }
    pub fn gray() -> Color {
        Color {red: 128u8, green: 128u8, blue: 128u8, alpha: 1.0}
    }
    pub fn grey() -> Color {
        Color {red: 128u8, green: 128u8, blue: 128u8, alpha: 1.0}
    }
    pub fn green() -> Color {
        Color {red: 0u8, green: 128u8, blue: 0u8, alpha: 1.0}
    }
    pub fn greenyellow() -> Color {
        Color {red: 173u8, green: 255u8, blue: 47u8, alpha: 1.0}
    }
    pub fn honeydew() -> Color {
        Color {red: 240u8, green: 255u8, blue: 240u8, alpha: 1.0}
    }
    pub fn hotpink() -> Color {
        Color {red: 255u8, green: 105u8, blue: 180u8, alpha: 1.0}
    }
    pub fn indianred() -> Color {
        Color {red: 205u8, green: 92u8, blue: 92u8, alpha: 1.0}
    }
    pub fn indigo() -> Color {
        Color {red: 75u8, green: 0u8, blue: 130u8, alpha: 1.0}
    }
    pub fn ivory() -> Color {
        Color {red: 255u8, green: 255u8, blue: 240u8, alpha: 1.0}
    }
    pub fn khaki() -> Color {
        Color {red: 240u8, green: 230u8, blue: 140u8, alpha: 1.0}
    }
    pub fn lavender() -> Color {
        Color {red: 230u8, green: 230u8, blue: 250u8, alpha: 1.0}
    }
    pub fn lavenderblush() -> Color {
        Color {red: 255u8, green: 240u8, blue: 245u8, alpha: 1.0}
    }
    pub fn lawngreen() -> Color {
        Color {red: 124u8, green: 252u8, blue: 0u8, alpha: 1.0}
    }
    pub fn lemonchiffon() -> Color {
        Color {red: 255u8, green: 250u8, blue: 205u8, alpha: 1.0}
    }
    pub fn lightblue() -> Color {
        Color {red: 173u8, green: 216u8, blue: 230u8, alpha: 1.0}
    }
    pub fn lightcoral() -> Color {
        Color {red: 240u8, green: 128u8, blue: 128u8, alpha: 1.0}
    }
    pub fn lightcyan() -> Color {
        Color {red: 224u8, green: 255u8, blue: 255u8, alpha: 1.0}
    }
    pub fn lightgoldenrodyellow() -> Color {
        Color {red: 250u8, green: 250u8, blue: 210u8, alpha: 1.0}
    }
    pub fn lightgray() -> Color {
        Color {red: 211u8, green: 211u8, blue: 211u8, alpha: 1.0}
    }
    pub fn lightgreen() -> Color {
        Color {red: 144u8, green: 238u8, blue: 144u8, alpha: 1.0}
    }
    pub fn lightgrey() -> Color {
        Color {red: 211u8, green: 211u8, blue: 211u8, alpha: 1.0}
    }
    pub fn lightpink() -> Color {
        Color {red: 255u8, green: 182u8, blue: 193u8, alpha: 1.0}
    }
    pub fn lightsalmon() -> Color {
        Color {red: 255u8, green: 160u8, blue: 122u8, alpha: 1.0}
    }
    pub fn lightseagreen() -> Color {
        Color {red: 32u8, green: 178u8, blue: 170u8, alpha: 1.0}
    }
    pub fn lightskyblue() -> Color {
        Color {red: 135u8, green: 206u8, blue: 250u8, alpha: 1.0}
    }
    pub fn lightslategray() -> Color {
        Color {red: 119u8, green: 136u8, blue: 153u8, alpha: 1.0}
    }
    pub fn lightslategrey() -> Color {
        Color {red: 119u8, green: 136u8, blue: 153u8, alpha: 1.0}
    }
    pub fn lightsteelblue() -> Color {
        Color {red: 176u8, green: 196u8, blue: 222u8, alpha: 1.0}
    }
    pub fn lightyellow() -> Color {
        Color {red: 255u8, green: 255u8, blue: 224u8, alpha: 1.0}
    }
    pub fn lime() -> Color {
        Color {red: 0u8, green: 255u8, blue: 0u8, alpha: 1.0}
    }
    pub fn limegreen() -> Color {
        Color {red: 50u8, green: 205u8, blue: 50u8, alpha: 1.0}
    }
    pub fn linen() -> Color {
        Color {red: 250u8, green: 240u8, blue: 230u8, alpha: 1.0}
    }
    pub fn magenta() -> Color {
        Color {red: 255u8, green: 0u8, blue: 255u8, alpha: 1.0}
    }
    pub fn maroon() -> Color {
        Color {red: 128u8, green: 0u8, blue: 0u8, alpha: 1.0}
    }
    pub fn mediumaquamarine() -> Color {
        Color {red: 102u8, green: 205u8, blue: 170u8, alpha: 1.0}
    }
    pub fn mediumblue() -> Color {
        Color {red: 0u8, green: 0u8, blue: 205u8, alpha: 1.0}
    }
    pub fn mediumorchid() -> Color {
        Color {red: 186u8, green: 85u8, blue: 211u8, alpha: 1.0}
    }
    pub fn mediumpurple() -> Color {
        Color {red: 147u8, green: 112u8, blue: 219u8, alpha: 1.0}
    }
    pub fn mediumseagreen() -> Color {
        Color {red: 60u8, green: 179u8, blue: 113u8, alpha: 1.0}
    }
    pub fn mediumslateblue() -> Color {
        Color {red: 123u8, green: 104u8, blue: 238u8, alpha: 1.0}
    }
    pub fn mediumspringgreen() -> Color {
        Color {red: 0u8, green: 250u8, blue: 154u8, alpha: 1.0}
    }
    pub fn mediumturquoise() -> Color {
        Color {red: 72u8, green: 209u8, blue: 204u8, alpha: 1.0}
    }
    pub fn mediumvioletred() -> Color {
        Color {red: 199u8, green: 21u8, blue: 133u8, alpha: 1.0}
    }
    pub fn midnightblue() -> Color {
        Color {red: 25u8, green: 25u8, blue: 112u8, alpha: 1.0}
    }
    pub fn mintcream() -> Color {
        Color {red: 245u8, green: 255u8, blue: 250u8, alpha: 1.0}
    }
    pub fn mistyrose() -> Color {
        Color {red: 255u8, green: 228u8, blue: 225u8, alpha: 1.0}
    }
    pub fn moccasin() -> Color {
        Color {red: 255u8, green: 228u8, blue: 181u8, alpha: 1.0}
    }
    pub fn navajowhite() -> Color {
        Color {red: 255u8, green: 222u8, blue: 173u8, alpha: 1.0}
    }
    pub fn navy() -> Color {
        Color {red: 0u8, green: 0u8, blue: 128u8, alpha: 1.0}
    }
    pub fn oldlace() -> Color {
        Color {red: 253u8, green: 245u8, blue: 230u8, alpha: 1.0}
    }
    pub fn olive() -> Color {
        Color {red: 128u8, green: 128u8, blue: 0u8, alpha: 1.0}
    }
    pub fn olivedrab() -> Color {
        Color {red: 107u8, green: 142u8, blue: 35u8, alpha: 1.0}
    }
    pub fn orange() -> Color {
        Color {red: 255u8, green: 165u8, blue: 0u8, alpha: 1.0}
    }
    pub fn orangered() -> Color {
        Color {red: 255u8, green: 69u8, blue: 0u8, alpha: 1.0}
    }
    pub fn orchid() -> Color {
        Color {red: 218u8, green: 112u8, blue: 214u8, alpha: 1.0}
    }
    pub fn palegoldenrod() -> Color {
        Color {red: 238u8, green: 232u8, blue: 170u8, alpha: 1.0}
    }
    pub fn palegreen() -> Color {
        Color {red: 152u8, green: 251u8, blue: 152u8, alpha: 1.0}
    }
    pub fn paleturquoise() -> Color {
        Color {red: 175u8, green: 238u8, blue: 238u8, alpha: 1.0}
    }
    pub fn palevioletred() -> Color {
        Color {red: 219u8, green: 112u8, blue: 147u8, alpha: 1.0}
    }
    pub fn papayawhip() -> Color {
        Color {red: 255u8, green: 239u8, blue: 213u8, alpha: 1.0}
    }
    pub fn peachpuff() -> Color {
        Color {red: 255u8, green: 218u8, blue: 185u8, alpha: 1.0}
    }
    pub fn peru() -> Color {
        Color {red: 205u8, green: 133u8, blue: 63u8, alpha: 1.0}
    }
    pub fn pink() -> Color {
        Color {red: 255u8, green: 192u8, blue: 203u8, alpha: 1.0}
    }
    pub fn plum() -> Color {
        Color {red: 221u8, green: 160u8, blue: 221u8, alpha: 1.0}
    }
    pub fn powderblue() -> Color {
        Color {red: 176u8, green: 224u8, blue: 230u8, alpha: 1.0}
    }
    pub fn purple() -> Color {
        Color {red: 128u8, green: 0u8, blue: 128u8, alpha: 1.0}
    }
    pub fn red() -> Color {
        Color {red: 255u8, green: 0u8, blue: 0u8, alpha: 1.0}
    }
    pub fn rosybrown() -> Color {
        Color {red: 188u8, green: 143u8, blue: 143u8, alpha: 1.0}
    }
    pub fn royalblue() -> Color {
        Color {red: 65u8, green: 105u8, blue: 225u8, alpha: 1.0}
    }
    pub fn saddlebrown() -> Color {
        Color {red: 139u8, green: 69u8, blue: 19u8, alpha: 1.0}
    }
    pub fn salmon() -> Color {
        Color {red: 250u8, green: 128u8, blue: 114u8, alpha: 1.0}
    }
    pub fn sandybrown() -> Color {
        Color {red: 244u8, green: 164u8, blue: 96u8, alpha: 1.0}
    }
    pub fn seagreen() -> Color {
        Color {red: 46u8, green: 139u8, blue: 87u8, alpha: 1.0}
    }
    pub fn seashell() -> Color {
        Color {red: 255u8, green: 245u8, blue: 238u8, alpha: 1.0}
    }
    pub fn sienna() -> Color {
        Color {red: 160u8, green: 82u8, blue: 45u8, alpha: 1.0}
    }
    pub fn silver() -> Color {
        Color {red: 192u8, green: 192u8, blue: 192u8, alpha: 1.0}
    }
    pub fn skyblue() -> Color {
        Color {red: 135u8, green: 206u8, blue: 235u8, alpha: 1.0}
    }
    pub fn slateblue() -> Color {
        Color {red: 106u8, green: 90u8, blue: 205u8, alpha: 1.0}
    }
    pub fn slategray() -> Color {
        Color {red: 112u8, green: 128u8, blue: 144u8, alpha: 1.0}
    }
    pub fn slategrey() -> Color {
        Color {red: 112u8, green: 128u8, blue: 144u8, alpha: 1.0}
    }
    pub fn snow() -> Color {
        Color {red: 255u8, green: 250u8, blue: 250u8, alpha: 1.0}
    }
    pub fn springgreen() -> Color {
        Color {red: 0u8, green: 255u8, blue: 127u8, alpha: 1.0}
    }
    pub fn steelblue() -> Color {
        Color {red: 70u8, green: 130u8, blue: 180u8, alpha: 1.0}
    }
    pub fn tan() -> Color {
        Color {red: 210u8, green: 180u8, blue: 140u8, alpha: 1.0}
    }
    pub fn teal() -> Color {
        Color {red: 0u8, green: 128u8, blue: 128u8, alpha: 1.0}
    }
    pub fn thistle() -> Color {
        Color {red: 216u8, green: 191u8, blue: 216u8, alpha: 1.0}
    }
    pub fn tomato() -> Color {
        Color {red: 255u8, green: 99u8, blue: 71u8, alpha: 1.0}
    }
    pub fn turquoise() -> Color {
        Color {red: 64u8, green: 224u8, blue: 208u8, alpha: 1.0}
    }
    pub fn violet() -> Color {
        Color {red: 238u8, green: 130u8, blue: 238u8, alpha: 1.0}
    }
    pub fn wheat() -> Color {
        Color {red: 245u8, green: 222u8, blue: 179u8, alpha: 1.0}
    }
    pub fn white() -> Color {
        Color {red: 255u8, green: 255u8, blue: 255u8, alpha: 1.0}
    }
    pub fn whitesmoke() -> Color {
        Color {red: 245u8, green: 245u8, blue: 245u8, alpha: 1.0}
    }
    pub fn yellow() -> Color {
        Color {red: 255u8, green: 255u8, blue: 0u8, alpha: 1.0}
    }
    pub fn yellowgreen() -> Color {
        Color {red: 154u8, green: 205u8, blue: 50u8, alpha: 1.0}
    }
}
