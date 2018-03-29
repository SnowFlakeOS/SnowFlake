/// A color
#[derive(Copy, Clone)]
pub struct Color(pub u32);

impl Color {
    /// Create a new color from RGB
    pub const fn rgb(r: u8, g: u8, b: u8) -> Self {
        Color(0xFF000000 | ((r as u32) << 16) | ((g as u32) << 8) | (b as u32))
    }

    /// Set the alpha
    pub const fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Color(((a as u32) << 24) | ((r as u32) << 16) | ((g as u32) << 8) | (b as u32))
    }

    /// Get the r value
    pub fn r(&self) -> u8 {
        ((self.0 & 0x00FF0000) >> 16) as u8
    }

    /// Get the g value
    pub fn g(&self) -> u8 {
        ((self.0 & 0x0000FF00) >> 8) as u8
    }

    /// Get the b value
    pub fn b(&self) -> u8 {
        (self.0 & 0x000000FF) as u8
    }

    /// Get the alpha value
    pub fn a(&self) -> u8 {
        ((self.0 & 0xFF000000) >> 24) as u8
    }

    /// Interpolate between two colors
    pub fn interpolate(start_color: Color, end_color: Color, scale: f64) -> Color {
        let r = Color::interp(start_color.r(), end_color.r(), scale);
        let g = Color::interp(start_color.g(), end_color.g(), scale);
        let b = Color::interp(start_color.b(), end_color.b(), scale);
        let a = Color::interp(start_color.a(), end_color.a(), scale);
        Color::rgba(r, g, b, a)
    }

    fn interp(start_color: u8, end_color: u8, scale: f64) -> u8 {
        ((end_color as f64 - start_color as f64) * scale + start_color as f64) as u8
    }
}