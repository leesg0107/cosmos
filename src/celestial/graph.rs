use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use super::{Node, Relation, NodeType, Position2D, RelationType};

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

    pub fn create_child_node(&mut self, title: String, node_type: NodeType, parent_id: &str) -> Option<String> {
        let parent = self.get_node(parent_id)?;
        let new_pos = Position2D::new(
            parent.position.x + 100.0,
            parent.position.y + 100.0
        );
        
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
} 