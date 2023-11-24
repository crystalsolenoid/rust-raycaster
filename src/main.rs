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

fn gen_map(w: u32, h: u32) -> Vec<bool> {
    let mut map = vec![false; (w * h) as usize];

    let mut draw_rect = |x1: u32, y1: u32, x2: u32, y2: u32| {
        // I don't know how a closure works,
        // but the compiler recommended it
        // so here we go!
        for y in y1..y2 {
            for x in x1..x2 {
                let idx = (x + y * w) as usize;
                map[idx] = true;
            }
        }
    };

    // test map
    const THICKNESS: u32 = 32;
    // outer walls
    draw_rect(0, 0, THICKNESS, h);
    draw_rect(0, 0, w, THICKNESS);
    draw_rect(w - THICKNESS, 0, w, h);
    draw_rect(0, h - THICKNESS, w, h);

    map
}

fn main() {
    let w: u32 = 512;
    let h: u32 = 512;
    let mut img = image::RgbImage::new(w, h);

    let map = gen_map(w, h);

    for y in 0..h {
        for x in 0..w {
            let idx = (x + y * w) as usize;
            let wall = map[idx];
            let r;
            let g;
            let b;
            if wall {
                r = 0 as u8;
                g = 128 as u8;
                b = 200 as u8;
            } else {
                r = ((255 * y) as f32 / (w as f32)) as u8;
                g = ((255 * x) as f32 / (w as f32)) as u8;
                b = 0;
            }
            img.put_pixel(x, y, Rgb([r, g, b]));
        }
    }

    write_image(&img);
}
