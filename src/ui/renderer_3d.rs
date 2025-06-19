use eframe::egui;
use glam::{Vec3, Vec2, Mat4};
use crate::celestial::{Node, Position3D};
use super::Camera3D;

pub struct Renderer3D {
    camera: Camera3D,
    is_dragging: bool,
    last_mouse_pos: Option<Vec2>,
    auto_rotate: bool,
    show_grid: bool,
    show_layer_platforms: bool,
}

impl Renderer3D {
    pub fn new() -> Self {
        Self {
            camera: Camera3D::new(16.0 / 9.0), // 기본 화면 비율
            is_dragging: false,
            last_mouse_pos: None,
            auto_rotate: false,
            show_grid: true,
            show_layer_platforms: true,
        }
    }

    /// 3D 장면 렌더링
    pub fn render_scene(
        &mut self,
        ui: &mut egui::Ui,
        nodes: &[Node],
        selected_node: Option<&String>,
    ) -> egui::Response {
        let (response, painter) = ui.allocate_painter(
            ui.available_size(),
            egui::Sense::click_and_drag(),
        );

        // 화면 크기에 따라 카메라 비율 업데이트
        let rect = response.rect;
        self.camera.update_aspect_ratio(rect.width(), rect.height());

        // 배경 그리기 (우주 배경)
        self.draw_background(&painter, &rect);

        // 계층 플랫폼 그리기 (케이크 층)
        if self.show_layer_platforms {
            self.draw_layer_platforms(&painter, &rect, nodes);
        }

        // 격자 그리기
        if self.show_grid {
            self.draw_3d_grid(&painter, &rect);
        }

        // 노드들 렌더링
        self.draw_nodes_3d(&painter, &rect, nodes, selected_node);

        // 연결선 렌더링
        self.draw_connections_3d(&painter, &rect, nodes);

        // 마우스 상호작용 처리
        self.handle_camera_controls(&response);

        // 자동 회전
        if self.auto_rotate {
            let delta_time = ui.ctx().input(|i| i.stable_dt);
            self.camera.auto_rotate(delta_time);
            ui.ctx().request_repaint();
        }

        response
    }

    /// 우주 배경 그리기
    fn draw_background(&self, painter: &egui::Painter, rect: &egui::Rect) {
        // 그라데이션 배경 (우주 느낌)
        let gradient = egui::Color32::from_rgb(5, 5, 15)
            .linear_multiply(0.8);
        
        painter.rect_filled(*rect, 0.0, gradient);

        // 별들 그리기
        for i in 0..100 {
            let x = (i * 17 + 23) % rect.width() as i32;
            let y = (i * 31 + 47) % rect.height() as i32;
            let alpha = ((i * 7 + 13) % 100) as f32 / 100.0;
            
            let star_color = egui::Color32::from_rgba_premultiplied(
                255, 255, 255, (alpha * 255.0) as u8
            );
            
            painter.circle_filled(
                egui::pos2(x as f32, y as f32),
                1.0 + alpha,
                star_color,
            );
        }
    }

    /// 계층 플랫폼 그리기 (케이크 층)
    fn draw_layer_platforms(&self, painter: &egui::Painter, rect: &egui::Rect, nodes: &[Node]) {
        let view_matrix = self.camera.view_matrix();
        let projection_matrix = self.camera.projection_matrix();
        let viewport = Vec2::new(rect.width(), rect.height());

        // 각 계층에 대해 플랫폼 그리기
        let max_layer = nodes.iter().map(|n| n.layer).max().unwrap_or(0);
        
        for layer in 0..=max_layer {
            let layer_height = layer as f32 * 100.0;
            let layer_nodes: Vec<_> = nodes.iter().filter(|n| n.layer == layer).collect();
            
            if layer_nodes.is_empty() {
                continue;
            }

            // 해당 계층의 최대 반지름 계산
            let max_radius = layer_nodes.iter()
                .map(|n| n.layer_radius)
                .fold(0.0, f32::max)
                .max(100.0);

            // 플랫폼 원 그리기
            self.draw_platform_circle(
                painter,
                rect,
                Vec3::new(0.0, layer_height, 0.0),
                max_radius + 50.0,
                layer,
                &view_matrix,
                &projection_matrix,
                &viewport,
            );
        }
    }

