use std::ops::{Add, Sub, Mul, Div};

#[derive(Debug, Copy, Clone)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

impl Color {

    /// construct a new color from red, green, blue and alpha
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self {
            r,
            g,
            b,
            a,
        }
    }

    /// construct a new color from red, green and blue
    pub fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self {
            r,
            g,
            b,
            a: std::u8::MAX,
        }
    }

    /// returns normalized (between 0.0 and 1.0) values for all color channels
    pub fn into_normalized(self) -> (f32, f32, f32, f32) {
        (
            self.r as f32 / 255.0,
            self.g as f32 / 255.0,
            self.b as f32 / 255.0,
            self.a as f32 / 255.0,
        )
    }
}

impl Add for Color {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            r: self.r + other.r,
            g: self.g + other.g,
            b: self.b + other.b,
            a: self.a + other.a,
        }
    }
}

impl Sub for Color {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            r: self.r - other.r,
            g: self.g - other.g,
            b: self.b - other.b,
            a: self.a - other.a,
        }
    }
}

impl Mul for Color {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self {
            r: self.r * other.r,
            g: self.g * other.g,
            b: self.b * other.b,
            a: self.a * other.a,
        }
    }
}

impl Div for Color {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        Self {
            r: self.r / other.r,
            g: self.g / other.g,
            b: self.b / other.b,
            a: self.a / other.a,
        }
    }
}

impl Into<(u8, u8, u8, u8)> for Color {
    fn into(self) -> (u8, u8, u8, u8) {
        (self.r, self.g, self.b, self.a)
    }
}

impl From<(u8, u8, u8, u8)> for Color {
    fn from(values: (u8, u8, u8, u8)) -> Self {
        Self::new(values.0, values.1, values.2, values.3)
    }
}
