use eframe::egui;
use crate::celestial::{
    Graph, Node, Position2D, NodeType, RelationType,
};
use crate::storage::Storage;
use crate::ui::{
    components::{
        node_creator::{NodeCreator, CreationAction},
        node_editor::{NodeEditor, EditorAction},
        connection_menu::{ConnectionMenu, ConnectionAction},
    },
    interactions::{
        drag_handler::{DragHandler, DragAction, DragMode},
        node_selector::NodeSelector,
    },
};
use crate::core::universe::Universe;
use crate::ui::effects::particle::Particle;

pub struct CosmosView {
    // 기본 데이터
    graph: Graph,
    storage: Storage,
    view_offset: egui::Vec2,
    current_universe_id: Option<String>,
    last_save_time: std::time::Instant,

    // UI 컴포넌트들
    node_creator: NodeCreator,
    node_editor: NodeEditor,
    connection_menu: ConnectionMenu,
    
    // 상호작용 핸들러들
    drag_handler: DragHandler,
    node_selector: NodeSelector,

    temp_connection_line: Option<(String, egui::Pos2)>, // 임시 연결선 (source_id, current_pos)
    show_start_menu: bool,  // 시작 메뉴 표시 여부

    particles: Vec<Particle>,
    big_bang_active: bool,
    big_bang_timer: f32,

    universe_title: String,  // 현재 우주의 이름

    show_time_log: bool,  // Time Log 창 표시 여부

    cached_universe_list: Vec<(String, String)>,  // (id, title) 캐시
}

impl CosmosView {
    pub fn new() -> Self {
        let mut graph = Graph::new();
        let graph_clone = graph.clone();
        
        //  Star 노드 생성
        graph.create_node(
            "New Star".to_string(),
            NodeType::Star,
            Position2D::new(400.0, 300.0)
        );

        Self {
            graph,
            storage: Storage::new(),
            view_offset: egui::Vec2::ZERO,
            current_universe_id: Some(uuid::Uuid::new_v4().to_string()),
            last_save_time: std::time::Instant::now(),

            node_creator: NodeCreator::new(graph_clone),
            node_editor: NodeEditor::new(),
            connection_menu: ConnectionMenu::new(),
            
            drag_handler: DragHandler::new(),
            node_selector: NodeSelector::new(),

            temp_connection_line: None,
            show_start_menu: true,  // 처음에는 시작 메뉴 표시

            particles: Vec::new(),
            big_bang_active: false,
            big_bang_timer: 0.0,

            universe_title: "New Universe".to_string(),

            show_time_log: false,

            cached_universe_list: Vec::new(),
        }
    }

    pub fn show(&mut self, ui: &mut egui::Ui) {
        if self.big_bang_active {
            self.update_big_bang(ui);
        } else if self.show_start_menu {
            self.show_start_menu(ui);
        } else {
            self.show_main_view(ui);
        }
    }

