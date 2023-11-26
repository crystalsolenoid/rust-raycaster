use std::f32::consts::PI;

pub struct Camera {
    pub x: i32,
    pub y: i32,
    pub radians: f32,
    pub fov: f32,
    pub max_distance: f32,
    pub ray_steps: u32,
}

#[derive(Clone, Copy)]
pub struct Ray<T> {
    pub distance: f32,
    pub angle: f32,
    pub wall: Option<T>, //TODO rename to collision?
}

pub fn calculate_ray(distance: f32, angle: f32) -> (i32, i32) {
    let x_off = distance * angle.cos();
    let y_off = distance * angle.sin();
    (x_off as i32, y_off as i32)
}

pub fn calculate_angle(cam: &Camera, span: f32) -> f32 {
    let angle = cam.radians + cam.fov * (span - 0.5);
    const MAX_ANGLE: f32 = 2.0 * PI;
    // angle % MAX_ANGLE but for deranged inconsistent Rust/C math:
    ((angle % MAX_ANGLE) + MAX_ANGLE ) % MAX_ANGLE
}

pub fn cast_ray<T: Copy>(w: u32,
            map: &[Option<T>], cam: &Camera, span: f32) -> Ray<T> {
    // step ranges from 0 to 1: percentage throug the fov
    let angle = calculate_angle(cam, span);
    for step in 0..cam.ray_steps {
        let dist = cam.max_distance * (step as f32) / (cam.ray_steps as f32) ;
        let offset = calculate_ray(dist, angle);
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
                angle: angle,
            }
        }
    }
    return Ray {
        distance: cam.max_distance,
        wall: None,
        angle: angle,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn fov_lowest() {
        let cam = Camera {
            x: 150,
            y: 150,
            radians: PI,
            fov: 2.0 * PI / 3.0,
            max_distance: 512.0,
            ray_steps: 256,
        };
        let result = calculate_angle(&cam, 0.0);
        assert_approx_eq!(result, 2.0 * PI / 3.0);
    }

    #[test]
    fn fov_highest() {
        let cam = Camera {
            x: 150,
            y: 150,
            radians: PI,
            fov: 2.0 * PI / 3.0,
            max_distance: 512.0,
            ray_steps: 256,
        };
        let result = calculate_angle(&cam, 1.0);
        assert_approx_eq!(result, 4.0 * PI / 3.0);
    }

    #[test]
    fn angle_at_zero() {
        let cam = Camera {
            x: 150,
            y: 150,
            radians: 0.0,
            fov: 2.0 * PI / 3.0,
            max_distance: 512.0,
            ray_steps: 256,
        };
        let result = calculate_angle(&cam, 0.0);
        assert_approx_eq!(result, 5.0 * PI / 3.0);
    }

    #[test]
    fn fov_below_zero() {
        let cam = Camera {
            x: 150,
            y: 150,
            radians: 1.0 * PI / 3.0,
            fov: 4.0 * PI / 3.0,
            max_distance: 512.0,
            ray_steps: 256,
        };
        let result = calculate_angle(&cam, 0.0);
        assert_approx_eq!(result, 5.0 * PI / 3.0);
    }

    #[test]
    fn fov_above_2_pi() {
        let cam = Camera {
            x: 150,
            y: 150,
            radians: 5.0 * PI / 3.0,
            fov: 4.0 * PI / 3.0,
            max_distance: 512.0,
            ray_steps: 256,
        };
        let result = calculate_angle(&cam, 1.0);
        assert_approx_eq!(result, 1.0 * PI / 3.0);
    }
}
