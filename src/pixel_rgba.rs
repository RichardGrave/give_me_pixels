#[derive(Debug, PartialEq, Clone, Copy)]
pub struct PixelRgba {
    pub pixel_r: u8,
    pub pixel_g: u8,
    pub pixel_b: u8,
    // At this point pixal_a is not used
    pub pixel_a: u8,
}

impl PixelRgba {
    pub fn new() -> Self {
        Self {
            // Defaults
            pixel_r: 0u8,
            pixel_g: 0u8,
            pixel_b: 0u8,
            pixel_a: 0u8,
        }
    }

    pub fn set_rgba(&mut self, r: u8, g: u8, b: u8, a: u8) {
        self.pixel_r = r;
        self.pixel_g = g;
        self.pixel_b = b;
        self.pixel_a = a;
    }
}
