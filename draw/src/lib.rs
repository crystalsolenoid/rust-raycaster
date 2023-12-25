use map::{Map, Wall};
use image::Rgb;
use raycast_utils::{Ray, Camera};
use std::cmp;

const PALETTE: [Rgb<u8>; 8] = [
    // Rust Gold 8 Palette
    // https://lospec.com/palette-list/rust-gold-8
    Rgb([246, 205, 38]), // Gold
    Rgb([172, 107, 38]), // Orange
    Rgb([86, 50, 38]),   // Rust
    Rgb([51, 28, 23]),   // Maroon
    Rgb([187, 127, 87]), // Creamsicle
    Rgb([114, 89, 86]),  // Purple
    Rgb([57, 57, 57]),   // Gray
    Rgb([32, 32, 32]),   // Black
];

fn pick_color(wall: Option<Wall>) -> Rgb<u8> {
    match wall {
        Some(Wall::Dirt) => PALETTE[2],
        Some(Wall::Brick) => PALETTE[0],
        Some(Wall::Stone) => PALETTE[6],
        Some(Wall::Crystal) => PALETTE[4],
        None => PALETTE[7],
    }
}

const HORIZONTAL_COLORS: [Rgb<u8>; 2] = [
    PALETTE[7], // Ceiling
    PALETTE[3], // Floor
];

pub fn draw_map(img: &mut image::RgbImage, map: &Map) {
    for y in 0..map.h {
        for x in 0..map.w {
            let idx = (x + y * map.w) as usize;
            let wall = map.map[idx];
            let color = pick_color(wall);
            
            img.put_pixel(x, y, color);
        }
    }
}

pub fn draw_view(img: &mut image::RgbImage, view: &[Ray<Wall>], cam: &Camera) {
    let heights = raycast_utils::calculate_heights(view, cam);
    for (i, ray) in view.iter().enumerate() {
        let color = pick_color(ray.wall);
        let mut from_axis = heights[i] as u32;
        from_axis = cmp::min(from_axis, cam.max_distance as u32);
        let x = 511 - i as u32;
        for y in 0..from_axis as u32 {
            // make sure there's no out of bounds errors
            if let Some(p) = img.get_pixel_mut_checked(x, 256 + y) {
                *p = color;
            }
            if let Some(p) = img.get_pixel_mut_checked(x, 256_u32.saturating_sub(y)) {
                *p = color;
            }
        }
        for y in 512/2 + from_axis..512 as u32 {
            // floor
            if let Some(p) = img.get_pixel_mut_checked(x, y) {
                *p = HORIZONTAL_COLORS[1];
            }
        }
        for y in 0..=256_u32.saturating_sub(from_axis) as u32 {
            // ceiling
            if let Some(p) = img.get_pixel_mut_checked(x, y) {
                *p = HORIZONTAL_COLORS[0];
            }
        }
    }
}

pub fn draw_ray(img: &mut image::RgbImage, cam: &Camera, ray: &Ray<Wall>) {
    // for debug
    for step in 0..cam.ray_steps {
        let dist = cam.max_distance * (step as f32) / (cam.ray_steps as f32);
        let offset = raycast_utils::calculate_ray(dist, ray.angle);
        let x_off = offset.0;
        let y_off = offset.1;
        let x = (cam.x + x_off) as u32;
        let y = (cam.y - y_off) as u32;
        if dist == ray.distance {
            break;
        }
        img.put_pixel(x, y, PALETTE[2]);
    }
}

pub fn draw_fov(img: &mut image::RgbImage, view: &[Ray<Wall>], cam: &Camera) {
    for ray in view {
        draw_ray(img, cam, ray);
    }
}

pub fn draw_camera(img: &mut image::RgbImage, camera: &Camera) {
    // crosshairs for camera location
    for x in camera.x - 10..=camera.x + 10 {
        img.put_pixel(x as u32, camera.y as u32, PALETTE[0]);
    }
    for y in camera.y - 10..=camera.y + 10 {
        img.put_pixel(camera.x as u32, y as u32, PALETTE[0]);
    }
}

#[cfg(test)]
mod tests {
    //use super::*;
}
