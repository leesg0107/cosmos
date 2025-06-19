use serde::{Serialize, Deserialize};
use crate::celestial::NodeType;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum RelationType {
    // 3D 케이크 구조용 관계들
    Parent,     // 부모-자식 관계 (계층간 연결)
    Child,      // 자식 관계 (Parent의 역방향)
    Sibling,    // 같은 계층 내 연결
    
    // 기존 우주 테마 관계들 (하위 호환성)
    Orbit,      // 궤도 관계
    Evolution,  // 시간 발전 관계
    Reference,  // 참조 관계
    Hierarchy,  // 계층 관계
    
    // 추가 관계 타입들
    Dependency, // 의존성 관계
    Association, // 연관 관계
}

impl RelationType {
    /// 관계 타입의 표시 이름
    pub fn display_name(&self) -> &'static str {
        match self {
            RelationType::Parent => "Parent",
            RelationType::Child => "Child",
            RelationType::Sibling => "Sibling",
            RelationType::Orbit => "Orbit",
            RelationType::Evolution => "Evolution",
            RelationType::Reference => "Reference",
            RelationType::Hierarchy => "Hierarchy",
            RelationType::Dependency => "Dependency",
            RelationType::Association => "Association",
        }
    }

    /// 관계의 색상 (시각화용)
    pub fn color(&self) -> [f32; 4] {
        match self {
            RelationType::Parent => [1.0, 0.8, 0.0, 1.0],    // 골드
            RelationType::Child => [0.8, 1.0, 0.0, 1.0],     // 라이트 그린
            RelationType::Sibling => [0.0, 0.8, 1.0, 1.0],   // 블루
            RelationType::Orbit => [0.6, 0.8, 1.0, 1.0],     // 라이트 블루
            RelationType::Evolution => [0.6, 1.0, 0.6, 1.0], // 그린
            RelationType::Reference => [1.0, 0.6, 0.6, 1.0], // 핑크
            RelationType::Hierarchy => [0.8, 0.8, 0.8, 1.0], // 그레이
            RelationType::Dependency => [1.0, 0.6, 0.0, 1.0], // 오렌지
            RelationType::Association => [0.8, 0.6, 1.0, 1.0], // 퍼플
        }
    }

    /// 관계의 두께 (시각화용)
    pub fn thickness(&self) -> f32 {
        match self {
            RelationType::Parent | RelationType::Child => 3.0, // 계층 관계는 굵게
            RelationType::Hierarchy => 2.5,
            RelationType::Evolution => 2.0,
            RelationType::Dependency => 2.0,
            _ => 1.5,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Relation {
    pub id: String,
    pub source_id: String,
    pub target_id: String,
    pub relation_type: RelationType,
    pub label: Option<String>,
    pub weight: f32,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl Relation {
    pub fn new(source_id: String, target_id: String, relation_type: RelationType) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            source_id,
            target_id,
            relation_type,
            label: None,
            weight: 1.0,
            created_at: chrono::Utc::now(),
        }
    }

    pub fn with_label(mut self, label: String) -> Self {
        self.label = Some(label);
        self
    }

    pub fn with_weight(mut self, weight: f32) -> Self {
        self.weight = weight;
        self
    }

    /// 계층 관계가 유효한지 확인
    pub fn is_valid_hierarchy(source_type: &NodeType, target_type: &NodeType) -> bool {
        matches!(
            (source_type, target_type),
            // 3D 케이크 구조 계층
            (NodeType::Root, NodeType::Concept) |
            (NodeType::Root, NodeType::Category) |
            (NodeType::Concept, NodeType::Task) |
            (NodeType::Concept, NodeType::Note) |
            (NodeType::Task, NodeType::Note) |
            (NodeType::Category, NodeType::Base) |
            (NodeType::Base, NodeType::Task) |
            (NodeType::Base, NodeType::Note) |
            
            // 기존 우주 테마 계층 (하위 호환성)
            (NodeType::Star, NodeType::Planet) | 
            (NodeType::Planet, NodeType::Satellite) |
            (NodeType::Satellite, NodeType::Asteroid)
        )
    }

    /// 부모-자식 관계인지 확인
    pub fn is_parent_child(&self) -> bool {
        matches!(self.relation_type, RelationType::Parent | RelationType::Child)
    }

    /// 역방향 관계 타입 반환
    pub fn reverse_type(&self) -> RelationType {
        match self.relation_type {
            RelationType::Parent => RelationType::Child,
            RelationType::Child => RelationType::Parent,
            _ => self.relation_type.clone(),
        }
    }
} 