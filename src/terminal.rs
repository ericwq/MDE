struct Cell {
    contets: String,
    renditions: Renditions,
}

struct Renditions {
    fg_color: Color,
    bg_color: Color,
    ul_color: Color,
}

pub struct Color(u64);

impl Color {
    // const DEFAULT: Color = Color(0_u64);
    pub const VALID: Color = Color(1_u64 << 32);
    const IS_RGB: Color = Color(1_u64 << 33);
    // const SPECIAL: Color = Color(1_u64 << 34);
    const RED: Color = Color((1_u64 << 32) + 9);
    const GREEN: Color = Color((1_u64 << 32) + 10);

    fn valid(&self) -> bool {
        self.0 & Color::VALID.0 != 0
    }

    fn is_rgb(&self) -> bool {
        (self.0 & Color::VALID.0 | Color::IS_RGB.0) == (Color::VALID.0 | Color::IS_RGB.0)
    }

    fn hex(&self) -> u32 {
        if self.valid() {
            return u32::MAX;
        } else if self.0 & Color::IS_RGB.0 != 0 {
            return self.0 as u32;
        } else {
            0
        }

        // match self {
        //     self.valid()==true => return u32::MAX,
        //     self.0 & Color::IS_RGB.0 !=0 => return self.0 as u32,
        // }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn color_values() {
        let tests = [(Color::RED, 0x00ff0000_u32)];

        for (color, hex) in tests {
            assert_eq!(color.hex(), hex);
        }
    }
}
