use serde::{Serialize, Deserialize};
use glam::{Vec3, Vec2, Mat4, Vec4Swizzles};

/// 3D 공간에서의 위치를 나타내는 구조체
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Position3D {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Position3D {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn zero() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }

    pub fn to_vec3(&self) -> Vec3 {
        Vec3::new(self.x, self.y, self.z)
    }

    pub fn from_vec3(vec: Vec3) -> Self {
        Self::new(vec.x, vec.y, vec.z)
    }

    pub fn distance_to(&self, other: &Position3D) -> f32 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let dz = self.z - other.z;
        (dx * dx + dy * dy + dz * dz).sqrt()
    }

    /// 2D 스크린 좌표로 투영
    pub fn project_to_screen(&self, view_matrix: &Mat4, projection_matrix: &Mat4, viewport: Vec2) -> Vec2 {
        let world_pos = Vec3::new(self.x, self.y, self.z);
        
        // Mat4 곱셈 수정: * 연산자 대신 mul 메서드 사용
        let clip_pos = projection_matrix.mul_vec4(view_matrix.mul_vec4(world_pos.extend(1.0)));
        
        // 정규화된 디바이스 좌표로 변환
        if clip_pos.w != 0.0 {
            let ndc = clip_pos.xyz() / clip_pos.w;
            
            // 스크린 좌표로 변환
            let screen_x = (ndc.x + 1.0) * 0.5 * viewport.x;
            let screen_y = (1.0 - ndc.y) * 0.5 * viewport.y; // Y축 뒤집기
            
            Vec2::new(screen_x, screen_y)
        } else {
            Vec2::new(viewport.x * 0.5, viewport.y * 0.5) // 중앙점 반환
        }
    }

    /// 위치를 특정 반지름과 각도로 업데이트
    pub fn set_polar(&mut self, radius: f32, angle: f32, height: f32) {
        self.x = radius * angle.cos();
        self.y = height;
        self.z = radius * angle.sin();
    }

    /// 극좌표로 변환 (반지름, 각도)
    pub fn to_polar(&self) -> (f32, f32) {
        let radius = (self.x * self.x + self.z * self.z).sqrt();
        let angle = self.z.atan2(self.x);
        (radius, angle)
    }
}

/// 다차원 계층 구조에서 노드의 위치와 계층 정보
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LayerPosition {
    /// 3D 공간에서의 위치
    pub position: Position3D,
    /// 계층 레벨 (0: 최상위, 1: 2단, 2: 3단 등)
    pub layer: usize,
    /// 같은 계층 내에서의 반지름 (케이크의 반지름)
    pub radius: f32,
    /// 각도 (0-360도)
    pub angle: f32,
    /// 계층 높이 오프셋
    pub height_offset: f32,
}

impl LayerPosition {
    pub fn new(layer: usize, radius: f32, angle: f32) -> Self {
        let height_offset = layer as f32 * 100.0; // 각 계층마다 100 단위 높이
        let x = radius * angle.to_radians().cos();
        let z = radius * angle.to_radians().sin();
        let y = height_offset;
        
        Self {
            position: Position3D::new(x, y, z),
            layer,
            radius,
            angle,
            height_offset,
        }
    }

    pub fn update_position(&mut self) {
        let x = self.radius * self.angle.to_radians().cos();
        let z = self.radius * self.angle.to_radians().sin();
        let y = self.height_offset;
        
        self.position = Position3D::new(x, y, z);
    }

    /// 케이크 계층의 색상 계산
    pub fn layer_color(&self) -> [f32; 4] {
        match self.layer {
            0 => [1.0, 0.9, 0.7, 1.0], // 골드 (최상위)
            1 => [0.8, 0.8, 1.0, 1.0], // 라이트 블루 (2단)
            2 => [1.0, 0.8, 0.8, 1.0], // 라이트 핑크 (3단)
            3 => [0.8, 1.0, 0.8, 1.0], // 라이트 그린 (4단)
            _ => [0.9, 0.9, 0.9, 1.0], // 그레이 (그 이상)
        }
    }

    /// 계층 크기 계산 (위로 갈수록 작아짐)
    pub fn layer_size(&self) -> f32 {
        match self.layer {
            0 => 25.0, // 최상위가 가장 큼
            1 => 20.0,
            2 => 15.0,
            3 => 12.0,
            _ => 10.0,
        }
    }
}

// 하위 호환성을 위해 기존 Position2D도 유지
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Position2D {
    pub x: f32,
    pub y: f32,
}

impl Position2D {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn to_position3d(&self, z: f32) -> Position3D {
        Position3D::new(self.x, self.y, z)
    }
} 