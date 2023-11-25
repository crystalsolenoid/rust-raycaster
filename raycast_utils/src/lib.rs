pub fn add(left: usize, right: usize) -> usize {
    left + right
}

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

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

}
