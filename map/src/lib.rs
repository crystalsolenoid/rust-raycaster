pub struct Map {
    pub map: Vec<Option<Wall>>,
    pub w: u32,
    pub h: u32,
}

#[derive(Clone, Copy)]
pub enum Wall {
    Dirt,
    Brick,
    Stone,
    Crystal,
}

impl Map {
    const WALL_THICKNESS: u32 = 32;

    pub fn new(w: u32, h: u32) -> Self {
        let mut map = Vec::with_capacity((w * h) as usize);
        map.resize((w * h) as usize, None);
        Self {
            map,
            w,
            h,
        }
    }

    fn draw_rect(&mut self, x1: u32, y1: u32, x2: u32, y2: u32, material: Option<Wall>) {
        for y in y1..y2 {
            for x in x1..x2 {
                let idx = (x + y * self.w) as usize;
                self.map[idx] = material;
            }
        }
    }

    fn horiz_wall(&mut self, x1: u32, x2: u32, y1: u32, material: Option<Wall>) {
        self.draw_rect(x1, y1, x2, y1 + Self::WALL_THICKNESS, material);
    }

    fn vert_wall(&mut self, y1: u32, y2: u32, x1: u32, material: Option<Wall>) {
        self.draw_rect(x1, y1, x1 + Self::WALL_THICKNESS, y2, material);
    }

    pub fn into_values(self) -> Vec<Option<Wall>> {
        self.map
    }

}

pub fn spooky_map() -> Map {
    let mut map = Map::new(512, 512);

    let mut material;

    material = Some(Wall::Stone);
    map.horiz_wall(0, 512, 0, material);
    map.horiz_wall(0, 512, 512 - 32, material);
    map.vert_wall(0, 512, 0, material);
    map.vert_wall(0, 512, 512 - 32, material);

    material = Some(Wall::Dirt);
    map.horiz_wall(224, 256, 480, material);

    material = Some(Wall::Stone);
    map.vert_wall(64, 192, 64, material);
    map.vert_wall(224, 352, 64, material);
    map.vert_wall(384, 448, 64, material);

    map.horiz_wall(96, 448, 64, material);

    material = Some(Wall::Brick);
    map.horiz_wall(128, 160, 128, material);
    map.horiz_wall(224, 256, 128, material);
    map.horiz_wall(320, 352, 128, material);

    map.horiz_wall(128, 160, 256, material);
    map.horiz_wall(224, 256, 256, material);
    map.horiz_wall(320, 352, 256, material);

    material = Some(Wall::Crystal);
    map.horiz_wall(384, 416, 192, material);

    material = Some(Wall::Stone);
    map.horiz_wall(64, 480, 320, material);

    map.vert_wall(64, 288, 416, material);

    map.vert_wall(384, 448, 64, material);
    map.vert_wall(384, 480, 160, material);
    map.vert_wall(384, 480, 288, material);

    map.horiz_wall(64, 224, 384, material);
    map.horiz_wall(256, 384, 384, material);
    map.horiz_wall(416, 480, 384, material);

    map
}


pub fn gen_map(w: u32, h: u32) -> Map {
    // hard-coded test map
    let mut map = Map::new(w, h);

    // outer walls
    let mut material = Some(Wall::Dirt);
    map.draw_rect(0, 0, Map::WALL_THICKNESS, h, material);
    map.draw_rect(0, 0, w, Map::WALL_THICKNESS, material);
    map.draw_rect(w - Map::WALL_THICKNESS, 0, w, h, material);
    map.draw_rect(0, h - Map::WALL_THICKNESS, w, h, material);

    // inner walls
    // little room
    material = Some(Wall::Stone);
    map.horiz_wall(0, 150, 200, material);
    map.horiz_wall(0, 150, 400, material);
    map.vert_wall(200, 280, 150, material);
    map.vert_wall(320, 400 + Map::WALL_THICKNESS, 150, material);
    map.vert_wall(200, 400, 0, material);
    // hallway
    material = Some(Wall::Brick);
    map.vert_wall(100, h, 250, material);
    map.horiz_wall(100, 250, 100, material);
    map.horiz_wall(340, w, 100, material);
    map.horiz_wall(250 + Map::WALL_THICKNESS, 450, 170, material);
    // bumps
    material = Some(Wall::Brick);
    map.vert_wall(450, h, 400, material);
    map.vert_wall(450, h, 350, material);
    map.vert_wall(450, h, 300, material);
    // columns
    material = Some(Wall::Stone);
    map.vert_wall(300, 300 + Map::WALL_THICKNESS, 380, material);

    map
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        //let result = add(2, 2);
        //assert_eq!(result, 4);
    }
}
