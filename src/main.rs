use raycast_utils::{Camera};
use std::f32::consts::PI;
use std::fs;

fn write_image(img: &image::RgbImage, fname: &str) {
    // TODO: handle errors
    // make the output directory if it doesn't already exist
    let _ = fs::create_dir("output");
    // write the image
    let kind = image::ColorType::Rgb8;
    let dims = img.dimensions();
    let _ = image::save_buffer("output/".to_string() + fname, img, dims.0, dims.1, kind);
}

fn main() {
    let w: u32 = 512;
    let h: u32 = 512;
    let mut img = image::RgbImage::new(w, h);
    let mut render = image::RgbImage::new(w, h);

    let map = map::gen_map(w, h);
    let camera = Camera {
        x: 380,
        y: 340,
        radians: 0.3 * PI,
        fov: 0.5 * PI,
        max_distance: 512.0,
        ray_steps: 256,
    };

    draw::draw_map(&mut img, &map);

    draw::draw_camera(&mut img, &camera);

    let view = raycast_utils::cast_fov(&map, &camera);
    draw::draw_fov(&mut img, &view, &camera);

    write_image(&img, "map.png");

    draw::draw_view(&mut render, &view, &camera);

    write_image(&render, "render.png");
}
