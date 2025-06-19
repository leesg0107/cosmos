use uuid::Uuid;
use serde::{Serialize, Deserialize};
use super::{Position2D, Position3D, LayerPosition};
use super::NodeType;

#[derive(Clone, Serialize, Deserialize)]
pub struct Node {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
    
    // 3D 지원을 위한 위치 시스템
    pub position: Position2D,  // 하위 호환성
    pub position_3d: Position3D,
    pub layer_position: Option<LayerPosition>,
    
    pub node_type: NodeType,
    pub parent_id: Option<String>,
    pub children_ids: Vec<String>,  // 자식 노드들의 ID
    
    // 계층 구조 정보
    pub layer: usize,           // 케이크의 몇 번째 층인지
    pub layer_radius: f32,      // 해당 층에서의 반지름
    pub layer_angle: f32,       // 해당 층에서의 각도
    
    // 시간 정보
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    
    // 스타일 정보
    #[serde(skip)]
    pub custom_color: Option<egui::Color32>,
    pub custom_color_rgba: Option<[u8; 4]>,
    pub custom_size: Option<f32>,
    
    // 3D 전용 속성
    pub is_visible: bool,
    pub opacity: f32,
    pub rotation: [f32; 3],     // x, y, z 회전
    pub scale: f32,
    
    // 케이크 관련 필드
    #[serde(skip)]
    pub color: [f32; 4],
    #[serde(skip)]
    pub size: f32,
    #[serde(skip)]
    pub selected: bool,
}

impl Node {
    pub fn new(title: String, node_type: NodeType, position: Position2D) -> Self {
        let layer = node_type.cake_layer();
        let now = chrono::Utc::now();
        
        Self {
            id: Uuid::new_v4().to_string(),
            title,
            description: None,
            position,
            position_3d: Position3D::new(position.x, position.y, layer as f32 * 100.0),
            layer_position: None,
            node_type,
            parent_id: None,
            children_ids: Vec::new(),
            layer,
            layer_radius: 100.0 + layer as f32 * 50.0, // 층마다 반지름 증가
            layer_angle: 0.0,
            created_at: now,
            updated_at: now,
            custom_color: None,
            custom_color_rgba: None,
            custom_size: None,
            is_visible: true,
            opacity: 1.0,
            rotation: [0.0, 0.0, 0.0],
            scale: 1.0,
            color: Self::get_default_color(&node_type),
            size: Self::get_default_size(&node_type),
            selected: false,
        }
    }

    /// 3D 케이크 구조로 노드 생성
    pub fn new_3d(title: String, node_type: NodeType, layer: usize, radius: f32, angle: f32) -> Self {
        let now = chrono::Utc::now();
        
        // 3D 좌표 계산
        let x = radius * angle.cos();
        let z = radius * angle.sin();
        let y = layer as f32 * 100.0; // 층 높이
        
        Self {
            id: Uuid::new_v4().to_string(),
            title,
            description: None,
            position: Position2D::new(x, z), // 2D 호환용
            position_3d: Position3D::new(x, y, z),
            layer_position: None,
            node_type,
            parent_id: None,
            children_ids: Vec::new(),
            layer,
            layer_radius: radius,
            layer_angle: angle,
            created_at: now,
            updated_at: now,
            custom_color: None,
            custom_color_rgba: None,
            custom_size: None,
            is_visible: true,
            opacity: 1.0,
            rotation: [0.0, 0.0, 0.0],
            scale: 1.0,
            color: Self::get_default_color(&node_type),
            size: Self::get_default_size(&node_type),
            selected: false,
        }
    }

    pub fn with_parent(mut self, parent_id: String) -> Self {
        self.parent_id = Some(parent_id);
        self
    }

