use image::Rgb;
use std::fs;
use std::f32::consts::PI;

struct Camera {
    x: u32,
    y: u32,
    radians: f32,
    fov: f32,
}

#[derive(Clone, Copy)]
enum Wall {
    Dirt,
    Brick,
    Stone,
}

fn cast_fov(w: u32, img: &mut image::RgbImage,
            map: &[Option<Wall>], cam: &Camera) {
    for i in 0..512 {
        let step = (i as f32) / 512.0;
        cast_ray(w, img, &map, &cam, step);
    }
}

fn cast_ray(w: u32,
            img: &mut image::RgbImage,
            map: &[Option<Wall>], cam: &Camera, span: f32) -> Option<Wall> {
    // step ranges from 0 to 1: percentage throug the fov
    let angle = cam.radians + cam.fov * (span - 0.5);
    const STEPS: u32 = 100;
    const MAX_DIST: f32 = 512.0;
    for step in 0..STEPS {
        let dist = MAX_DIST * (step as f32) / (STEPS as f32) ;
        let x_off = (dist * angle.cos()) as u32;
        let y_off = (dist * angle.sin()) as u32;
        let x = cam.x + x_off;
        let y = cam.y - y_off; // minus because +y is down
        let idx = (x + y * w) as usize;
        // TODO: make this pattern matching
        if map[idx].is_some() {
            return map[idx];
        } else {
            img.put_pixel(x, y, Rgb([86, 50, 38]));
        }
    }
    return None;
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

fn gen_map(w: u32, h: u32) -> Vec<Option<Wall>> {
//    let mut map = vec![None; (w * h) as usize];
    let mut map = Vec::with_capacity((w * h) as usize);
    map.resize((w * h) as usize, None);

    let draw_rect = |map: &mut [Option<Wall>],
                     x1: u32, y1: u32, x2: u32, y2: u32, material: Option<Wall>| {
        for y in y1..y2 {
            for x in x1..x2 {
                let idx = (x + y * w) as usize;
                map[idx] = material;
            }
        }
    };

    let horiz_wall = |map: &mut [Option<Wall>],
                      x1: u32, x2: u32, y1: u32, material: Option<Wall>| {
        draw_rect(map, x1, y1, x2, y1 + THICKNESS, material);
    };

    let vert_wall = |map: &mut [Option<Wall>],
                      y1: u32, y2: u32, x1: u32, material: Option<Wall>| {
        draw_rect(map, x1, y1, x1 + THICKNESS, y2, material);
    };

    // test map
    const THICKNESS: u32 = 32;
    // outer walls
    let mut material = Some(Wall::Dirt);
    draw_rect(&mut map, 0, 0, THICKNESS, h, material);
    draw_rect(&mut map, 0, 0, w, THICKNESS, material);
    draw_rect(&mut map, w - THICKNESS, 0, w, h, material);
    draw_rect(&mut map, 0, h - THICKNESS, w, h, material);
    // inner walls
    // little room
    material = Some(Wall::Stone);
    horiz_wall(&mut map, 0, 150, 200, material);
    horiz_wall(&mut map, 0, 150, 400, material);
    vert_wall(&mut map, 200, 280, 150, material);
    vert_wall(&mut map, 320, 400 + THICKNESS, 150, material);
    // hallway
    material = Some(Wall::Brick);
    vert_wall(&mut map, 100, h, 250, material);
    horiz_wall(&mut map, 100, 250, 100, material);
    horiz_wall(&mut map, 340, w, 100, material);
    horiz_wall(&mut map, 250 + THICKNESS, 450, 170, material);
    // bumps
    material = Some(Wall::Brick);
    vert_wall(&mut map, 450, h, 400, material);
    vert_wall(&mut map, 450, h, 350, material);
    vert_wall(&mut map, 450, h, 300, material);
    // columns
    vert_wall(&mut map, 300, 300 + THICKNESS, 380, material);

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
        radians: 2.0 * PI / 8.0,
        fov: PI/3.0,
    };

    for y in 0..h {
        for x in 0..w {
            let idx = (x + y * w) as usize;
            let wall = map[idx];
            let r;
            let g;
            let b;
            match wall {
                Some(Wall::Dirt) => {
                    r = 86;
                    g = 50;
                    b = 38;
                },
                Some(Wall::Brick) => {
                    r = 246;
                    g = 205;
                    b = 38;
                },
                Some(Wall::Stone) => {
                    r = 57;
                    g = 57;
                    b = 57;
                },
                _ => {
                    r = 32;
                    g = 32;
                    b = 32;
                }
            }
            img.put_pixel(x, y, Rgb([r, g, b]));
        }
    }

    cast_fov(w, &mut img, &map, &camera);

    draw_camera(&mut img, &camera);

    write_image(&img);
}
