use crate::pixel_rgba;
use pixel_rgba::PixelRgba;

#[derive(Debug)]
pub struct Pixel {
    // We could have used the image:Rgba, but I wanted something very simple
    pub original_rgba: PixelRgba,
    pub new_rgba: PixelRgba,
    pub position_x: u32,
    pub position_y: u32,
}

impl Pixel {
    pub fn new() -> Self {
        Self {
            // We could use the image::rgba, but I wanted something very simple
            original_rgba: PixelRgba::new(),
            new_rgba: PixelRgba::new(),

            position_x: 0u32,
            position_y: 0u32,
        }
    }

    pub fn set_pixel(&mut self, position_x: u32, position_y: u32, r: u8, g: u8, b: u8, a: u8) {
        self.original_rgba.set_rgba(r, g, b, a);

        self.position_x = position_x;
        self.position_y = position_y;
    }

    pub fn calculate_new_pixels_by_color_palette(&mut self, color_palette: &Vec<PixelRgba>) {
        // As we need the lowest distance, we will start with usize::MAX.
        // This way the first tmp_distance is always lower
        let mut rgba_distance = usize::MAX;

        for rgba in color_palette {
            let tmp_distance = get_euclidean_distance(&self.original_rgba, &rgba);

            // Each time we find a tmp_distance that is lower then the current,
            // it will become the new_rgba color
            if tmp_distance < rgba_distance {
                rgba_distance = tmp_distance;
                self.new_rgba
                    .set_rgba(rgba.pixel_r, rgba.pixel_g, rgba.pixel_b, rgba.pixel_a);
            }
        }
    }
}

fn get_euclidean_distance(original: &PixelRgba, color_palette: &PixelRgba) -> usize {
    // Calculating the differences
    let difference_r = original.pixel_r as usize - color_palette.pixel_r as usize;
    let difference_g = original.pixel_g as usize - color_palette.pixel_g as usize;
    let difference_b = original.pixel_b as usize - color_palette.pixel_b as usize;

    // Use differences in the euclidean formula
    difference_r.pow(2) + difference_g.pow(2) + difference_b.pow(2)
}
