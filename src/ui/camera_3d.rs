use glam::{Vec3, Mat4, Quat};

#[derive(Debug, Clone)]
pub struct Camera3D {
    /// 카메라 위치
    pub position: Vec3,
    /// 카메라가 보는 방향 (정규화된 벡터)
    pub direction: Vec3,
    /// 카메라의 위쪽 방향
    pub up: Vec3,
    /// 시야각 (Field of View in degrees)
    pub fov: f32,
    /// 화면 비율 (width / height)
    pub aspect_ratio: f32,
    /// 근접 클리핑 평면
    pub near: f32,
    /// 원거리 클리핑 평면
    pub far: f32,
    
    // 궤도 카메라 속성 (케이크 구조를 둘러보기 위해)
    pub orbit_target: Vec3,  // 카메라가 돌고 있는 중심점
    pub orbit_radius: f32,   // 중심점으로부터의 거리
    pub orbit_angle_horizontal: f32, // 수평 각도 (degrees)
    pub orbit_angle_vertical: f32,   // 수직 각도 (degrees)
    
    // 줌 제어
    pub zoom_speed: f32,
    pub min_zoom: f32,
    pub max_zoom: f32,
}

impl Camera3D {
    pub fn new(aspect_ratio: f32) -> Self {
        let mut camera = Self {
            position: Vec3::new(0.0, 200.0, 300.0),
            direction: Vec3::new(0.0, -0.5, -1.0).normalize(),
            up: Vec3::Y,
            fov: 45.0,
            aspect_ratio,
            near: 0.1,
            far: 1000.0,
            orbit_target: Vec3::ZERO,
            orbit_radius: 400.0,
            orbit_angle_horizontal: 0.0,
            orbit_angle_vertical: 30.0,
            zoom_speed: 2.0,
            min_zoom: 50.0,
            max_zoom: 1000.0,
        };
        
        camera.update_orbit_position();
        camera
    }

    /// 궤도 카메라 위치 업데이트
    pub fn update_orbit_position(&mut self) {
        let h_rad = self.orbit_angle_horizontal.to_radians();
        let v_rad = self.orbit_angle_vertical.to_radians();
        
        let x = self.orbit_radius * v_rad.cos() * h_rad.sin();
        let y = self.orbit_radius * v_rad.sin();
        let z = self.orbit_radius * v_rad.cos() * h_rad.cos();
        
        self.position = self.orbit_target + Vec3::new(x, y, z);
        self.direction = (self.orbit_target - self.position).normalize();
    }

    /// 카메라 회전 (마우스 드래그로 제어)
    pub fn rotate(&mut self, delta_x: f32, delta_y: f32) {
        let sensitivity = 0.5;
        
        self.orbit_angle_horizontal += delta_x * sensitivity;
        self.orbit_angle_vertical += delta_y * sensitivity;
        
        // 수직 각도 제한 (케이크의 밑면과 윗면을 모두 볼 수 있도록)
        self.orbit_angle_vertical = self.orbit_angle_vertical.clamp(-80.0, 80.0);
        
        // 수평 각도 정규화
        if self.orbit_angle_horizontal > 360.0 {
            self.orbit_angle_horizontal -= 360.0;
        } else if self.orbit_angle_horizontal < 0.0 {
            self.orbit_angle_horizontal += 360.0;
        }
        
        self.update_orbit_position();
    }

    /// 줌 인/아웃
    pub fn zoom(&mut self, delta: f32) {
        self.orbit_radius += delta * self.zoom_speed;
        self.orbit_radius = self.orbit_radius.clamp(self.min_zoom, self.max_zoom);
        self.update_orbit_position();
    }

    /// 카메라 타겟 이동 (팬)
    pub fn pan(&mut self, delta_x: f32, delta_y: f32) {
        let right = self.direction.cross(self.up).normalize();
        let up = right.cross(self.direction).normalize();
        
        let pan_speed = self.orbit_radius * 0.001;
        self.orbit_target += right * delta_x * pan_speed;
        self.orbit_target += up * delta_y * pan_speed;
        
        self.update_orbit_position();
    }

    /// 뷰 매트릭스 계산
    pub fn view_matrix(&self) -> Mat4 {
        Mat4::look_at_rh(self.position, self.orbit_target, self.up)
    }

    /// 프로젝션 매트릭스 계산
    pub fn projection_matrix(&self) -> Mat4 {
        Mat4::perspective_rh(
            self.fov.to_radians(),
            self.aspect_ratio,
            self.near,
            self.far,
        )
    }

    /// 화면 비율 업데이트
    pub fn update_aspect_ratio(&mut self, width: f32, height: f32) {
        self.aspect_ratio = width / height;
    }

    /// 카메라를 특정 계층에 포커스
    pub fn focus_on_layer(&mut self, layer: usize) {
        let layer_height = layer as f32 * 100.0;
        self.orbit_target.y = layer_height;
        
        // 계층에 따라 적절한 거리로 조정
        match layer {
            0 => self.orbit_radius = 200.0, // 최상위 층은 가까이
            1 => self.orbit_radius = 300.0,
            2 => self.orbit_radius = 400.0,
            _ => self.orbit_radius = 500.0,
        }
        
        self.update_orbit_position();
    }

    /// 자동 회전 (데모용)
    pub fn auto_rotate(&mut self, delta_time: f32) {
        let rotation_speed = 10.0; // degrees per second
        self.orbit_angle_horizontal += rotation_speed * delta_time;
        
        if self.orbit_angle_horizontal > 360.0 {
            self.orbit_angle_horizontal -= 360.0;
        }
        
        self.update_orbit_position();
    }

    /// 전체 그래프가 보이도록 카메라 위치 조정
    pub fn fit_all_layers(&mut self, max_layer: usize, max_radius: f32) {
        let total_height = max_layer as f32 * 100.0;
        self.orbit_target.y = total_height * 0.5; // 중간 높이로 설정
        
        // 모든 층과 노드가 보이도록 거리 조정
        let needed_distance = (total_height * 0.5 + max_radius * 1.5).max(300.0);
        self.orbit_radius = needed_distance;
        
        // 약간 위에서 내려다보는 각도
        self.orbit_angle_vertical = 20.0;
        
        self.update_orbit_position();
    }

    /// 카메라가 특정 노드를 바라보도록 설정
    pub fn look_at_node(&mut self, node_position: Vec3) {
        self.orbit_target = node_position;
        self.orbit_radius = 200.0; // 노드 근처로 이동
        self.update_orbit_position();
    }
} 