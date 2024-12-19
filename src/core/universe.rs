use serde::{Serialize, Deserialize};
use crate::celestial::Graph;

#[derive(Clone, Serialize, Deserialize)]
pub struct Universe {
    pub id: String,
    pub title: String,
    pub graph: Graph,
}

impl Universe {
    pub fn new(id: String, title: String, graph: Graph) -> Self {
        Self {
            id,
            title,
            graph,
        }
    }
}

impl From<Graph> for Universe {
    fn from(graph: Graph) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            title: "New Universe".to_string(),
            graph,
        }
    }
}

impl From<Universe> for Graph {
    fn from(universe: Universe) -> Self {
        universe.graph
    }
} 