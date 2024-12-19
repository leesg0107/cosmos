use eframe::egui;
use super::start_menu::{StartMenu, MenuAction};
use super::cosmos_view::CosmosView;
use super::cosmos_view::NavigationAction;

pub enum AppState {
    StartMenu,
    Cosmos,
    TimeLog,
}

pub struct CosmosApp {
    state: AppState,
    start_menu: StartMenu,
    cosmos_view: CosmosView,
}

impl CosmosApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            state: AppState::StartMenu,
            start_menu: StartMenu::new(),
            cosmos_view: CosmosView::new(),
        }
    }
}

impl eframe::App for CosmosApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let painter = ui.painter();
            let rect = ui.max_rect();
            painter.rect_filled(
                rect,
                0.0,
                egui::Color32::BLACK,
            );

            match self.state {
                AppState::StartMenu => {
                    match self.start_menu.show(ui) {
                        MenuAction::BigBang => self.state = AppState::Cosmos,
                        MenuAction::TimeLog => self.state = AppState::TimeLog,
                        MenuAction::BlackHole => std::process::exit(0),
                        MenuAction::None => {}
                    }
                }
                AppState::Cosmos => {
                    if let Some(action) = self.cosmos_view.show(ui) {
                        match action {
                            NavigationAction::Back => self.state = AppState::StartMenu,
                        }
                    }
                }
                AppState::TimeLog => {
                    // TimeLog 화면 구현
                }
            }
        });
    }
} 