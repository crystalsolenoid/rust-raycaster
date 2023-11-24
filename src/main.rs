use image::Rgb;
use std::fs;

fn write_image(img: &image::RgbImage) {
    // TODO: handle errors
    // make the output directory if it doesn't already exist
    let _ = fs::create_dir("output");
    // write the image
    let kind = image::ColorType::Rgb8;
    let dims = img.dimensions();
    let _ = image::save_buffer("output/render.png", img, dims.0, dims.1, kind);
}

fn main() {
    let w = 512;
    let h = 512;
    let mut img = image::RgbImage::new(w, h);

    for y in 0..h {
        for x in 0..w {
            let r = ((255 * y) as f32 / (w as f32)) as u8;
            let g = ((255 * x) as f32 / (w as f32)) as u8;
            let b = 0;
            img.put_pixel(x, y, Rgb([r, g, b]));
        }
    }

    write_image(&img);
}
