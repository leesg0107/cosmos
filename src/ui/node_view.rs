use eframe::egui;
use crate::celestial::star::Star;

pub struct NodeView {
    content: String,
}

impl NodeView {
    pub fn new() -> Self {
        Self {
            content: String::new(),
        }
    }

    pub fn show(&mut self, ui: &mut egui::Ui, star: &mut Star) {
        ui.heading(&star.title);
        
        let mut content = star.content.clone();
        ui.text_edit_multiline(&mut content);
        if content != star.content {
            star.update_content(content);
        }
    }
} 