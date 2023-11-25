pub struct Camera {
    pub x: u32,
    pub y: u32,
    pub radians: f32,
    pub fov: f32,
}

#[derive(Clone, Copy)]
pub struct Ray<T> {
    pub distance: f32,
    pub wall: Option<T>, //TODO rename to collision?
}

pub fn calculate_ray(distance: f32, angle: f32) -> (u32, u32) {
    let x_off = distance * angle.cos();
    let y_off = distance * angle.sin();
    (x_off as u32, y_off as u32)
}

pub fn calculate_angle(cam: &Camera, span: f32) -> f32 {
    cam.radians - cam.fov * (span - 0.5)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f32::consts::PI;

    #[test]
    fn fov_lowest() {
        let cam = Camera {
            x: 150,
            y: 150,
            radians: PI,
            fov: 2.0 * PI / 3.0,
        };
        let result = calculate_angle(&cam, 0.0);
        assert_eq!(result, 2.0 * PI / 3.0);
    }

    #[test]
    fn fov_highest() {
        let cam = Camera {
            x: 150,
            y: 150,
            radians: PI,
            fov: 2.0 * PI / 3.0,
        };
        let result = calculate_angle(&cam, 1.0);
        assert_eq!(result, 4.0 * PI / 3.0);
    }

    #[test]
    fn angle_at_zero() {
        let cam = Camera {
            x: 150,
            y: 150,
            radians: 0.0,
            fov: 2.0 * PI / 3.0,
        };
        let result = calculate_angle(&cam, 0.0);
        assert_eq!(result, 5.0 * PI / 3.0);
    }

    #[test]
    fn fov_below_zero() {
        let cam = Camera {
            x: 150,
            y: 150,
            radians: 1.0 * PI / 3.0,
            fov: 4.0 * PI / 3.0,
        };
        let result = calculate_angle(&cam, 0.0);
        assert_eq!(result, 5.0 * PI / 3.0);
    }

    #[test]
    fn fov_above_2_pi() {
        let cam = Camera {
            x: 150,
            y: 150,
            radians: 5.0 * PI / 3.0,
            fov: 4.0 * PI / 3.0,
        };
        let result = calculate_angle(&cam, 0.0);
        assert_eq!(result, 1.0 * PI / 3.0);
    }
}
