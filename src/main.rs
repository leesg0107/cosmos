use eframe::egui;
use cosmos::ui::CosmosView;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1200.0, 800.0])
            .with_min_inner_size([300.0, 200.0]),
        vsync: true,
        multisampling: 0,
        ..Default::default()
    };

    eframe::run_native(
        "Cosmos Graph",
        options,
        Box::new(|cc| {
            cc.egui_ctx.set_visuals(egui::Visuals::dark());
            Box::new(App::default())
        }),
    )
}

struct App {
    cosmos_view: CosmosView,
}

impl Default for App {
    fn default() -> Self {
        Self {
            cosmos_view: CosmosView::new(),
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.request_repaint_after(std::time::Duration::from_secs_f32(1.0 / 60.0));

        egui::CentralPanel::default().show(ctx, |ui| {
            self.cosmos_view.show(ui);
        });
    }
} 