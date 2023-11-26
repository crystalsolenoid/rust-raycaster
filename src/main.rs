use image::Rgb;
use std::cmp;
use std::fs;
use std::f32::consts::PI;
use raycast_utils;
use raycast_utils::{Camera, Ray};

const PALETTE: [Rgb<u8>; 8] = [
    // Rust Gold 8 Palette
    // https://lospec.com/palette-list/rust-gold-8
    Rgb([246, 205, 38]), // Gold
    Rgb([172, 107, 38]), // Orange
    Rgb([86, 50, 38]), // Rust
    Rgb([51, 28, 23]), // Maroon
    Rgb([187, 127, 87]), // Creamsicle
    Rgb([114, 89, 86]), // Purple
    Rgb([57, 57, 57]), // Gray
    Rgb([32, 32, 32]), // Black
];

#[derive(Clone, Copy)]
enum Wall {
    Dirt,
    Brick,
    Stone,
}

fn pick_color(wall: Option<Wall>) -> Rgb<u8> {
    match wall {
        Some(Wall::Dirt) => {
            PALETTE[2]
        },
        Some(Wall::Brick) => {
            PALETTE[0]
        },
        Some(Wall::Stone) => {
            PALETTE[6]
        },
        None => {
            PALETTE[7]
        }
    }
}

fn draw_view(img: &mut image::RgbImage, view: &[Ray<Wall>]) {
    for x in 0 .. 512 {
        let ray = view[(511 - x) as usize];
        let color = pick_color(ray.wall);
        let mut from_axis = (512.0 / ray.distance) as u32;
        from_axis = cmp::min(from_axis, 512);
        for y in 0..from_axis as u32 {
            img.put_pixel(x, 256 + y, color);
            img.put_pixel(x, 256 - y, color);
        }
    }
}

fn cast_fov(w: u32, img: &mut image::RgbImage,
            map: &[Option<Wall>], cam: &Camera) -> Vec<Ray<Wall>> {
    let mut view = Vec::with_capacity(w as usize);
    view.resize(w as usize, Ray{ distance: 512.0, wall: None });

    for i in 0..512 {
        let step = (i as f32) / 512.0;
        view[i as usize] = cast_ray(w, img, &map, &cam, step);
    }

    view
}

fn cast_ray(w: u32,
            img: &mut image::RgbImage,
            map: &[Option<Wall>], cam: &Camera, span: f32) -> Ray<Wall> {
    // step ranges from 0 to 1: percentage throug the fov
    let angle = raycast_utils::calculate_angle(cam, span);
    const STEPS: u32 = 100;
    const MAX_DIST: f32 = 512.0;
    for step in 0..STEPS {
        let dist = MAX_DIST * (step as f32) / (STEPS as f32) ;
        let offset = raycast_utils::calculate_ray(dist, angle);
        let x_off = offset.0;
        let y_off = offset.1;
        let x = (cam.x + x_off) as u32;
        let y = (cam.y - y_off) as u32; // minus because +y is down
        let idx = (x + y * w) as usize;
        // TODO: make this pattern matching
        if map[idx].is_some() {
            return Ray {
                distance: dist,
                wall: map[idx],
            }
        } else {
            img.put_pixel(x, y, PALETTE[2]);
        }
    }
    return Ray {
        distance: MAX_DIST,
        wall: None,
    }
}

fn draw_camera(img: &mut image::RgbImage, camera: &Camera) {
    // crosshairs for camera location
    for x in camera.x - 10 ..= camera.x + 10 {
        img.put_pixel(x as u32, camera.y as u32, PALETTE[0]);
    }
    for y in camera.y - 10 ..= camera.y + 10 {
        img.put_pixel(camera.x as u32, y as u32, PALETTE[0]);
    }
}

fn write_image(img: &image::RgbImage, fname: &str) {
    // TODO: handle errors
    // make the output directory if it doesn't already exist
    let _ = fs::create_dir("output");
    // write the image
    let kind = image::ColorType::Rgb8;
    let dims = img.dimensions();
    let _ = image::save_buffer("output/".to_string() + fname, img, dims.0, dims.1, kind);
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
    vert_wall(&mut map, 200, 400, 0, material);
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
    material = Some(Wall::Stone);
    vert_wall(&mut map, 300, 300 + THICKNESS, 380, material);

    map
}

fn main() {
    let w: u32 = 512;
    let h: u32 = 512;
    let mut img = image::RgbImage::new(w, h);
    let mut render = image::RgbImage::new(w, h);

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
            let color = pick_color(wall);

            img.put_pixel(x, y, color);
        }
    }

    draw_camera(&mut img, &camera);

    let view = cast_fov(w, &mut img, &map, &camera);

    write_image(&img, "map.png");

    draw_view(&mut render, &view);

    write_image(&render, "render.png");
}
