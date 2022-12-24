use std::io::{self, Write};

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
}

impl Color {
    /// Creates a color from RGB intensity.
    /// Each channel must be from 0 to 100.
    ///
    /// # Panics
    /// Panics if an intensity is over 100.
    pub fn from_intensity([r, g, b]: [u8; 3]) -> Self {
        assert!(r <= 100 && g <= 100 && b <= 100);
        Self { r, g, b }
    }

    pub fn from_rgb_u8(array: [u8; 3]) -> Self {
        Self::from_intensity(array.map(u8_to_intensity))
    }

    pub fn from_rgb_f32(array: [f32; 3]) -> Self {
        Self::from_intensity(array.map(f32_to_intensity))
    }

    pub fn intensity(self) -> [u8; 3] {
        [self.r, self.g, self.b]
    }

    pub fn difference(self, rhs: Self) -> [u8; 3] {
        [
            self.r.abs_diff(rhs.r),
            self.g.abs_diff(rhs.g),
            self.b.abs_diff(rhs.b),
        ]
    }

    pub(crate) fn write<W>(self, index: usize, mut out: W) -> Result<(), io::Error>
    where
        W: Write,
    {
        let Self { r, g, b } = self;
        write!(out, "#{index};2;{r};{g};{b}")
    }
}

impl From<[u8; 3]> for Color {
    fn from(v: [u8; 3]) -> Self {
        Self::from_rgb_u8(v)
    }
}

impl From<[f32; 3]> for Color {
    fn from(v: [f32; 3]) -> Self {
        Self::from_rgb_f32(v)
    }
}

fn u8_to_intensity(v: u8) -> u8 {
    ((v as u16 * 100) / 255) as _
}

fn f32_to_intensity(v: f32) -> u8 {
    (v * 100.) as _
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_rgb_u8() {
        let actual = Color::from_rgb_u8([0, 128, !0]);
        let expected = Color::from_intensity([0, 50, 100]);
        assert_eq!(actual, expected);
    }

    #[test]
    fn from_rgb_f32() {
        let actual = Color::from_rgb_f32([0., 0.5, 1.]);
        let expected = Color::from_intensity([0, 50, 100]);
        assert_eq!(actual, expected);
    }
}
