use eframe::egui;

#[derive(Clone)]
pub enum MenuAction {
    BigBang,
    TimeLog,
    BlackHole,
    None,
}

pub struct StartMenu {
    selected_action: MenuAction,
}

impl StartMenu {
    pub fn new() -> Self {
        Self {
            selected_action: MenuAction::None,
        }
    }

    pub fn show(&mut self, _ui: &mut egui::Ui) -> MenuAction {
        self.selected_action.clone()
    }
} 