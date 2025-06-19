use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use super::{Node, Relation, NodeType, Position2D, Position3D, RelationType};

#[derive(Clone, Serialize, Deserialize)]
pub struct Graph {
    nodes: HashMap<String, Node>,
    relations: Vec<Relation>,
}

impl Graph {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            relations: Vec::new(),
        }
    }

    pub fn create_node(&mut self, title: String, node_type: NodeType, position: Position2D) -> String {
        let node = Node::new(title, node_type, position);
        let id = node.id.clone();
        self.nodes.insert(id.clone(), node);
        id
    }

    /// 3D 위치로 노드 생성
    pub fn create_node_3d(&mut self, title: String, node_type: NodeType, layer: usize, radius: f32, angle: f32) -> String {
        let node = Node::new_3d(title, node_type, layer, radius, angle);
        let id = node.id.clone();
        self.nodes.insert(id.clone(), node);
        id
    }

    pub fn get_node(&self, id: &str) -> Option<&Node> {
        self.nodes.get(id)
    }

    pub fn get_node_mut(&mut self, id: &str) -> Option<&mut Node> {
        self.nodes.get_mut(id)
    }

    pub fn get_nodes(&self) -> impl Iterator<Item = &Node> {
        self.nodes.values()
    }

    pub fn get_relations(&self) -> impl Iterator<Item = &Relation> {
        self.relations.iter()
    }

    pub fn create_child_node(&mut self, title: String, node_type: NodeType, parent_id: &str, position: Position2D) -> Option<String> {
        let parent = self.get_node(parent_id)?;
        let new_pos = position;
        
        let node = Node::new(title, node_type, new_pos)
            .with_parent(parent_id.to_string());
        let id = node.id.clone();
        
        self.nodes.insert(id.clone(), node);
        self.add_relation(parent_id, &id, RelationType::Hierarchy);

        Some(id)
    }

    pub fn evolve_node(&mut self, base_node_id: &str, title: String, position: Option<Position2D>) -> Option<String> {
        let base_node = self.get_node(base_node_id)?;
        let pos = position.unwrap_or_else(|| Position2D::new(
            base_node.position.x + 50.0,
            base_node.position.y + 50.0
        ));
        
        let node = Node::new(title, base_node.node_type.clone(), pos);
        let id = node.id.clone();
        
        self.nodes.insert(id.clone(), node);
        self.add_relation(base_node_id, &id, RelationType::Evolution);

        Some(id)
    }

    pub fn add_relation(&mut self, source_id: &str, target_id: &str, relation_type: RelationType) {
        let relation = Relation::new(source_id.to_string(), target_id.to_string(), relation_type);
        self.relations.push(relation);
    }

    /// create_relation은 add_relation의 별칭 (호환성을 위해)
    pub fn create_relation(&mut self, source_id: &str, target_id: &str, relation_type: RelationType) {
        self.add_relation(source_id, target_id, relation_type);
    }

    /// 관계 제거
    pub fn remove_relation(&mut self, relation_id: &str) {
        self.relations.retain(|r| r.id != relation_id);
    }

    /// 노드 제거
    pub fn remove_node(&mut self, node_id: &str) {
        self.nodes.remove(node_id);
        // 해당 노드와 관련된 모든 관계도 제거
        self.relations.retain(|r| r.source_id != node_id && r.target_id != node_id);
    }

    /// 특정 노드의 자식들 가져오기
    pub fn get_children(&self, parent_id: &str) -> Vec<&Node> {
        self.relations.iter()
            .filter(|r| r.source_id == parent_id && r.is_parent_child())
            .filter_map(|r| self.get_node(&r.target_id))
            .collect()
    }

    /// 특정 노드의 부모 가져오기
    pub fn get_parent(&self, child_id: &str) -> Option<&Node> {
        self.relations.iter()
            .find(|r| r.target_id == child_id && r.is_parent_child())
            .and_then(|r| self.get_node(&r.source_id))
    }

    /// 진화 체인 가져오기
    pub fn get_evolution_chain(&self, node_id: &str) -> Vec<&Node> {
        let mut chain = Vec::new();
        let mut current_id = node_id;
        
        // 시작 노드 추가
        if let Some(node) = self.get_node(current_id) {
            chain.push(node);
        }
        
        // 진화 체인 따라가기
        while let Some(relation) = self.relations.iter()
            .find(|r| r.source_id == current_id && r.relation_type == RelationType::Evolution) {
            if let Some(node) = self.get_node(&relation.target_id) {
                chain.push(node);
                current_id = &relation.target_id;
            } else {
                break;
            }
        }
        
        chain
    }

    /// 노드 개수
    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    /// 관계 개수
    pub fn relation_count(&self) -> usize {
        self.relations.len()
    }

    /// 그래프가 비어있는지 확인
    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }
} 