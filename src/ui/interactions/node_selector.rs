use crate::celestial::{Graph, Node};

pub struct NodeSelector {
    pub selected_node: Option<String>,
}

impl NodeSelector {
    pub fn new() -> Self {
        Self {
            selected_node: None,
        }
    }

    pub fn select_node(&mut self, node_id: String) {
        self.selected_node = Some(node_id);
    }

    pub fn deselect(&mut self) {
        self.selected_node = None;
    }

    pub fn is_selected(&self, node_id: &str) -> bool {
        self.selected_node.as_ref().map_or(false, |id| id == node_id)
    }

    pub fn get_selected_node<'a>(&self, graph: &'a Graph) -> Option<&'a Node> {
        self.selected_node.as_ref().and_then(|id| graph.get_node(id))
    }
}

#[derive(Debug)]
pub enum SelectionAction {
    Select(String),
    StartEditing(String),
} 