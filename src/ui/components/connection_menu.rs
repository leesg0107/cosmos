use eframe::egui;
use crate::celestial::RelationType;

pub struct ConnectionMenu {
    pub show_menu: bool,
    pub source_id: Option<String>,
    pub target_id: Option<String>,
}

impl ConnectionMenu {
    pub fn new() -> Self {
        Self {
            show_menu: false,
            source_id: None,
            target_id: None,
        }
    }

    pub fn show(&mut self, ui: &mut egui::Ui) -> Option<ConnectionAction> {
        if !self.show_menu {
            return None;
        }

        let source_id = self.source_id.clone()?;
        let target_id = self.target_id.clone()?;
        let mut result = None;

        egui::Window::new("Create Connection")
            .show(ui.ctx(), |ui| {
                ui.vertical(|ui| {
                    if ui.button("Orbit Connection").clicked() {
                        result = Some(ConnectionAction::CreateConnection {
                            source_id: source_id.clone(),
                            target_id: target_id.clone(),
                            relation_type: RelationType::Orbit,
                        });
                        self.reset();
                    }
                    if ui.button("Evolution Connection").clicked() {
                        result = Some(ConnectionAction::CreateConnection {
                            source_id: source_id.clone(),
                            target_id: target_id.clone(),
                            relation_type: RelationType::Evolution,
                        });
                        self.reset();
                    }
                    if ui.button("Reference Connection").clicked() {
                        result = Some(ConnectionAction::CreateConnection {
                            source_id: source_id.clone(),
                            target_id: target_id.clone(),
                            relation_type: RelationType::Reference,
                        });
                        self.reset();
                    }
                    ui.separator();
                    if ui.button("Cancel").clicked() {
                        self.reset();
                    }
                });
            });

        result
    }

    pub fn show_for_nodes(&mut self, source_id: String, target_id: String) {
        self.source_id = Some(source_id);
        self.target_id = Some(target_id);
        self.show_menu = true;
    }

    fn reset(&mut self) {
        self.show_menu = false;
        self.source_id = None;
        self.target_id = None;
    }
}

#[derive(Debug)]
pub enum ConnectionAction {
    CreateConnection {
        source_id: String,
        target_id: String,
        relation_type: RelationType,
    },
} 