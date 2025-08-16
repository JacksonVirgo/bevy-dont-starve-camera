use bevy::prelude::*;

#[derive(Component, Default, Clone, Copy)]
pub enum CameraDirection {
    #[default]
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}

impl CameraDirection {
    pub fn xz_unit(&self) -> Vec2 {
        use CameraDirection::*;
        let v = match self {
            North => Vec2::new(0.0, -1.0),
            NorthEast => Vec2::new(1.0, -1.0),
            East => Vec2::new(1.0, 0.0),
            SouthEast => Vec2::new(1.0, 1.0),
            South => Vec2::new(0.0, 1.0),
            SouthWest => Vec2::new(-1.0, 1.0),
            West => Vec2::new(-1.0, 0.0),
            NorthWest => Vec2::new(-1.0, -1.0),
        };
        v.normalize()
    }

    pub fn offset(&self, r_xy: f32, h: f32) -> Vec3 {
        let u = self.xz_unit() * r_xy;
        Vec3::new(u.x, h, u.y)
    }

    pub fn next(&self) -> Self {
        use CameraDirection::*;
        match self {
            North => NorthEast,
            NorthEast => East,
            East => SouthEast,
            SouthEast => South,
            South => SouthWest,
            SouthWest => West,
            West => NorthWest,
            NorthWest => North,
        }
    }

    pub fn prev(&self) -> Self {
        use CameraDirection::*;
        match self {
            North => NorthWest,
            NorthWest => West,
            West => SouthWest,
            SouthWest => South,
            South => SouthEast,
            SouthEast => East,
            East => NorthEast,
            NorthEast => North,
        }
    }
}
