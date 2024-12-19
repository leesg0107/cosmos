use eframe::egui;
use crate::celestial::{NodeType, Position2D, Graph};

#[derive(Debug, Clone)]
pub enum NodeCreationInfo {
    Root(egui::Pos2),
    Child {
        parent_id: String,
        node_type: NodeType,
        position: egui::Pos2,
    },
    Evolution {
        base_id: String,
        position: egui::Pos2,
    }
}

pub struct NodeCreator {
    pub show_creator: bool,
    pub new_node_title: String,
    pub hover_pos: Option<egui::Pos2>,
    pub creation_info: Option<NodeCreationInfo>,
    pub source_node_id: Option<String>,
    pub description: String,
    graph: Graph,
}

impl NodeCreator {
    pub fn new(graph: Graph) -> Self {
        Self {
            show_creator: false,
            new_node_title: String::new(),
            hover_pos: None,
            creation_info: None,
            source_node_id: None,
            description: String::new(),
            graph,
        }
    }

    pub fn show(&mut self, ui: &mut egui::Ui) -> Option<CreationAction> {
        if !self.show_creator {
            return None;
        }

        let mut result = None;

        egui::Window::new("Node Creator")
            .fixed_size([400.0, 300.0])
            .show(ui.ctx(), |ui| {
                ui.vertical(|ui| {
                    ui.label("Title:");
                    ui.text_edit_singleline(&mut self.new_node_title);
                    ui.add_space(10.0);

                    ui.label("Description:");
                    ui.add_sized(
                        [ui.available_width(), 100.0],
                        egui::TextEdit::multiline(&mut self.description)
                    );
                    ui.add_space(20.0);

                    ui.horizontal(|ui| {
                        if let Some(source_id) = self.source_node_id.clone() {
                            if let Some(parent) = self.graph.get_node(&source_id) {
                                for valid_type in parent.node_type.get_valid_children() {
                                    if ui.button(format!("Create {}", valid_type.display_name())).clicked() {
                                        result = Some(CreationAction::CreateChild {
                                            parent_id: source_id.clone(),
                                            title: self.new_node_title.clone(),
                                            description: self.description.clone(),
                                            node_type: valid_type,
                                            position: Position2D::new(
                                                self.hover_pos.unwrap().x,
                                                self.hover_pos.unwrap().y,
                                            ),
                                        });
                                        self.reset();
                                    }
                                }
                            }
                        } else {
                            if ui.button("Create Star").clicked() {
                                result = Some(CreationAction::CreateRoot {
                                    title: self.new_node_title.clone(),
                                    description: self.description.clone(),
                                    position: Position2D::new(
                                        self.hover_pos.unwrap().x,
                                        self.hover_pos.unwrap().y,
                                    ),
                                });
                                self.reset();
                            }
                        }

                        if let Some(source_id) = self.source_node_id.clone() {
                            if ui.button("Create Evolution Layer").clicked() {
                                result = Some(CreationAction::CreateEvolution {
                                    base_id: source_id,
                                    title: self.new_node_title.clone(),
                                    description: self.description.clone(),
                                    position: Position2D::new(
                                        self.hover_pos.unwrap().x,
                                        self.hover_pos.unwrap().y + 50.0,
                                    ),
                                });
                                self.reset();
                            }
                        }
                    });
                });
            });

        result
    }

    fn reset(&mut self) {
        self.show_creator = false;
        self.new_node_title.clear();
        self.hover_pos = None;
        self.creation_info = None;
        self.source_node_id = None;
        self.description.clear();
    }
}

#[derive(Debug)]
pub enum CreationAction {
    CreateRoot {
        title: String,
        description: String,
        position: Position2D,
    },
    CreateChild {
        parent_id: String,
        title: String,
        description: String,
        node_type: NodeType,
        position: Position2D,
    },
    CreateEvolution {
        base_id: String,
        title: String,
        description: String,
        position: Position2D,
    },
} 