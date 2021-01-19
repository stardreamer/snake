#[derive(PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right
}

pub struct CobraSegment {
    pub direction: Direction
}

pub struct Cobra {
    pub segment_width: f32,
    pub segment_height: f32,
    pub velocity: f32,
    pub head_direction: Direction,
    pub segments: Vec<CobraSegment>
}