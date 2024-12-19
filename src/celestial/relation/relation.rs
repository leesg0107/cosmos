use serde::{Serialize, Deserialize};
use crate::celestial::NodeType;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum RelationType {
    Orbit,      // 궤도 관계
    Evolution,  // 시간 발전 관계
    Reference,  // 참조 관계
    Hierarchy,  // 계층 관계
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Relation {
    pub id: String,
    pub source_id: String,
    pub target_id: String,
    pub relation_type: RelationType,
    pub label: Option<String>,
    pub weight: f32,
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
        }
    }

    pub fn is_valid_hierarchy(source_type: &NodeType, target_type: &NodeType) -> bool {
        matches!(
            (source_type, target_type),
            (NodeType::Star, NodeType::Planet) | 
            (NodeType::Planet, NodeType::Satellite)
        )
    }
} 