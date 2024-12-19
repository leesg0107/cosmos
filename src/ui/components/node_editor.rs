use eframe::egui;
use crate::celestial::{Node, NodeType};

pub struct NodeEditor {
    pub show_editor: bool,
    pub editing_node: Option<String>,
    pub title: String,
    pub description: String,
    show_color_picker: bool,
    size_edit_mode: bool,
    temp_size: f32,
}

impl NodeEditor {
    pub fn new() -> Self {
        Self {
            show_editor: false,
            editing_node: None,
            title: String::new(),
            description: String::new(),
            show_color_picker: false,
            size_edit_mode: false,
            temp_size: 0.0,
        }
    }

    pub fn show(&mut self, ui: &mut egui::Ui, node: &mut Node) -> Option<EditorAction> {
        let mut action = None;
        let mut current_color = node.get_color()
            .unwrap_or_else(|| self.get_default_color(&node.node_type));
        let default_size = self.get_default_size(&node.node_type);

        egui::Window::new("Node Editor")
            .open(&mut self.show_editor)
            .show(ui.ctx(), |ui| {
                ui.horizontal(|ui| {
                    // 제목 편집
                    let mut title = node.title.clone();
                    if ui.text_edit_singleline(&mut title).changed() {
                        action = Some(EditorAction::UpdateTitle(title));
                    }

                    // 색상 선택 버튼
                    let button_response = ui.button("🎨");
                    let button_rect = button_response.rect;
                    if button_response.on_hover_text("Change color").clicked() {
                        self.show_color_picker = !self.show_color_picker;
                    }
                    ui.painter().rect_filled(
                        button_rect,
                        0.0,
                        current_color,
                    );

                    // 크기 조절 버튼
                    if ui.button("📏").clicked() {
                        self.size_edit_mode = !self.size_edit_mode;
                        self.temp_size = node.custom_size.unwrap_or(default_size);
                    }
                });

                // 색상 선택기
                if self.show_color_picker {
                    if ui.color_edit_button_srgba(&mut current_color).changed() {
                        action = Some(EditorAction::UpdateColor(current_color));
                    }
                }

                // 크기 조절 UI
                if self.size_edit_mode {
                    ui.horizontal(|ui| {
                        ui.label("Size:");
                        if ui.add(egui::Slider::new(&mut self.temp_size, 5.0..=50.0)).changed() {
                            action = Some(EditorAction::UpdateSize(self.temp_size));
                        }
                    });
                }

                // 설명 편집
                ui.label("Description:");
                let mut description = node.description.clone().unwrap_or_default();
                if ui.text_edit_multiline(&mut description).changed() {
                    action = Some(EditorAction::UpdateDescription(description));
                }

                if ui.button("Create Evolution Layer").clicked() {
                    action = Some(EditorAction::CreateEvolutionLayer);
                }
            });

        action
    }

    fn get_default_color(&self, node_type: &NodeType) -> egui::Color32 {
        match node_type {
            NodeType::Star => egui::Color32::from_rgb(255, 223, 186),
            NodeType::Planet => egui::Color32::from_rgb(186, 223, 255),
            NodeType::Satellite => egui::Color32::from_rgb(200, 200, 200),
            NodeType::Asteroid => egui::Color32::from_rgb(169, 169, 169),
        }
    }

    fn get_default_size(&self, node_type: &NodeType) -> f32 {
        match node_type {
            NodeType::Star => 20.0,
            NodeType::Planet => 15.0,
            NodeType::Satellite => 8.0,
            NodeType::Asteroid => 5.0,
        }
    }
}

#[derive(Debug)]
pub enum EditorAction {
    UpdateTitle(String),
    UpdateDescription(String),
    CreateEvolutionLayer,
    UpdateColor(egui::Color32),
    UpdateSize(f32),
} 