    pub fn with_description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }

    pub fn with_layer(mut self, layer: usize) -> Self {
        self.layer = layer;
        // 3D 위치도 업데이트
        self.position_3d.y = layer as f32 * 100.0;
        self.updated_at = chrono::Utc::now();
        self
    }

    pub fn set_description(&mut self, description: String) {
        self.description = Some(description);
        self.updated_at = chrono::Utc::now();
    }

    pub fn add_child(&mut self, child_id: String) {
        if !self.children_ids.contains(&child_id) {
            self.children_ids.push(child_id);
            self.updated_at = chrono::Utc::now();
        }
    }

    pub fn remove_child(&mut self, child_id: &str) {
        self.children_ids.retain(|id| id != child_id);
        self.updated_at = chrono::Utc::now();
    }

    /// 케이크 레이어에서의 위치 업데이트
    pub fn update_cake_position(&mut self, radius: f32, angle: f32) {
        self.layer_radius = radius;
        self.layer_angle = angle;
        
        let x = radius * angle.cos();
        let z = radius * angle.sin();
        
        self.position.x = x;
        self.position.y = z;
        self.position_3d.x = x;
        self.position_3d.z = z;
        
        self.updated_at = chrono::Utc::now();
    }

    /// 노드 타입에 따른 기본 색상
    fn get_default_color(node_type: &NodeType) -> [f32; 4] {
        match node_type {
            NodeType::Root => [1.0, 0.8, 0.0, 1.0],      // 골드
            NodeType::Concept => [0.0, 0.6, 1.0, 1.0],   // 블루
            NodeType::Task => [1.0, 0.4, 0.6, 1.0],      // 핑크
            NodeType::Note => [0.4, 1.0, 0.4, 1.0],      // 그린
            NodeType::Evolution => [0.8, 0.4, 1.0, 1.0], // 퍼플
            NodeType::Star => [1.0, 0.9, 0.0, 1.0],      // 노란색
            NodeType::Planet => [0.0, 0.7, 1.0, 1.0],    // 파란색
            NodeType::Satellite => [0.7, 0.7, 0.7, 1.0], // 회색
            NodeType::Asteroid => [0.5, 0.3, 0.1, 1.0],  // 갈색
            NodeType::Category => [0.8, 0.8, 0.8, 1.0],  // 라이트 그레이
            NodeType::Base => [0.6, 0.6, 0.6, 1.0],      // 그레이
        }
    }

    /// 노드 타입에 따른 기본 크기
    fn get_default_size(node_type: &NodeType) -> f32 {
        match node_type {
            NodeType::Root => 50.0,
            NodeType::Concept => 40.0,
            NodeType::Task => 30.0,
            NodeType::Note => 25.0,
            NodeType::Evolution => 35.0,
            NodeType::Star => 45.0,
            NodeType::Planet => 35.0,
            NodeType::Satellite => 25.0,
            NodeType::Asteroid => 20.0,
            NodeType::Category => 40.0,
            NodeType::Base => 30.0,
        }
    }

    /// 케이크 레이어에서의 크기 (거리에 따른 스케일링)
    pub fn get_layer_size(&self) -> f32 {
        let base_size = self.size;
        let layer_scale = 1.0 + (self.layer as f32 * 0.1); // 위층일수록 약간 커짐
        base_size * layer_scale
    }

    /// 노드가 선택되었는지 확인
    pub fn is_selected(&self) -> bool {
        self.selected
    }

    /// 노드 선택 상태 설정
    pub fn set_selected(&mut self, selected: bool) {
        self.selected = selected;
    }

    /// 노드의 이모지 아이콘 반환
    pub fn get_emoji(&self) -> &'static str {
        self.node_type.emoji()
    }

    /// 노드의 전체 표시 텍스트 (이모지 + 제목)
    pub fn display_text(&self) -> String {
        format!("{} {}", self.get_emoji(), self.title)
    }

    /// 노드가 루트 노드인지 확인
    pub fn is_root(&self) -> bool {
        matches!(self.node_type, NodeType::Root) || self.parent_id.is_none()
    }

    /// 노드에 자식이 있는지 확인
    pub fn has_children(&self) -> bool {
        !self.children_ids.is_empty()
    }

    /// 자식 노드 개수
    pub fn child_count(&self) -> usize {
        self.children_ids.len()
    }

    pub fn set_color(&mut self, color: egui::Color32) {
        self.custom_color = Some(color);
        self.custom_color_rgba = Some([color.r(), color.g(), color.b(), color.a()]);
        self.updated_at = chrono::Utc::now();
    }

    pub fn set_size(&mut self, size: f32) {
        self.custom_size = Some(size);
        self.updated_at = chrono::Utc::now();
    }

    pub fn get_color(&self) -> Option<egui::Color32> {
        self.custom_color.or_else(|| {
            self.custom_color_rgba.map(|rgba| 
                egui::Color32::from_rgba_unmultiplied(rgba[0], rgba[1], rgba[2], rgba[3])
            )
        })
    }

    /// 3D 위치 업데이트
    pub fn set_position_3d(&mut self, position: Position3D) {
        self.position_3d = position;
        self.position = Position2D::new(position.x, position.z); // Top-down view
        self.updated_at = chrono::Utc::now();
    }

    /// 계층 위치 업데이트
    pub fn set_layer_position(&mut self, layer: usize, radius: f32, angle: f32) {
        self.layer = layer;
        self.layer_radius = radius;
        self.layer_angle = angle;
        
        let mut layer_position = LayerPosition::new(layer, radius, angle);
        layer_position.update_position();
        
        self.layer_position = Some(layer_position.clone());
        self.set_position_3d(layer_position.position);
    }

    /// 노드가 특정 계층에 속하는지 확인
    pub fn is_in_layer(&self, layer: usize) -> bool {
        self.layer == layer
    }

    /// 계층에 따른 색상 가져오기
    pub fn get_layer_color(&self) -> [f32; 4] {
        if let Some(layer_pos) = &self.layer_position {
            layer_pos.layer_color()
        } else {
            [1.0, 1.0, 1.0, 1.0] // 기본 흰색
        }
    }

    /// 노드 회전 설정
    pub fn set_rotation(&mut self, x: f32, y: f32, z: f32) {
        self.rotation = [x, y, z];
        self.updated_at = chrono::Utc::now();
    }

    /// 노드 스케일 설정
    pub fn set_scale(&mut self, scale: f32) {
        self.scale = scale;
        self.updated_at = chrono::Utc::now();
    }

    /// 노드 투명도 설정
    pub fn set_opacity(&mut self, opacity: f32) {
        self.opacity = opacity.clamp(0.0, 1.0);
        self.updated_at = chrono::Utc::now();
    }

    /// 노드 보이기/숨기기
    pub fn set_visible(&mut self, visible: bool) {
        self.is_visible = visible;
        self.updated_at = chrono::Utc::now();
    }
} 