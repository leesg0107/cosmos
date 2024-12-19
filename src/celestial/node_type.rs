use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum NodeType {
    Star,
    Planet,
    Satellite,
    Asteroid,
}

impl NodeType {
    pub fn display_name(&self) -> &'static str {
        match self {
            NodeType::Star => "Star",
            NodeType::Planet => "Planet",
            NodeType::Satellite => "Satellite",
            NodeType::Asteroid => "Asteroid",
        }
    }

    pub fn get_valid_children(&self) -> Vec<NodeType> {
        match self {
            NodeType::Star => vec![NodeType::Planet],
            NodeType::Planet => vec![NodeType::Satellite],
            NodeType::Satellite => vec![NodeType::Asteroid],
            NodeType::Asteroid => vec![],
        }
    }
} 