    /// 개별 플랫폼 원 그리기
    fn draw_platform_circle(
        &self,
        painter: &egui::Painter,
        rect: &egui::Rect,
        center: Vec3,
        radius: f32,
        layer: usize,
        view_matrix: &Mat4,
        projection_matrix: &Mat4,
        viewport: &Vec2,
    ) {
        let segments = 64;
        let mut points = Vec::new();

        // 원 둘레의 점들 계산
        for i in 0..segments {
            let angle = (i as f32 / segments as f32) * 2.0 * std::f32::consts::PI;
            let x = center.x + radius * angle.cos();
            let z = center.z + radius * angle.sin();
            let world_pos = Vec3::new(x, center.y, z);

            // 3D to 2D 투영
            let screen_pos = Position3D::from_vec3(world_pos)
                .project_to_screen(view_matrix, projection_matrix, *viewport);
            
            points.push(egui::pos2(screen_pos.x, screen_pos.y));
        }

        // 원 그리기
        let layer_color = match layer {
            0 => egui::Color32::from_rgba_premultiplied(255, 230, 180, 60), // 골드
            1 => egui::Color32::from_rgba_premultiplied(180, 180, 255, 60), // 블루
            2 => egui::Color32::from_rgba_premultiplied(255, 180, 180, 60), // 핑크
            3 => egui::Color32::from_rgba_premultiplied(180, 255, 180, 60), // 그린
            _ => egui::Color32::from_rgba_premultiplied(200, 200, 200, 60), // 그레이
        };

        // 플랫폼 경계선
        for i in 0..points.len() {
            let next_i = (i + 1) % points.len();
            painter.line_segment(
                [points[i], points[next_i]],
                egui::Stroke::new(2.0, layer_color),
            );
        }

        // 중심에서 방사형 선 그리기 (8개 방향)
        let center_2d = Position3D::from_vec3(center)
            .project_to_screen(view_matrix, projection_matrix, *viewport);
        let center_pos = egui::pos2(center_2d.x, center_2d.y);
        
        for i in 0..8 {
            let angle = (i as f32 / 8.0) * 2.0 * std::f32::consts::PI;
            let edge_x = center.x + radius * angle.cos();
            let edge_z = center.z + radius * angle.sin();
            let edge_world = Vec3::new(edge_x, center.y, edge_z);
            
            let edge_2d = Position3D::from_vec3(edge_world)
                .project_to_screen(view_matrix, projection_matrix, *viewport);
            let edge_pos = egui::pos2(edge_2d.x, edge_2d.y);
            
            painter.line_segment(
                [center_pos, edge_pos],
                egui::Stroke::new(1.0, layer_color.linear_multiply(0.5)),
            );
        }
    }

    /// 3D 격자 그리기
    fn draw_3d_grid(&self, painter: &egui::Painter, rect: &egui::Rect) {
        let view_matrix = self.camera.view_matrix();
        let projection_matrix = self.camera.projection_matrix();
        let viewport = Vec2::new(rect.width(), rect.height());
        
        let grid_size = 50.0;
        let grid_range = 500.0;
        let grid_color = egui::Color32::from_rgba_premultiplied(100, 100, 100, 30);

        // XZ 평면 격자 (Y=0에서)
        let y = 0.0;
        
        // X 방향 선들
        let mut x = -grid_range;
        while x <= grid_range {
            let start = Vec3::new(x, y, -grid_range);
            let end = Vec3::new(x, y, grid_range);
            
            let start_2d = Position3D::from_vec3(start)
                .project_to_screen(&view_matrix, &projection_matrix, viewport);
            let end_2d = Position3D::from_vec3(end)
                .project_to_screen(&view_matrix, &projection_matrix, viewport);
            
            painter.line_segment(
                [egui::pos2(start_2d.x, start_2d.y), egui::pos2(end_2d.x, end_2d.y)],
                egui::Stroke::new(1.0, grid_color),
            );
            
            x += grid_size;
        }
        
        // Z 방향 선들
        let mut z = -grid_range;
        while z <= grid_range {
            let start = Vec3::new(-grid_range, y, z);
            let end = Vec3::new(grid_range, y, z);
            
            let start_2d = Position3D::from_vec3(start)
                .project_to_screen(&view_matrix, &projection_matrix, viewport);
            let end_2d = Position3D::from_vec3(end)
                .project_to_screen(&view_matrix, &projection_matrix, viewport);
            
            painter.line_segment(
                [egui::pos2(start_2d.x, start_2d.y), egui::pos2(end_2d.x, end_2d.y)],
                egui::Stroke::new(1.0, grid_color),
            );
            
            z += grid_size;
        }
    }

