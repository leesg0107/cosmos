use eframe::egui;
use crate::core::universe::Universe;

pub enum ViewMode {
    Galaxy,     // 전체 우주 시스템 뷰
    Timeline,   // 시간순 진화 뷰
}

pub struct SpaceView {
    scale: f32,
    offset: egui::Vec2,
    selected_node: Option<String>,
    view_mode: ViewMode,
}

impl SpaceView {
    pub fn new() -> Self {
        Self {
            scale: 1.0,
            offset: egui::Vec2::ZERO,
            selected_node: None,
            view_mode: ViewMode::Galaxy,
        }
    }

    pub fn show(&mut self, ui: &mut egui::Ui, universe: &Universe) {
        // 뷰 모드 전환 버튼
        ui.horizontal(|ui| {
            if ui.button("Galaxy View").clicked() {
                self.view_mode = ViewMode::Galaxy;
            }
            if ui.button("Timeline View").clicked() && self.selected_node.is_some() {
                self.view_mode = ViewMode::Timeline;
            }
        });

        let (response, painter) = ui.allocate_painter(
            ui.available_size(),
            egui::Sense::drag(),
        );

        match self.view_mode {
            ViewMode::Galaxy => self.show_galaxy_view(&response, &painter, universe),
            ViewMode::Timeline => self.show_timeline_view(&response, &painter, universe),
        }
    }

    fn show_galaxy_view(&mut self, response: &egui::Response, painter: &egui::Painter, universe: &Universe) {
        // 드래그로 화면 이동
        if response.dragged() {
            self.offset += response.drag_delta();
        }

        let center = response.rect.center();
        
        // 은하수 형태로 노드 배치
        for (i, star) in universe.get_stars().enumerate() {
            let spiral_angle = (i as f32) * 0.5;
            let spiral_radius = 50.0 + (i as f32) * 20.0;
            
            let x = center.x + spiral_radius * spiral_angle.cos();
            let y = center.y + spiral_radius * spiral_angle.sin();
            let pos = egui::pos2(x, y);

            // 노드 그리기
            let node_size = 20.0 * self.scale;
            let node_rect = egui::Rect::from_center_size(
                pos + self.offset,
                egui::vec2(node_size, node_size),
            );

            // 클릭 감지
            if response.clicked() && node_rect.contains(response.hover_pos().unwrap_or_default()) {
                self.selected_node = Some(star.id.clone());
                self.view_mode = ViewMode::Timeline;
            }

            painter.circle_filled(
                pos + self.offset,
                node_size,
                egui::Color32::BLUE,
            );

            // 연결된 노드들과 궤도 그리기
            if let Some(connections) = universe.get_connections(&star.id) {
                for conn in connections {
                    if let Some(_target) = universe.get_star(&conn.target_id) {
                        // 연결선 그리기 (궤도)
                        painter.line_segment(
                            [pos + self.offset, pos + self.offset + egui::vec2(50.0, 50.0)],
                            egui::Stroke::new(1.0, egui::Color32::GRAY),
                        );
                    }
                }
            }
        }
    }

    fn show_timeline_view(&mut self, response: &egui::Response, painter: &egui::Painter, universe: &Universe) {
        if let Some(selected_id) = &self.selected_node {
            let mut current_id = selected_id.clone();
            let mut x = 100.0;
            let y = response.rect.center().y;

            // 시간순으 진화 과정 표시
            while let Some(star) = universe.get_star(&current_id) {
                let pos = egui::pos2(x, y);
                
                // 노드 그리기
                painter.circle_filled(
                    pos + self.offset,
                    30.0 * self.scale,
                    egui::Color32::BLUE,
                );

                // 노드 제목
                painter.text(
                    pos + self.offset + egui::vec2(0.0, 40.0),
                    egui::Align2::CENTER_CENTER,
                    &star.title,
                    egui::FontId::proportional(14.0),
                    egui::Color32::WHITE,
                );

                // 다음 진화 단계로
                if let Some(next_star) = universe.get_next_evolution(&current_id) {
                    // 진화 관계 화살표
                    painter.arrow(
                        pos + self.offset,
                        egui::Vec2::new(100.0, 0.0),
                        egui::Stroke::new(2.0, egui::Color32::LIGHT_BLUE),
                    );
                    current_id = next_star.id.clone();
                } else {
                    break;
                }
                x += 150.0;
            }
        }
    }
} 