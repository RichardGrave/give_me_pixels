extern crate image;

mod app_window;
mod pixel;
mod pixel_rgba;

use image::{DynamicImage, Rgb};
use pixel::Pixel;
use pixel_rgba::PixelRgba;
use std::{env, path::Path, process};

use image::GenericImageView;

pub const DEFAULT_COLOR_PALETTE_FILE: &[u8] = include_bytes!(".././default_palette.png");
const DEFAULT_COLOR_PALETTE: &str = "./default_palette.png";
const DEFAULT_FILENAME: &str = "new_pixel_file.png";

//TODO:RG use 'ICED' for the UI

fn main() {
    let arguments: Vec<String> = env::args().collect();

    if arguments.len() >= 3 {
        process_image(&arguments[1], &arguments[2]);
    } else if arguments.len() == 2 {
        process_image(&arguments[1], &DEFAULT_COLOR_PALETTE.to_string());
    } else {
        println!("\nNo params found. Example: ");
        println!("./give_me_pixels image_file palette_file\n");
        println!("If no palette file is given then the default is used");
    }
}

fn process_image(file_path: &String, palette_name: &String) {
    let mut pixel_vec: Vec<Pixel> = Vec::<Pixel>::new();
    let (width, heigth) = read_image(&file_path.to_string(), &mut pixel_vec);

    if pixel_vec.len() > 0 {
        println!("Get color palette");
        // After reading the image file we can proceed with the color palette
        let color_palette: Vec<PixelRgba> = get_palette_colors(&palette_name);

        if color_palette.len() == 0 {
            // In this case we need to stop
            println!("Can't find a color palette");
            println!("Exiting program");
            process::exit(1);
        }

        println!("Size pixel_vec: {}", pixel_vec.len());
        calculate_new_pixels(&mut pixel_vec, &color_palette);

        write_new_image(width, heigth, &file_path.to_string(), &pixel_vec);
    }
}

fn get_palette_colors(palette_name: &String) -> Vec<PixelRgba> {
    let mut color_palette = Vec::<PixelRgba>::new();

    // Read the color palette image
    if let Ok(img_palette) = image::open(&Path::new(&palette_name)) {
        color_palette = read_pixels_for_color_palette(&img_palette);
    }

    // If the palette is empty then we need to fall back to the default
    if color_palette.len() == 0 {
        println!("Going to use the default color palette");
        if let Ok(default) = image::load_from_memory(DEFAULT_COLOR_PALETTE_FILE) {
            color_palette = read_pixels_for_color_palette(&default);
        }
    }

    color_palette
}

fn read_pixels_for_color_palette(image: &DynamicImage) -> Vec<PixelRgba> {
    let mut color_palette = Vec::<PixelRgba>::new();
    let mut tmp_rgba_pixel = Vec::<image::Rgba<u8>>::new();

    for pix in image.pixels() {
        // Filter out duplicate rgba's
        if !tmp_rgba_pixel.contains(&pix.2) {
            // Save for comparing with other rgba's
            tmp_rgba_pixel.push(pix.2);

            let mut pixel_rgba = PixelRgba::new();
            // Rgba[r, g, b, a]
            pixel_rgba.set_rgba(pix.2[0], pix.2[1], pix.2[2], pix.2[3]);

            // Save the rgba
            color_palette.push(pixel_rgba);
        }
    }

    color_palette
}

fn calculate_new_pixels(pixel_vec: &mut Vec<Pixel>, color_palette: &Vec<PixelRgba>) {
    println!("Calculating the new pixels");

    let one_procent = pixel_vec.len() / 100usize;
    let mut count_pixels = 0usize;
    let mut count_procent = 0usize;

    for pixel in pixel_vec {
        count_pixels += 1;

        // Tried using Threads, but it seems to be 5 times slower.
        // Don't know why. Maybe creating the Threads takes more time
        pixel.calculate_new_pixels_by_color_palette(&color_palette);

        if count_pixels % one_procent == 0 {
            count_procent += 1;
            println!("{}%", count_procent);
        }
    }
}

fn read_image(file_path: &String, pixel_vec: &mut Vec<Pixel>) -> (u32, u32) {
    let mut width = 0u32;
    let mut heigth = 0u32;

    if let Ok(img_open) = image::open(&Path::new(&file_path)) {
        let (img_width, img_height) = img_open.dimensions();
        width = img_width;
        heigth = img_height;

        println!("Dimensions width: {} - height: {}", img_width, img_height);

        for pix in img_open.pixels() {
            let mut new_pixel = Pixel::new();
            // The pix contains -> position_x, position_y, Rgba[r, g, b, a]
            new_pixel.set_pixel(pix.0, pix.1, pix.2[0], pix.2[1], pix.2[2], pix.2[3]);

            // Save the Pixel
            pixel_vec.push(new_pixel);
        }
    }

    (width, heigth)
}

fn write_new_image(width: u32, height: u32, file_path: &String, pixel_vec: &Vec<Pixel>) {
    println!("{:?}", file_path);

    let first_part_filename = get_new_filename(&file_path);
    let new_filename = format!("{}{}{}", "./new_", first_part_filename, ".png");

    // Create a new image buffer for all the pixels
    let mut new_image_buffer = image::ImageBuffer::new(width, height);

    for pix in pixel_vec {
        // Set each new pixel
        new_image_buffer.put_pixel(
            pix.position_x,
            pix.position_y,
            Rgb([
                pix.new_rgba.pixel_r,
                pix.new_rgba.pixel_g,
                pix.new_rgba.pixel_b,
            ]),
        );
    }

    println!("Saving");

    // Always save as PNG image format
    if let Ok(_) =
        new_image_buffer.save_with_format(&Path::new(&new_filename), image::ImageFormat::Png)
    {
        println!("Saved to: {}", &new_filename);
    } else {
        println!("File not created");
    }
}

fn get_new_filename(file_path: &String) -> String {
    // If something goes wrong then we have the default filename
    let mut new_filename = DEFAULT_FILENAME.to_string();

    // Last from split should return the filename
    if let Some(full_filename) = file_path.split('/').collect::<Vec<&str>>().last() {
        // First from this split should return the filename without extension (.jpg, .png, ...)
        if let Some(filename) = full_filename.split('.').collect::<Vec<&str>>().first() {
            new_filename = filename.to_string();
        }
    }

    new_filename
}