    /// 3D 노드들 그리기
    fn draw_nodes_3d(
        &self,
        painter: &egui::Painter,
        rect: &egui::Rect,
        nodes: &[Node],
        selected_node: Option<&String>,
    ) {
        let view_matrix = self.camera.view_matrix();
        let projection_matrix = self.camera.projection_matrix();
        let viewport = Vec2::new(rect.width(), rect.height());

        // 깊이에 따라 노드 정렬 (뒤에서 앞으로)
        let mut sorted_nodes: Vec<_> = nodes.iter().collect();
        sorted_nodes.sort_by(|a, b| {
            let dist_a = (a.position_3d.to_vec3() - self.camera.position).length_squared();
            let dist_b = (b.position_3d.to_vec3() - self.camera.position).length_squared();
            dist_b.partial_cmp(&dist_a).unwrap_or(std::cmp::Ordering::Equal)
        });

        for node in sorted_nodes {
            if !node.is_visible {
                continue;
            }

            let screen_pos = node.position_3d.project_to_screen(
                &view_matrix,
                &projection_matrix,
                viewport,
            );

            // 화면 밖이면 그리지 않음
            if screen_pos.x < 0.0 || screen_pos.x > rect.width() ||
               screen_pos.y < 0.0 || screen_pos.y > rect.height() {
                continue;
            }

            let pos = egui::pos2(screen_pos.x, screen_pos.y);
            let size = node.get_layer_size() * node.scale;
            let layer_color = node.get_layer_color();
            
            let color = egui::Color32::from_rgba_premultiplied(
                (layer_color[0] * 255.0) as u8,
                (layer_color[1] * 255.0) as u8,
                (layer_color[2] * 255.0) as u8,
                (layer_color[3] * node.opacity * 255.0) as u8,
            );

            // 선택된 노드는 테두리 표시
            if selected_node.map_or(false, |id| id == &node.id) {
                painter.circle_stroke(
                    pos,
                    size + 3.0,
                    egui::Stroke::new(3.0, egui::Color32::WHITE),
                );
            }

            // 노드 원 그리기
            painter.circle_filled(pos, size, color);

            // 레이어 표시를 위한 내부 원
            if node.layer > 0 {
                painter.circle_filled(
                    pos,
                    size * 0.6,
                    egui::Color32::from_rgba_premultiplied(255, 255, 255, 100),
                );
            }

            // 노드 제목
            painter.text(
                pos,
                egui::Align2::CENTER_CENTER,
                &node.title,
                egui::FontId::proportional(12.0),
                egui::Color32::WHITE,
            );

            // 레이어 번호 표시
            if node.layer > 0 {
                let layer_text = format!("L{}", node.layer);
                painter.text(
                    egui::pos2(pos.x, pos.y - size - 10.0),
                    egui::Align2::CENTER_CENTER,
                    layer_text,
                    egui::FontId::monospace(10.0),
                    egui::Color32::YELLOW,
                );
            }
        }
    }

    /// 3D 연결선 그리기
    fn draw_connections_3d(&self, painter: &egui::Painter, rect: &egui::Rect, nodes: &[Node]) {
        let view_matrix = self.camera.view_matrix();
        let projection_matrix = self.camera.projection_matrix();
        let viewport = Vec2::new(rect.width(), rect.height());

        for node in nodes {
            if let Some(parent_id) = &node.parent_id {
                if let Some(parent) = nodes.iter().find(|n| &n.id == parent_id) {
                    let start_pos = parent.position_3d.project_to_screen(
                        &view_matrix,
                        &projection_matrix,
                        viewport,
                    );
                    let end_pos = node.position_3d.project_to_screen(
                        &view_matrix,
                        &projection_matrix,
                        viewport,
                    );

                    let start = egui::pos2(start_pos.x, start_pos.y);
                    let end = egui::pos2(end_pos.x, end_pos.y);

                    // 계층간 연결은 더 굵게
                    let thickness = if parent.layer != node.layer { 3.0 } else { 2.0 };
                    let color = if parent.layer != node.layer {
                        egui::Color32::from_rgb(255, 215, 0) // 골드
                    } else {
                        egui::Color32::from_rgba_premultiplied(150, 150, 255, 180)
                    };

                    painter.line_segment([start, end], egui::Stroke::new(thickness, color));
                }
            }
        }
    }

    /// 카메라 컨트롤 처리
    fn handle_camera_controls(&mut self, response: &egui::Response) {
        // 마우스 드래그로 카메라 회전
        if response.dragged_by(egui::PointerButton::Primary) {
            if let Some(delta) = response.drag_delta() {
                self.camera.rotate(delta.x, -delta.y);
            }
        }

        // 휠로 줌
        let scroll_delta = response.ctx.input(|i| i.scroll_delta.y);
        if scroll_delta != 0.0 {
            self.camera.zoom(-scroll_delta);
        }

        // 우클릭 드래그로 팬
        if response.dragged_by(egui::PointerButton::Secondary) {
            if let Some(delta) = response.drag_delta() {
                self.camera.pan(delta.x, -delta.y);
            }
        }
    }

    /// 컨트롤 UI 표시
    pub fn show_controls(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.label("3D Controls:");
            
            if ui.button("Reset View").clicked() {
                self.camera = Camera3D::new(self.camera.aspect_ratio);
            }
            
            ui.checkbox(&mut self.auto_rotate, "Auto Rotate");
            ui.checkbox(&mut self.show_grid, "Show Grid");
            ui.checkbox(&mut self.show_layer_platforms, "Show Platforms");
        });

        ui.horizontal(|ui| {
            ui.label("Focus Layer:");
            for layer in 0..=3 {
                if ui.button(&format!("L{}", layer)).clicked() {
                    self.camera.focus_on_layer(layer);
                }
            }
        });
    }

    /// 카메라 참조 반환
    pub fn camera(&self) -> &Camera3D {
        &self.camera
    }

    /// 카메라 가변 참조 반환
    pub fn camera_mut(&mut self) -> &mut Camera3D {
        &mut self.camera
    }
} 