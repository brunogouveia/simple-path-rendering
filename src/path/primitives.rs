use glam::Vec2;

#[derive(Clone, Default, Debug)]
pub struct Line {
    pub c0: Vec2,
    pub c1: Vec2,
}

impl Line {
    pub fn new(c0: Vec2, c1: Vec2) -> Self {
        Self { c0, c1 }
    }
}

#[derive(Clone, Default, Debug)]
pub struct QuadCurve {
    pub c0: Vec2,
    pub c1: Vec2,
    pub c2: Vec2,
}

impl QuadCurve {
    pub fn new(c0: Vec2, c1: Vec2, c2: Vec2) -> Self {
        Self { c0, c1, c2 }
    }
}
