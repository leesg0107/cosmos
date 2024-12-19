use uuid::Uuid;
use serde::{Serialize, Deserialize};
use super::Position2D;
use super::NodeType;

#[derive(Clone, Serialize, Deserialize)]
pub struct Node {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
    pub position: Position2D,
    pub node_type: NodeType,
    pub parent_id: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    #[serde(skip)]
    pub custom_color: Option<egui::Color32>,
    pub custom_color_rgba: Option<[u8; 4]>,
    pub custom_size: Option<f32>,
}

impl Node {
    pub fn new(title: String, node_type: NodeType, position: Position2D) -> Self {
        let now = chrono::Utc::now();
        Self {
            id: Uuid::new_v4().to_string(),
            title,
            description: None,
            position,
            node_type,
            parent_id: None,
            created_at: now,
            updated_at: now,
            custom_color: None,
            custom_color_rgba: None,
            custom_size: None,
        }
    }

    pub fn set_description(&mut self, description: String) {
        self.description = Some(description);
        self.updated_at = chrono::Utc::now();
    }

    pub fn with_parent(mut self, parent_id: String) -> Self {
        self.parent_id = Some(parent_id);
        self
    }

    pub fn set_title(&mut self, title: String) {
        self.title = title;
        self.updated_at = chrono::Utc::now();
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
} 