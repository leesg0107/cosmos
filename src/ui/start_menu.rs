use eframe::egui;

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

    pub fn show(&mut self, ui: &mut egui::Ui) -> MenuAction {
        let mut action = MenuAction::None;

        // 중앙 정렬을 위한 레이아웃
        ui.vertical_centered(|ui| {
            ui.add_space(200.0); // 상단 여백

            // 메뉴 버튼들
            if ui.button(egui::RichText::new("Big Bang")
                .size(40.0)
                .color(egui::Color32::from_rgb(255, 255, 255)))
                .clicked() 
            {
                action = MenuAction::BigBang;
            }

            ui.add_space(20.0);

            if ui.button(egui::RichText::new("Time Log")
                .size(40.0)
                .color(egui::Color32::from_rgb(200, 200, 200)))
                .clicked() 
            {
                action = MenuAction::TimeLog;
            }

            ui.add_space(20.0);

            if ui.button(egui::RichText::new("Black Hole")
                .size(40.0)
                .color(egui::Color32::from_rgb(150, 150, 150)))
                .clicked() 
            {
                action = MenuAction::BlackHole;
            }
        });

        action
    }
} 