use serde::{Serialize, Deserialize};

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum NodeType {
    // 3D 케이크 구조용 타입들
    Root,       // 최하위층 (골드)
    Concept,    // 2단 (블루)
    Task,       // 3단 (핑크)
    Note,       // 4단 (그린)
    Evolution,  // 진화/발전된 노드
    
    // 기존 우주 테마 타입들 (하위 호환성)
    Star,
    Planet,
    Satellite,
    Asteroid,
    
    // 추가 타입들
    Category,   // 카테고리
    Base,       // 기본
}

impl NodeType {
    pub fn display_name(&self) -> &'static str {
        match self {
            NodeType::Root => "Root",
            NodeType::Concept => "Concept", 
            NodeType::Task => "Task",
            NodeType::Note => "Note",
            NodeType::Evolution => "Evolution",
            NodeType::Star => "Star",
            NodeType::Planet => "Planet",
            NodeType::Satellite => "Satellite",
            NodeType::Asteroid => "Asteroid",
            NodeType::Category => "Category",
            NodeType::Base => "Base",
        }
    }

    pub fn get_valid_children(&self) -> Vec<NodeType> {
        match self {
            // 3D 케이크 구조 계층
            NodeType::Root => vec![NodeType::Concept, NodeType::Category],
            NodeType::Concept => vec![NodeType::Task, NodeType::Note],
            NodeType::Task => vec![NodeType::Note],
            NodeType::Note => vec![],
            NodeType::Evolution => vec![NodeType::Evolution], // 진화 체인
            
            // 기존 우주 테마 계층
            NodeType::Star => vec![NodeType::Planet],
            NodeType::Planet => vec![NodeType::Satellite],
            NodeType::Satellite => vec![NodeType::Asteroid],
            NodeType::Asteroid => vec![],
            
            // 추가 타입들
            NodeType::Category => vec![NodeType::Base, NodeType::Concept],
            NodeType::Base => vec![NodeType::Task, NodeType::Note],
        }
    }

    /// 다음 계층의 기본 타입 반환
    pub fn next_level(&self) -> Option<NodeType> {
        match self {
            NodeType::Root => Some(NodeType::Concept),
            NodeType::Concept => Some(NodeType::Task),
            NodeType::Task => Some(NodeType::Note),
            NodeType::Note => None,
            
            NodeType::Star => Some(NodeType::Planet),
            NodeType::Planet => Some(NodeType::Satellite),
            NodeType::Satellite => Some(NodeType::Asteroid),
            NodeType::Asteroid => None,
            
            NodeType::Category => Some(NodeType::Base),
            NodeType::Base => Some(NodeType::Task),
            NodeType::Evolution => Some(NodeType::Evolution),
        }
    }

    /// 케이크 계층에서의 레벨 (0부터 시작)
    pub fn cake_layer(&self) -> usize {
        match self {
            NodeType::Root => 0,
            NodeType::Concept | NodeType::Category => 1,
            NodeType::Task | NodeType::Base => 2,
            NodeType::Note => 3,
            NodeType::Evolution => 4,
            
            // 우주 테마는 기본 레벨로 매핑
            NodeType::Star => 0,
            NodeType::Planet => 1,
            NodeType::Satellite => 2,
            NodeType::Asteroid => 3,
        }
    }

    /// 타입에 따른 이모지 반환
    pub fn emoji(&self) -> &'static str {
        match self {
            NodeType::Root => "🌌",
            NodeType::Concept => "💭",
            NodeType::Task => "📋",
            NodeType::Note => "📝",
            NodeType::Evolution => "🔄",
            NodeType::Star => "⭐",
            NodeType::Planet => "🪐",
            NodeType::Satellite => "🛰️",
            NodeType::Asteroid => "☄️",
            NodeType::Category => "📁",
            NodeType::Base => "🔧",
        }
    }
} 