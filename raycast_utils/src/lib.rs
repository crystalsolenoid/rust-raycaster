use std::f32::consts::PI;
use map::{Wall, Map};

pub struct Camera {
    pub x: i32,
    pub y: i32,
    pub height: i32,
    pub radians: f32,
    pub fov: f32,
    pub max_distance: f32,
    pub ray_steps: u32,
}

impl Camera {
    pub fn default() -> Self {
        Self {
            x: 0,
            y: 0,
            height: 32,
            radians: 0.0,
            fov: PI / 3.0,
            max_distance: 512.0,
            ray_steps: 512,
        }
    }
}

#[derive(Clone, Copy)]
pub struct Ray<T> {
    pub distance: f32,
    pub angle: f32,
    pub wall: Option<T>, //TODO rename to collision?
}

pub fn calculate_heights<T>(rays: &[Ray<T>], cam: &Camera) -> Vec<f32> {
    let mut heights = vec![0.0; rays.len()];
    for (i, ray) in rays.iter().enumerate() {
        let from_axis = 14.0 * cam.max_distance / ray.distance;
        heights[i] = from_axis;
    }
    heights
}

pub fn calculate_ray(distance: f32, angle: f32) -> (i32, i32) {
    let x_off = distance * angle.cos();
    let y_off = distance * angle.sin();
    (x_off as i32, y_off as i32)
}

pub fn calculate_angle(cam: &Camera, span: f32) -> f32 {
    // span: value between 0.0 and 1.0 that's the percent through the field of view
    let angle = cam.radians + cam.fov * (span - 0.5);
    const MAX_ANGLE: f32 = 2.0 * PI;
    // angle % MAX_ANGLE but for deranged inconsistent Rust/C math:
    ((angle % MAX_ANGLE) + MAX_ANGLE) % MAX_ANGLE
}

pub fn cast_ray(map: &Map, cam: &Camera, span: f32) -> Ray<Wall> {
    // step ranges from 0 to 1: percentage throug the fov
    let angle = calculate_angle(cam, span);
    for step in 0..cam.ray_steps {
        let dist = cam.max_distance * (step as f32) / (cam.ray_steps as f32);
        let offset = calculate_ray(dist, angle);
        let x_off = offset.0;
        let y_off = offset.1;
        let x = (cam.x + x_off) as u32;
        let y = (cam.y - y_off) as u32; // minus because +y is down
        let idx = (x + y * map.w) as usize;
        // TODO: make this pattern matching
        if map.map[idx].is_some() {
            return Ray {
                distance: dist,
                wall: map.map[idx],
                angle: angle,
            };
        }
    }
    return Ray {
        distance: cam.max_distance,
        wall: None,
        angle: angle,
    };
}

pub fn cast_fov(map: &Map, cam: &Camera) -> Vec<Ray<Wall>> {
    let mut view = Vec::with_capacity(map.w as usize);
    view.resize(
        map.w as usize,
        Ray {
            distance: cam.max_distance,
            wall: None,
            angle: 0.0,
        },
    );
    for i in 0..512 {
        let step = (i as f32) / cam.max_distance;
        let ray = cast_ray(&map, &cam, step);
        view[i as usize] = ray;
    }
    view
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn fov_lowest() {
        let cam = Camera {
            radians: PI,
            fov: 2.0 * PI / 3.0,
            ..Camera::default()
        };
        let result = calculate_angle(&cam, 0.0);
        assert_approx_eq!(result, 2.0 * PI / 3.0);
    }

    #[test]
    fn fov_highest() {
        let cam = Camera {
            radians: PI,
            fov: 2.0 * PI / 3.0,
            ..Camera::default()
        };
        let result = calculate_angle(&cam, 1.0);
        assert_approx_eq!(result, 4.0 * PI / 3.0);
    }

    #[test]
    fn angle_at_zero() {
        let cam = Camera {
            radians: 0.0,
            fov: 2.0 * PI / 3.0,
            ..Camera::default()
        };
        let result = calculate_angle(&cam, 0.0);
        assert_approx_eq!(result, 5.0 * PI / 3.0);
    }

    #[test]
    fn fov_below_zero() {
        let cam = Camera {
            radians: 1.0 * PI / 3.0,
            fov: 4.0 * PI / 3.0,
            ..Camera::default()
        };
        let result = calculate_angle(&cam, 0.0);
        assert_approx_eq!(result, 5.0 * PI / 3.0);
    }

    #[test]
    fn fov_above_2_pi() {
        let cam = Camera {
            radians: 5.0 * PI / 3.0,
            fov: 4.0 * PI / 3.0,
            ..Camera::default()
        };
        let result = calculate_angle(&cam, 1.0);
        assert_approx_eq!(result, 1.0 * PI / 3.0);
    }
}