    fn show_start_menu(&mut self, ui: &mut egui::Ui) {
        // 전체 화면을 어둡게
        let screen_rect = ui.ctx().screen_rect();
        ui.painter().rect_filled(
            screen_rect,
            0.0,
            egui::Color32::from_black_alpha(240)
        );

        // Time Log 창
        if self.show_time_log {
            // 창이 열릴 때 목록 갱신
            if self.cached_universe_list.is_empty() {
                self.update_universe_list();
            }

            egui::Window::new("Time Log")
                .fixed_size([400.0, 300.0])
                .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
                .collapsible(false)
                .show(ui.ctx(), |ui| {
                    ui.heading("Previous Universes");
                    ui.add_space(10.0);

                    egui::ScrollArea::vertical().show(ui, |ui| {
                        if self.cached_universe_list.is_empty() {
                            ui.label("No universes found");
                        } else {
                            let mut to_delete = None;
                            let mut to_load = None;

                            for (id, title) in &self.cached_universe_list {
                                ui.horizontal(|ui| {
                                    if ui.button(title).clicked() {
                                        to_load = Some(id.clone());
                                    }
                                    
                                    if ui.button("🗑")
                                        .on_hover_text("Delete this universe")
                                        .clicked() 
                                    {
                                        to_delete = Some(id.clone());
                                    }
                                });
                            }

                            // 클로저 밖에서 상태 변경 처리
                            if let Some(id) = to_delete {
                                if self.storage.delete_universe(&id) {
                                    self.cached_universe_list.retain(|(i, _)| i != &id);
                                }
                            }

                            if let Some(id) = to_load {
                                self.load_universe(&id);
                                self.show_time_log = false;
                                self.show_start_menu = false;
                            }
                        }
                    });

                    ui.separator();
                    if ui.button("Close").clicked() {
                        self.show_time_log = false;
                        self.cached_universe_list.clear();
                    }
                });
            return;
        }

        // 중앙에 메뉴 배치
        egui::CentralPanel::default().show(ui.ctx(), |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(screen_rect.height() * 0.3);  // 위쪽 여백

                ui.heading("COSMOS");
                ui.add_space(40.0);

                let button_size = egui::vec2(200.0, 50.0);
                
                if ui.add_sized(button_size, egui::Button::new("Big Bang"))
                    .on_hover_text("Create new universe")
                    .clicked() 
                {
                    self.start_big_bang(ui.clip_rect().center());
                }

                if ui.add_sized(button_size, egui::Button::new("Time Log"))
                    .on_hover_text("Load previous universe")
                    .clicked() 
                {
                    self.show_time_log = true;  // Time Log 창 표시
                }

                if ui.add_sized(button_size, egui::Button::new("Black Hole"))
                    .on_hover_text("Exit")
                    .clicked() 
                {
                    std::process::exit(0);
                }
            });
        });
    }

    fn show_main_view(&mut self, ui: &mut egui::Ui) {
        // 상단 메뉴바
        egui::TopBottomPanel::top("menu_bar").show(ui.ctx(), |ui| {
            egui::menu::bar(ui, |ui| {
                if ui.button("⬅ Back").clicked() {
                    self.handle_back_button(ui);
                }

                ui.separator();

                ui.menu_button("BigBang", |ui| {
                    if ui.button("New Universe").clicked() {
                        self.create_new_universe();
                    }
                });

                ui.menu_button("TimeLog", |ui| {
                    let universe_list: Vec<_> = self.storage.get_universe_list()
                        .map(|u| (u.id.clone(), u.title.clone()))
                        .collect();
                    for (id, title) in &universe_list {
                        if ui.button(title).clicked() {
                            self.load_universe(id);
                        }
                    }
                });

                if ui.button("BlackHole").clicked() {
                    std::process::exit(0);
                }

                // 우주 이름 편집 UI
                ui.separator();
                ui.label("Universe Name:");
                if ui.text_edit_singleline(&mut self.universe_title).changed() {
                    self.auto_save();
                }
            });
        });

        // 메인 캔버스
        egui::CentralPanel::default().show(ui.ctx(), |ui| {
            let (response, painter) = ui.allocate_painter(
                ui.available_size(),
                egui::Sense::click_and_drag()
            );

            // 배경 그리기
            self.draw_background(&painter, &response);

            // 관계선 그리기
            self.draw_relations(&painter, &response);

            // 노드 그리기 및 상호작용 처리
            self.draw_nodes(ui, &response, &painter);

            // 드래그 처리
            if let Some(action) = self.drag_handler.handle_drag(&response, ui, &mut self.graph) {
                self.handle_drag_action(action, &response);
            }

            // 노드 생성 UI
            if let Some(action) = self.node_creator.show(ui) {
                self.handle_creation_action(action);
            }

            // 노드 편집 UI
            if let Some(mut node) = self.node_selector.get_selected_node(&self.graph).cloned() {
                if let Some(action) = self.node_editor.show(ui, &mut node) {
                    self.handle_editor_action(action);
                }
            }

            // 연결 메뉴
            if let Some(action) = self.connection_menu.show(ui) {
                self.handle_connection_action(action);
            }
        });
    }

    fn handle_drag_action(&mut self, action: DragAction, _response: &egui::Response) {
        match action {
            DragAction::SelectNode(node_id) => {
                self.node_selector.select_node(node_id);
                self.temp_connection_line = None;
            }
            DragAction::Deselect => {
                self.node_selector.deselect();
                self.temp_connection_line = None;
            }
            DragAction::StartViewPan => {
                self.temp_connection_line = None;
                self.node_selector.deselect();
            }
            DragAction::StartMoveNode(node_id) => {
                self.node_selector.select_node(node_id);
                self.temp_connection_line = None;
            }
            DragAction::ViewPan(delta) => {
                self.view_offset += delta;
            }
            DragAction::Dragging { node_id, mode: DragMode::MoveNode, current_pos } => {
                if let Some(node) = self.graph.get_node_mut(&node_id) {
                    node.position = Position2D::new(
                        current_pos.x - self.view_offset.x,
                        current_pos.y - self.view_offset.y,
                    );
                }
            }
            DragAction::StartDrawConnection(source_id, pos) => {
                self.temp_connection_line = Some((source_id, pos));
            }
            DragAction::DrawingConnection { source_id, current_pos } => {
                self.temp_connection_line = Some((source_id, current_pos));
            }
            DragAction::EndDrawConnection { source_id: _, end_pos: _ } => {
                self.temp_connection_line = None;
            }
            DragAction::EndMoveNode { node_id: _, end_pos: _ } => {
                // 노드 이동 종료 - 특별한 처리 필요 없음
            }
            DragAction::RequestCreateNode(pos) => {
                // 빈 공간 더블클릭: 새 Star 노드 생성
                let screen_pos = Position2D::new(
                    pos.x - self.view_offset.x,
                    pos.y - self.view_offset.y,
                );
                let node_id = self.graph.create_node(
                    "New Star".to_string(),
                    NodeType::Star,
                    screen_pos
                );
                
                // 새로 생성된 노드를 바로 선택 상태로 만들고
                // 다른 ��태들 초기화
                self.node_selector.select_node(node_id);
                self.temp_connection_line = None;
                self.node_creator.show_creator = false;
                self.drag_handler.drag_mode = DragMode::None;
                self.drag_handler.dragging = None;
                
                self.auto_save();
            }
            DragAction::NodeDoubleClicked(node_id) => {
                // 노드 더블클릭: 편집 UI 표시
                self.node_editor.show_editor = true;
                let node_id_clone = node_id.clone();
                self.node_editor.editing_node = Some(node_id);
                if let Some(node) = self.graph.get_node(&node_id_clone) {
                    self.node_editor.description = node.description.clone().unwrap_or_default();
                }
            }
            DragAction::CreateChildNode { parent_id, position } => {
                if let Some(parent) = self.graph.get_node(&parent_id) {
                    let child_type = match parent.node_type {
                        NodeType::Star => Some(NodeType::Planet),
                        NodeType::Planet => Some(NodeType::Satellite),
                        NodeType::Satellite => Some(NodeType::Asteroid),
                        _ => None,
                    };

                    if let Some(node_type) = child_type {
                        let screen_pos = Position2D::new(
                            position.x - self.view_offset.x,
                            position.y - self.view_offset.y,
                        );
                        let title = format!("New {}", node_type.display_name());
                        let new_node_id = self.graph.create_child_node(title, node_type, &parent_id);
                        
                        // 새로 생성된 노드의 위치를 설정
                        if let Some(node_id) = new_node_id {
                            if let Some(new_node) = self.graph.get_node_mut(&node_id) {
                                new_node.position = screen_pos;
                            }
                        }
                    }
                }
                self.temp_connection_line = None;
            }
            DragAction::Dragging { mode: _, .. } => {
                // 다른 드래그 모드는 무시
            }
        }
    }

    fn draw_background(&self, painter: &egui::Painter, response: &egui::Response) {
        // 경 그리기를 단순화
        painter.rect_filled(
            response.rect,
            0.0,
            egui::Color32::from_rgb(16, 16, 24)
        );

        // 자 그리기 최적화
        let grid_size = 50.0;
        let grid_color = egui::Color32::from_rgba_premultiplied(100, 100, 100, 30);
        
        // 화면에 보는 영역만 계산
        let start_x = (-self.view_offset.x / grid_size).floor() as i32;
        let end_x = ((response.rect.width() - self.view_offset.x) / grid_size).ceil() as i32;
        let start_y = (-self.view_offset.y / grid_size).floor() as i32;
        let end_y = ((response.rect.height() - self.view_offset.y) / grid_size).ceil() as i32;
        
        // 화면에 보이는 격자만 그리기
        for x in start_x..=end_x {
            let x_pos = x as f32 * grid_size + self.view_offset.x;
            if x_pos >= 0.0 && x_pos <= response.rect.width() {
                painter.line_segment(
                    [
                        egui::pos2(x_pos, 0.0),
                        egui::pos2(x_pos, response.rect.height())
                    ],
                    egui::Stroke::new(1.0, grid_color)
                );
            }
        }

        for y in start_y..=end_y {
            let y_pos = y as f32 * grid_size + self.view_offset.y;
            if y_pos >= 0.0 && y_pos <= response.rect.height() {
                painter.line_segment(
                    [
                        egui::pos2(0.0, y_pos),
                        egui::pos2(response.rect.width(), y_pos)
                    ],
                    egui::Stroke::new(1.0, grid_color)
                );
            }
        }
    }

    fn draw_relations(&self, painter: &egui::Painter, _response: &egui::Response) {
        for relation in self.graph.get_relations() {
            if let (Some(source), Some(target)) = (
                self.graph.get_node(&relation.source_id),
                self.graph.get_node(&relation.target_id)
            ) {
                let (start_x, start_y) = source.position.to_screen_pos();
                let (end_x, end_y) = target.position.to_screen_pos();
                let start = self.world_to_screen_pos(start_x, start_y);
                let end = self.world_to_screen_pos(end_x, end_y);

                match relation.relation_type {
                    RelationType::Evolution => {
                        // 진화 관계는 나선형 곡선으로 표현
                        self.draw_spiral_connection(painter, start, end);
                    }
                    RelationType::Hierarchy => {
                        // 층 관계는 점선으로 표현
                        painter.line_segment(
                            [start, end],
                            egui::Stroke::new(1.0, egui::Color32::from_rgb(165, 142, 117))
                        );
                    }
                    RelationType::Reference => {
                        // 참조 관계는 곡선으로 표현
                        self.draw_curved_connection(painter, start, end);
                    }
                    RelationType::Orbit => {
                        // 궤도 관계는 원형 곡선으로 표현
                        self.draw_orbit_connection(painter, start, end);
                    }
                }
            }
        }

        // 임시 연결선 그리기 (드래그 중)
        if let Some((source_id, current_pos)) = &self.temp_connection_line {
            if let Some(source) = self.graph.get_node(source_id) {
                let start = self.world_to_screen_pos(source.position.x, source.position.y);
                painter.line_segment(
                    [start, *current_pos],
                    egui::Stroke::new(2.0, egui::Color32::from_rgb(150, 150, 150))
                );
            }
        }
    }

    // 궤도 연결선 그리기 함수 추가
    fn draw_orbit_connection(&self, painter: &egui::Painter, start: egui::Pos2, end: egui::Pos2) {
        let center = egui::pos2(
            (start.x + end.x) / 2.0,
            (start.y + end.y) / 2.0
        );
        let radius = start.distance(center);
        
        // 원형 궤도 리기
        const SEGMENTS: usize = 32;
        let mut points = Vec::with_capacity(SEGMENTS + 1);
        
        for i in 0..=SEGMENTS {
            let angle = (i as f32 / SEGMENTS as f32) * std::f32::consts::TAU;
            let x = center.x + radius * angle.cos();
            let y = center.y + radius * angle.sin();
            points.push(egui::pos2(x, y));
        }

        for i in 0..points.len() - 1 {
            painter.line_segment(
                [points[i], points[i + 1]],
                egui::Stroke::new(1.0, egui::Color32::from_rgb(100, 200, 255))
            );
        }
    }

    // 나선형 연결선 그리기
    fn draw_spiral_connection(&self, painter: &egui::Painter, start: egui::Pos2, end: egui::Pos2) {
        let control1 = egui::pos2(
            start.x + (end.x - start.x) * 0.5 - 30.0,
            start.y + (end.y - start.y) * 0.2
        );
        let control2 = egui::pos2(
            start.x + (end.x - start.x) * 0.5 + 30.0,
            start.y + (end.y - start.y) * 0.8
        );

        // 베지어 곡선으로 나선형 과
        self.draw_bezier_curve(painter, start, control1, control2, end);
    }

    // 선 연결선 그리기
    fn draw_curved_connection(&self, painter: &egui::Painter, start: egui::Pos2, end: egui::Pos2) {
        let mid_x = (start.x + end.x) / 2.0;
        let control = egui::pos2(mid_x, start.y);
        
        self.draw_bezier_curve(painter, start, control, control, end);
    }

    // 베지어 곡선 그리기 헬 함수
    fn draw_bezier_curve(
        &self,
        painter: &egui::Painter,
        start: egui::Pos2,
        control1: egui::Pos2,
        control2: egui::Pos2,
        end: egui::Pos2
    ) {
        const STEPS: usize = 20;
        let mut points = Vec::with_capacity(STEPS + 1);
        
        for i in 0..=STEPS {
            let t = i as f32 / STEPS as f32;
            let point = self.cubic_bezier(start, control1, control2, end, t);
            points.push(point);
        }

        for i in 0..points.len() - 1 {
            painter.line_segment(
                [points[i], points[i + 1]],
                egui::Stroke::new(1.0, egui::Color32::from_rgb(165, 142, 117))
            );
        }
    }

    // 3차 베지어 곡선 계산
    fn cubic_bezier(
        &self,
        start: egui::Pos2,
        control1: egui::Pos2,
        control2: egui::Pos2,
        end: egui::Pos2,
        t: f32
    ) -> egui::Pos2 {
        let t2 = t * t;
        let t3 = t2 * t;
        let mt = 1.0 - t;
        let mt2 = mt * mt;
        let mt3 = mt2 * mt;
        
        egui::pos2(
            start.x * mt3 + 3.0 * control1.x * mt2 * t + 3.0 * control2.x * mt * t2 + end.x * t3,
            start.y * mt3 + 3.0 * control1.y * mt2 * t + 3.0 * control2.y * mt * t2 + end.y * t3
        )
    }

    fn handle_creation_action(&mut self, action: CreationAction) {
        match action {
            CreationAction::CreateRoot { title, description, position } => {
                let node = self.graph.create_node(title, NodeType::Star, position);
                if let Some(node) = self.graph.get_node_mut(&node) {
                    node.set_description(description);
                }
                self.auto_save();
            }
            CreationAction::CreateChild { parent_id, title, description, node_type, position: _ } => {
                if let Some(node_id) = self.graph.create_child_node(title, node_type, &parent_id) {
                    if let Some(node) = self.graph.get_node_mut(&node_id) {
                        node.set_description(description);
                    }
                }
                self.auto_save();
            }
            CreationAction::CreateEvolution { base_id, title, description, position } => {
                if let Some(node_id) = self.graph.evolve_node(&base_id, title, Some(position)) {
                    if let Some(node) = self.graph.get_node_mut(&node_id) {
                        node.set_description(description);
                    }
                }
                self.auto_save();
            }
        }
    }

    fn handle_editor_action(&mut self, action: EditorAction) {
        match action {
            EditorAction::UpdateTitle(title) => {
                if let Some(node_id) = &self.node_selector.selected_node {
                    if let Some(node) = self.graph.get_node_mut(node_id) {
                        node.set_title(title);
                        self.auto_save();
                    }
                }
            }
            EditorAction::UpdateDescription(description) => {
                if let Some(node_id) = &self.node_selector.selected_node {
                    if let Some(node) = self.graph.get_node_mut(node_id) {
                        node.set_description(description);
                        self.auto_save();
                    }
                }
            }
            EditorAction::CreateEvolutionLayer => {
                if let Some(node_id) = &self.node_selector.selected_node {
                    if let Some(base_node) = self.graph.get_node(node_id) {
                        let new_pos = Position2D::new(
                            base_node.position.x,
                            base_node.position.y - 50.0,  // 로 오프셋
                        );
                        let title = format!("{} Evolution", base_node.title);
                        self.graph.evolve_node(node_id, title, Some(new_pos));
                        self.auto_save();
                    }
                }
                self.node_editor.show_editor = false;
            }
            EditorAction::UpdateColor(color) => {
                if let Some(node_id) = &self.node_selector.selected_node {
                    if let Some(node) = self.graph.get_node_mut(node_id) {
                        node.set_color(color);
                        self.auto_save();
                    }
                }
            }
            EditorAction::UpdateSize(size) => {
                if let Some(node_id) = &self.node_selector.selected_node {
                    if let Some(node) = self.graph.get_node_mut(node_id) {
                        node.set_size(size);
                        self.auto_save();
                    }
                }
            }
        }
    }

    fn handle_connection_action(&mut self, action: ConnectionAction) {
        match action {
            ConnectionAction::CreateConnection { source_id, target_id, relation_type } => {
                self.graph.add_relation(&source_id, &target_id, relation_type);
                self.auto_save();
            }
        }
    }

    fn world_to_screen_pos(&self, x: f32, y: f32) -> egui::Pos2 {
        egui::pos2(x + self.view_offset.x, y + self.view_offset.y)
    }

    fn auto_save(&mut self) {
        const SAVE_INTERVAL: std::time::Duration = std::time::Duration::from_secs(5);
        let now = std::time::Instant::now();
        
        if now.duration_since(self.last_save_time) >= SAVE_INTERVAL {
            if let Some(universe_id) = &self.current_universe_id {
                let mut universe = Universe::from(self.graph.clone());
                universe.title = self.universe_title.clone();
                self.storage.save_universe(&universe, universe_id);
                self.last_save_time = now;
            }
        }
    }

    fn create_new_universe(&mut self) {
        self.graph = Graph::new();
        self.universe_title = "New Universe".to_string();
        self.current_universe_id = Some(uuid::Uuid::new_v4().to_string());
        self.drag_handler.dragging = None;
        self.node_selector.selected_node = None;
        self.node_creator.show_creator = false;
        self.node_creator.new_node_title.clear();
        self.view_offset = egui::Vec2::ZERO;
    }

    fn load_universe(&mut self, universe_id: &str) {
        if let Some(universe) = self.storage.load_universe(universe_id) {
            self.universe_title = universe.title.clone();
            self.current_universe_id = Some(universe_id.to_string());
            self.graph = universe.into();
        }
    }

    fn draw_nodes(&mut self, ui: &mut egui::Ui, _response: &egui::Response, painter: &egui::Painter) {
        let mut layer_nodes: Vec<_> = self.graph.get_nodes().collect();
        layer_nodes.sort_by_key(|node| node.position.z as i32);

        for node in &layer_nodes {
            let (screen_x, screen_y) = node.position.to_screen_pos();
            let pos = self.world_to_screen_pos(screen_x, screen_y);
            let (size, color) = self.get_node_style(node);

            // 선택 노드는 흰색 테두리로 강조
            if self.node_selector.is_selected(&node.id) {
                painter.circle_stroke(
                    pos,
                    size + 2.0,
                    egui::Stroke::new(2.0, egui::Color32::WHITE)
                );
            }

            // 노드 본체
            painter.circle_filled(pos, size, color);

            // 노드 제목
            let galley = ui.painter().layout_no_wrap(
                node.title.clone(),
                egui::TextStyle::Body.resolve(ui.style()),
                egui::Color32::WHITE,
            );
            
            painter.galley(
                pos - egui::vec2(galley.size().x / 2.0, galley.size().y / 2.0),
                galley,
            );
        }
    }

    fn get_node_style(&self, node: &Node) -> (f32, egui::Color32) {
        let color = node.custom_color.unwrap_or_else(|| {
            match node.node_type {
                NodeType::Star => egui::Color32::from_rgb(255, 223, 186),
                NodeType::Planet => egui::Color32::from_rgb(186, 223, 255),
                NodeType::Satellite => egui::Color32::from_rgb(200, 200, 200),
                NodeType::Asteroid => egui::Color32::from_rgb(169, 169, 169),
            }
        });

        let size = node.custom_size.unwrap_or_else(|| {
            match node.node_type {
                NodeType::Star => 20.0,
                NodeType::Planet => 15.0,
                NodeType::Satellite => 8.0,
                NodeType::Asteroid => 5.0,
            }
        });

        (size, color)
    }

    fn start_big_bang(&mut self, screen_center: egui::Pos2) {
        self.big_bang_active = true;
        self.big_bang_timer = 0.0;
        self.particles.clear();
        
        // 초기 파티클 수 증가
        for _ in 0..200 {  // 100 -> 200
            self.particles.push(Particle::new(screen_center));
        }
    }

    fn update_big_bang(&mut self, ui: &egui::Ui) {
        if !self.big_bang_active {
            return;
        }

        self.big_bang_timer += 1.0 / 60.0;
        
        // 파티클 업데이트 및 그리기
        self.particles.retain_mut(|p| {
            p.update();
            p.draw(ui.painter());
            p.is_alive()
        });

        // 새 파티클 추가 빈도 증가
        if self.big_bang_timer < 1.0 {
            let center = ui.clip_rect().center();
            for _ in 0..10 {  // 5 -> 10
                self.particles.push(Particle::new(center));
            }
        }

        // 과 지속 시간 증가
        if self.big_bang_timer >= 3.0 {  // 2.0 -> 3.0
            self.big_bang_active = false;
            self.create_new_universe();
            self.show_start_menu = false;
        }
    }

    fn handle_back_button(&mut self, ui: &mut egui::Ui) {
        if self.universe_title.is_empty() {
            // 이름이 비어있으면 이름 입력 다이얼로그 표시
            let mut show_dialog = true;
            egui::Window::new("Name Your Universe")
                .collapsible(false)
                .resizable(false)
                .show(ui.ctx(), |ui| {
                    ui.label("Please enter a name for your universe:");
                    if ui.text_edit_singleline(&mut self.universe_title).changed() {
                        // 이름 입력 처리
                    }
                    ui.horizontal(|ui| {
                        if ui.button("Save & Exit").clicked() {
                            if !self.universe_title.is_empty() {
                                self.save_and_exit();
                                show_dialog = false;
                            }
                        }
                        if ui.button("Cancel").clicked() {
                            show_dialog = false;
                        }
                    });
                });
            
            if !show_dialog && !self.universe_title.is_empty() {
                self.save_and_exit();
            }
        } else {
            // 이름이 있으면 바로 저장하고 종료
            self.save_and_exit();
        }
    }

    fn save_and_exit(&mut self) {
        // 현재 상태 저장
        if let Some(universe_id) = &self.current_universe_id {
            let mut universe = Universe::from(self.graph.clone());
            universe.title = self.universe_title.clone();
            self.storage.save_universe(&universe, universe_id);
        }
        // 시작 화면으로 돌아가기
        self.show_start_menu = true;
    }

    fn update_universe_list(&mut self) {
        self.cached_universe_list = self.storage.get_universe_list()
            .map(|u| (u.id.clone(), u.title.clone()))
            .collect();
    }
} 