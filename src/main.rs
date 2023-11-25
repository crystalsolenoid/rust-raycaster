use image::Rgb;
use std::fs;
use std::f32::consts::PI;

struct Camera {
    x: u32,
    y: u32,
    angle: f32,
    fov: f32,
}

fn draw_camera(img: &mut image::RgbImage, camera: &Camera) {
    // crosshairs for camera location
    for x in camera.x - 10 ..= camera.x + 10 {
        img.put_pixel(x, camera.y, Rgb([246, 205, 38]));
    }
    for y in camera.y - 10 ..= camera.y + 10 {
        img.put_pixel(camera.x, y, Rgb([246, 205, 38]));
    }
}

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

    let draw_rect = |map: &mut Vec<bool>,
                     x1: u32, y1: u32, x2: u32, y2: u32| {
        for y in y1..y2 {
            for x in x1..x2 {
                let idx = (x + y * w) as usize;
                map[idx] = true;
            }
        }
    };

    let horiz_wall = |map: &mut Vec<bool>,
                      x1: u32, x2: u32, y1: u32| {
        draw_rect(map, x1, y1, x2, y1 + THICKNESS);
    };

    let vert_wall = |map: &mut Vec<bool>,
                      y1: u32, y2: u32, x1: u32| {
        draw_rect(map, x1, y1, x1 + THICKNESS, y2);
    };

    // test map
    const THICKNESS: u32 = 32;
    // outer walls
    draw_rect(&mut map, 0, 0, THICKNESS, h);
    draw_rect(&mut map, 0, 0, w, THICKNESS);
    draw_rect(&mut map, w - THICKNESS, 0, w, h);
    draw_rect(&mut map, 0, h - THICKNESS, w, h);
    // inner walls
    // little room
    horiz_wall(&mut map, 0, 150, 200);
    horiz_wall(&mut map, 0, 150, 400);
    vert_wall(&mut map, 200, 280, 150);
    vert_wall(&mut map, 320, 400 + THICKNESS, 150);
    // hallway
    vert_wall(&mut map, 100, h, 250);
    horiz_wall(&mut map, 100, 250, 100);
    horiz_wall(&mut map, 340, w, 100);
    horiz_wall(&mut map, 250 + THICKNESS, 450, 170);
    // bumps
    vert_wall(&mut map, 450, h, 400);
    vert_wall(&mut map, 450, h, 350);
    vert_wall(&mut map, 450, h, 300);
    // columns
    vert_wall(&mut map, 300, 300 + THICKNESS, 380);

    map
}

fn main() {
    let w: u32 = 512;
    let h: u32 = 512;
    let mut img = image::RgbImage::new(w, h);

    let map = gen_map(w, h);
    let camera = Camera {
        x: 80,
        y: 310,
        angle: 0.0,
        fov: PI/3.0,
    };

    for y in 0..h {
        for x in 0..w {
            let idx = (x + y * w) as usize;
            let wall = map[idx];
            let r;
            let g;
            let b;
            if wall {
                r = 114;
                g = 89;
                b = 86;
            } else {
                r = 32;
                g = 32;
                b = 32;
            }
            img.put_pixel(x, y, Rgb([r, g, b]));
        }
    }

    draw_camera(&mut img, &camera);

    write_image(&img);
}
