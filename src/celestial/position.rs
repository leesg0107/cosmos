use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Position2D {
    pub x: f32,
    pub y: f32,
    pub z: f32,  // 레이어 순서를 위한 z 좌표
}

impl Position2D {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y, z: 0.0 }
    }

    pub fn to_screen_pos(&self) -> (f32, f32) {
        (self.x, self.y)
    }
} 