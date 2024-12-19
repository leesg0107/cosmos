use eframe::egui;
use crate::celestial::{
    Graph, Node, NodeType, RelationType, Position2D
};
use crate::storage::{Storage, UniverseInfo};

#[derive(Debug, Clone)]
enum NodeCreationInfo {
    Root(egui::Pos2),
    Child {
        parent_id: String,
        node_type: NodeType,
        position: egui::Pos2,
    },
    Evolution {
        base_id: String,
        position: egui::Pos2,
    }
}

enum ShowMenu {
    Connection(String, String),
    NodeCreation(String, egui::Pos2),
}

#[derive(Debug, PartialEq)]
enum DragMode {
    None,
    ViewPan,           // 빈 공간 왼쪽 클릭 드래그로 화면 이동
    CreateNode,        // 오른쪽 클릭 드래그로 새 노드 생성
    MoveNode,          // 왼쪽 클릭 드래그로 노드 이동
}

pub struct CosmosView {
    graph: Graph,
    storage: Storage,
    dragging: Option<(String, egui::Pos2)>,     // 드래그 중인 노드
    view_offset: egui::Vec2,                    // 화면 이동량
    selected_node: Option<String>,              // 선택된 노드
    show_node_creator: bool,                    // 노드 생성 UI 표시 여부
    show_evolution_view: bool,                  // 추가
    new_node_title: String,                     // 새 노드 제목
    new_node_type: Option<NodeType>,            // 생성할 노드 타입
    hover_pos: Option<egui::Pos2>,              // 마우스 위치
    node_creation: Option<NodeCreationInfo>,  // 추가
    show_node_content: bool,                    // 노드 내용 표시 여부
    editing_node: bool,                         // 노드 편집 모드 여부
    drag_mode: DragMode,
    current_universe_id: Option<String>,  // 필드 추가
    last_save_time: std::time::Instant,  // 마지막 저장 시간 추가
}

impl CosmosView {
    pub fn new() -> Self {
        let mut graph = Graph::new();
        
        // 초기 Root 노드 생성
        graph.create_node(
            "New Root".to_string(),
            NodeType::Root,
            Position2D::new(400.0, 300.0)
        );

        Self {
            graph,
            storage: Storage::new(),
            dragging: None,
            view_offset: egui::Vec2::ZERO,
            selected_node: None,
            show_node_creator: false,
            show_evolution_view: false,
            new_node_title: String::new(),
            new_node_type: None,
            hover_pos: None,
            node_creation: None,
            show_node_content: false,
            editing_node: false,
            drag_mode: DragMode::None,
            current_universe_id: Some(uuid::Uuid::new_v4().to_string()),  // 새 우주 ID 생성
            last_save_time: std::time::Instant::now(),
        }
    }

    pub fn show(&mut self, ui: &mut egui::Ui) {
        // 상단 메뉴바 추가
        egui::TopBottomPanel::top("menu_bar").show(ui.ctx(), |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("BigBang", |ui| {
                    if ui.button("New Universe").clicked() {
                        self.create_new_universe();
                    }
                });

                ui.menu_button("TimeLog", |ui| {
                    // 저장된 마인드맵 목록 표시
                    for universe in self.get_saved_universes() {
                        if ui.button(&universe.title).clicked() {
                            self.load_universe(&universe.id);
                        }
                    }
                });

                if ui.button("BlackHole").clicked() {
                    // 프로그램 종료
                    std::process::exit(0);
                }
            });
        });

        // 기존 캔버스 영역
        egui::CentralPanel::default().show(ui.ctx(), |ui| {
            let (response, painter) = ui.allocate_painter(
                ui.available_size(),
                egui::Sense::click_and_drag()
            );

            // 배경 그리기
            self.draw_background(&painter, &response);

            // 드래그 연결선 그리기
            self.draw_relations(&painter, &response);
            self.draw_nodes(ui, &response, &painter);

            // 마우스 상호작용 처리
            self.handle_interactions(&response, ui, &painter);

            // 노드 생성 UI
            if self.show_node_creator {
                self.show_node_creator_ui(ui);
            }
        });

        // 노드 내용 창 표시
        if self.show_node_content {
            self.show_node_content_window(ui);
        }
    }

    fn draw_background(&self, painter: &egui::Painter, response: &egui::Response) {
        painter.rect_filled(
            response.rect,
            0.0,
            egui::Color32::from_rgb(16, 16, 24)
        );

        // 격자 그리기
        let grid_size = 50.0;
        let grid_color = egui::Color32::from_rgba_premultiplied(100, 100, 100, 30);
        
        for x in (0..(response.rect.width() as i32)).step_by(grid_size as usize) {
            let x = x as f32 + self.view_offset.x % grid_size;
            painter.line_segment(
                [
                    egui::pos2(x, 0.0),
                    egui::pos2(x, response.rect.height())
                ],
                egui::Stroke::new(1.0, grid_color)
            );
        }

        for y in (0..(response.rect.height() as i32)).step_by(grid_size as usize) {
            let y = y as f32 + self.view_offset.y % grid_size;
            painter.line_segment(
                [
                    egui::pos2(0.0, y),
                    egui::pos2(response.rect.width(), y)
                ],
                egui::Stroke::new(1.0, grid_color)
            );
        }
    }

    fn draw_nodes(&mut self, ui: &mut egui::Ui, response: &egui::Response, painter: &egui::Painter) {
        let nodes: Vec<_> = self.graph.get_nodes().cloned().collect();
        for node in &nodes {
            let pos = self.world_to_screen_pos(node.position.x, node.position.y);
            let (size, color) = self.get_node_style(node);

            // 선택된 노드는 테두리 표시
            if self.selected_node.as_ref() == Some(&node.id) {
                painter.circle_stroke(pos, size + 2.0, egui::Stroke::new(2.0, egui::Color32::WHITE));
            }

            // 노드 그리기
            painter.circle_filled(pos, size, color);
            
            // 노드 제목 표시
            let text = node.title.clone();
            painter.text(
                pos,
                egui::Align2::CENTER_CENTER,
                text,
                egui::TextStyle::Body.resolve(&ui.style()),
                egui::Color32::WHITE,
            );

            // 상호작용 영역
            let interact_rect = egui::Rect::from_center_size(
                pos,
                egui::vec2(size * 2.0, size * 2.0),
            );

            let node_response = ui.interact(
                interact_rect,
                ui.id().with(node.id.clone()),
                egui::Sense::click_and_drag()
            );

            // 클릭 처리
            if node_response.clicked() {
                self.handle_node_interaction(&node.id, &node_response);
            }

            // 드래그 시작과 드래그 중 처리
            if node_response.dragged() {
                // 드래그 시작
                if self.drag_mode == DragMode::None {
                    if ui.input(|i| i.pointer.secondary_down()) {
                        // 오른쪽 클릭 드래그: 새 노드 생성
                        if self.selected_node.as_ref() == Some(&node.id) {
                            self.drag_mode = DragMode::CreateNode;
                            self.dragging = Some((node.id.clone(), pos));
                        }
                    } else if ui.input(|i| i.pointer.primary_down()) {
                        // 왼쪽 클릭 드래그: 노드 이동
                        self.drag_mode = DragMode::MoveNode;
                        self.dragging = Some((node.id.clone(), pos));
                        self.selected_node = Some(node.id.clone());
                    }
                }

                // 드래그 중 처리
                match self.drag_mode {
                    DragMode::MoveNode => {
                        if let Some((dragged_id, _)) = &self.dragging {
                            if dragged_id == &node.id {
                                let delta = node_response.drag_delta();
                                if let Some(node) = self.graph.get_node_mut(dragged_id) {
                                    node.position.x += delta.x;
                                    node.position.y += delta.y;
                                    self.auto_save();
                                }
                            }
                        }
                    }
                    DragMode::CreateNode => {
                        if let Some((dragged_id, _)) = &self.dragging {
                            if dragged_id == &node.id {
                                if let Some(pointer_pos) = ui.input(|i| i.pointer.hover_pos()) {
                                    painter.line_segment(
                                        [pos, pointer_pos],
                                        egui::Stroke::new(2.0, egui::Color32::WHITE)
                                    );
                                }
                            }
                        }
                    }
                    _ => {}
                }
            }

            // 드래그 종료 처리
            if node_response.drag_released() {
                if let Some((dragged_id, _)) = &self.dragging {
                    if dragged_id == &node.id {
                        match self.drag_mode {
                            DragMode::CreateNode => {
                                if let Some(pointer_pos) = ui.input(|i| i.pointer.hover_pos()) {
                                    // 드래그가 다른 노드 위에서 끝났는지 확인
                                    if let Some(target_node) = self.find_node_at_pos(pointer_pos) {
                                        if target_node.id != *dragged_id {
                                            // 연결 메뉴 표시
                                            let source_id = dragged_id.clone();
                                            let target_id = target_node.id.clone();
                                            
                                            // 드래그 상태 초기화
                                            self.dragging = None;
                                            self.drag_mode = DragMode::None;
                                            
                                            // 연결 메뉴 표시
                                            self.show_connection_menu(&source_id, &target_id, &response.ctx);
                                            return;  // 여기서 함수 종료
                                        }
                                    } else {
                                        // 빈 공간에서 끝났다면 새 노드 생성 메뉴 표시
                                        self.show_node_creator = true;
                                        self.hover_pos = Some(pointer_pos);
                                        self.node_creation = Some(NodeCreationInfo::Child {
                                            parent_id: dragged_id.clone(),
                                            node_type: self.graph.get_node(dragged_id)
                                                .and_then(|n| n.node_type.next_level())
                                                .unwrap_or(NodeType::Category),
                                            position: pointer_pos,
                                        });
                                    }
                                }
                            }
                            _ => {}
                        }
                    }
                }
                // 드래그 상태 초기화
                self.dragging = None;
                self.drag_mode = DragMode::None;
            }
        }
    }

    fn handle_interactions(&mut self, response: &egui::Response, ui: &mut egui::Ui, _painter: &egui::Painter) {
        // 왼쪽 클릭 드래그로 화면 이동
        if response.drag_started() && ui.input(|i| i.pointer.primary_down()) {
            if !self.is_clicking_node(response.hover_pos().unwrap_or_default()) {
                self.drag_mode = DragMode::ViewPan;
            }
        }

        // 화면 이동 처리
        if response.dragged() && self.drag_mode == DragMode::ViewPan {
            self.view_offset += response.drag_delta();
        }

        // 드래그 종료 처리
        if response.drag_released() {
            match self.drag_mode {
                DragMode::CreateNode => {
                    if let Some((source_id, _)) = &self.dragging {
                        // 마우스 포인터 위치 가져오기
                        let pointer_pos = ui.input(|i| i.pointer.hover_pos()).unwrap_or_default();
                        
                        // 노드 생성 UI 상태 설정
                        if let Some(source_node) = self.graph.get_node(source_id) {
                            if let Some(next_type) = source_node.node_type.next_level() {
                                self.show_node_creator = true;
                                self.hover_pos = Some(pointer_pos);
                                self.node_creation = Some(NodeCreationInfo::Child {
                                    parent_id: source_id.to_string(),
                                    node_type: next_type,
                                    position: pointer_pos,
                                });
                                self.new_node_title.clear();
                            }
                        }
                    }
                    
                    // 드래그 상태 초기화는 UI 설정 후에
                    self.dragging = None;
                    self.drag_mode = DragMode::None;
                }
                _ => {
                    self.dragging = None;
                    self.drag_mode = DragMode::None;
                }
            }
        }

        // 빈블클릭으로 루트 노드 생성 (기존 기능 유지)
        if response.double_clicked() {
            let pos = response.hover_pos().unwrap_or_default();
            if !self.is_clicking_node(pos) {
                self.show_node_creator = true;
                self.hover_pos = Some(pos);
                self.node_creation = Some(NodeCreationInfo::Root(pos));
            }
        }
    }

    fn draw_relations(&mut self, painter: &egui::Painter, response: &egui::Response) {
        let mut clicked_relation = None;
        
        // 먼저 모든 관계를 수집
        let relations: Vec<_> = self.graph.get_relations().collect();

        // 관계선 그리기와 클릭 감지
        for relation in &relations {
            if let (Some(source), Some(target)) = (
                self.graph.get_node(&relation.source_id),
                self.graph.get_node(&relation.target_id)
            ) {
                let start = self.world_to_screen_pos(source.position.x, source.position.y);
                let end = self.world_to_screen_pos(target.position.x, target.position.y);

                // 연결선 스타일
                let (color, width) = match relation.relation_type {
                    RelationType::Orbit => (egui::Color32::from_rgb(100, 200, 255), 2.0),
                    RelationType::Evolution => (egui::Color32::from_rgb(100, 255, 100), 2.0),
                    RelationType::Reference => (egui::Color32::from_rgb(255, 100, 100), 1.0),
                    RelationType::Hierarchy => (egui::Color32::from_rgb(200, 200, 200), 1.0),
                };

                // 연결 그리기
                painter.line_segment([start, end], egui::Stroke::new(width, color));

                // 연결 클릭 지
                let line_rect = CosmosView::line_hit_area(start, end);
                if response.clicked() && line_rect.contains(response.hover_pos().unwrap_or_default()) {
                    clicked_relation = Some(relation.id.clone());
                    break;
                }
            }
        }

        // 클릭된 관계선 있으면 메뉴 표시
        if let Some(relation_id) = clicked_relation {
            self.show_relation_menu(&relation_id, &response.ctx);
        }
    }

    fn get_node_style(&self, node: &Node) -> (f32, egui::Color32) {
        match node.node_type {
            NodeType::Root => (
                40.0,
                egui::Color32::from_rgb(255, 200, 50)  // 노란색
            ),
            NodeType::Category => (
                30.0,
                egui::Color32::from_rgb(50, 150, 255)  // 파란색
            ),
            NodeType::Base => (
                20.0,
                egui::Color32::from_rgb(100, 255, 100) // 초록색
            ),
            NodeType::Star => (
                35.0,
                egui::Color32::from_rgb(255, 220, 100) //  노란색
            ),
            NodeType::Planet => (
                25.0,
                egui::Color32::from_rgb(100, 200, 255) // 하늘색
            ),
            NodeType::Satellite => (
                20.0,
                egui::Color32::from_rgb(200, 200, 200) // 회색
            ),
            NodeType::Asteroid => (
                15.0,
                egui::Color32::from_rgb(150, 150, 150) // 두운 회색
            ),
        }
    }

    fn world_to_screen_pos(&self, x: f32, y: f32) -> egui::Pos2 {
        egui::pos2(x + self.view_offset.x, y + self.view_offset.y)
    }

    fn screen_to_world_pos(&self, screen_pos: egui::Pos2) -> (f32, f32) {
        (
            screen_pos.x - self.view_offset.x,
            screen_pos.y - self.view_offset.y,
        )
    }

    fn show_node_creator_ui(&mut self, ui: &mut egui::Ui) {
        let creation_info = self.node_creation.clone();
        let mut should_create = None;

        match creation_info {
            Some(NodeCreationInfo::Root(pos)) => {
                egui::Window::new("Create Root Node")
                    .fixed_size([200.0, 150.0])
                    .current_pos(pos)
                    .show(ui.ctx(), |ui| {
                        ui.vertical(|ui| {
                            ui.label("Node Title:");
                            ui.text_edit_singleline(&mut self.new_node_title);
                            ui.add_space(10.0);

                            if ui.button("Create Root Node").clicked() {
                                should_create = Some(());
                            }

                            if ui.button("Cancel").clicked() {
                                self.show_node_creator = false;
                                self.new_node_title.clear();
                                self.node_creation = None;
                            }
                        });
                    });

                if should_create.is_some() {
                    self.create_root_node();
                }
            }
            Some(NodeCreationInfo::Child { parent_id, node_type, position }) => {
                egui::Window::new("Create Child Node")
                    .fixed_size([200.0, 150.0])
                    .current_pos(position)
                    .show(ui.ctx(), |ui| {
                        ui.vertical(|ui| {
                            ui.label("Node Title:");
                            ui.text_edit_singleline(&mut self.new_node_title);
                            ui.add_space(10.0);

                            if ui.button(format!("Create {}", node_type.display_name())).clicked() {
                                self.create_child_node(&parent_id, node_type, position);
                            }

                            if ui.button("Cancel").clicked() {
                                self.show_node_creator = false;
                                self.new_node_title.clear();
                                self.node_creation = None;
                            }
                        });
                    });
            }
            Some(NodeCreationInfo::Evolution { .. }) => {
                // Evolution 노드 생성 UI는 나중에 구현
            }
            None => {}
        }
    }

    fn show_evolution_chain(&mut self, ui: &mut egui::Ui, node_id: &str) {
        let chain: Vec<_> = self.graph.get_evolution_chain(node_id).into_iter().cloned().collect();
        
        egui::Window::new("Evolution Chain")
            .fixed_size([300.0, 400.0])
            .show(ui.ctx(), |ui| {
                ui.vertical(|ui| {
                    for (i, node) in chain.iter().enumerate() {
                        ui.horizontal(|ui| {
                            if i > 0 {
                                ui.label("↓");
                            }
                            
                            let mut text = node.title.clone();
                            if let Some(layer) = &node.time_layer {
                                text = format!("{} (L{})", text, layer.level);
                            }

                            if ui.selectable_label(
                                Some(node_id) == Some(&node.id),
                                text
                            ).clicked() {
                                self.selected_node = Some(node.id.clone());
                            }
                        });
                    }
                });
            });
    }

    fn create_root_node(&mut self) {
        if !self.new_node_title.is_empty() {
            if let Some(pos) = self.hover_pos {
                let (x, y) = self.screen_to_world_pos(pos);
                self.graph.create_node(
                    self.new_node_title.clone(),
                    NodeType::Root,
                    Position2D::new(x, y)
                );
                self.show_node_creator = false;
                self.new_node_title.clear();
                self.hover_pos = None;  // 위치 정보 초기화
                self.auto_save();
            }
        }
    }

    fn create_child_node(&mut self, parent_id: &str, node_type: NodeType, pos: egui::Pos2) {
        if !self.new_node_title.is_empty() {
            let (x, y) = self.screen_to_world_pos(pos);
            self.graph.create_child_node(
                self.new_node_title.clone(),
                node_type,
                parent_id,
                Position2D::new(x, y)
            );
            self.show_node_creator = false;
            self.new_node_title.clear();
            self.auto_save();
        }
    }

    fn create_evolution_node(&mut self) {
        if let Some(base_id) = &self.selected_node {
            if !self.new_node_title.is_empty() {
                let (x, y) = self.screen_to_world_pos(
                    self.hover_pos.unwrap_or_default()
                );
                self.graph.evolve_node(
                    base_id,
                    self.new_node_title.clone(),
                    Some(Position2D::new(x, y))
                );
                self.show_node_creator = false;
                self.new_node_title.clear();
                self.auto_save();
            }
        }
    }

    fn handle_node_click(&mut self, node_id: String) {
        self.selected_node = Some(node_id);
        self.show_evolution_view = true;
    }

    fn is_clicking_node(&self, pos: egui::Pos2) -> bool {
        for node in self.graph.get_nodes() {
            let node_pos = self.world_to_screen_pos(node.position.x, node.position.y);
            let (size, _) = self.get_node_style(node);
            let rect = egui::Rect::from_center_size(
                node_pos,
                egui::vec2(size * 2.0, size * 2.0),
            );
            if rect.contains(pos) {
                return true;
            }
        }
        false
    }

    fn show_node_creation_menu(&mut self, source_id: &str, pos: egui::Pos2, ctx: &egui::Context) {
        let source_node = self.graph.get_node(source_id).cloned();
        if let Some(source_node) = source_node {
            if let Some(next_type) = source_node.node_type.next_level() {
                // 상태 설정
                self.show_node_creator = true;  // 이 부분이 중요
                self.hover_pos = Some(pos);
                self.node_creation = Some(NodeCreationInfo::Child {
                    parent_id: source_id.to_string(),
                    node_type: next_type,
                    position: pos,
                });
            }
        }
    }

    fn show_relation_menu(&mut self, relation_id: &str, ctx: &egui::Context) {
        egui::Window::new("Connection Menu")
            .show(ctx, |ui| {
                if ui.button("Delete Connection").clicked() {
                    self.graph.remove_relation(relation_id);
                }
            });
    }

    fn show_connection_menu(&mut self, source_id: &str, target_id: &str, ctx: &egui::Context) {
        egui::Window::new("Create Connection")
            .show(ctx, |ui| {
                ui.vertical(|ui| {
                    if ui.button("Orbit Connection").clicked() {
                        self.graph.add_relation(source_id, target_id, RelationType::Orbit);
                        self.auto_save();
                    }
                    if ui.button("Evolution Connection").clicked() {
                        self.graph.add_relation(source_id, target_id, RelationType::Evolution);
                        self.auto_save();
                    }
                    if ui.button("Reference Connection").clicked() {
                        self.graph.add_relation(source_id, target_id, RelationType::Reference);
                        self.auto_save();
                    }
                    ui.separator();
                    if ui.button("Cancel").clicked() {
                        // 이미 드래깅 태는 초기화어 있음
                    }
                });
            });
    }

    // 선 클릭 감지를 한 히 영역 계산
    fn line_hit_area(start: egui::Pos2, end: egui::Pos2) -> egui::Rect {
        let padding = 5.0; // 클릭 영역 유
        let min_x = start.x.min(end.x) - padding;
        let max_x = start.x.max(end.x) + padding;
        let min_y = start.y.min(end.y) - padding;
        let max_y = start.y.max(end.y) + padding;
        egui::Rect::from_min_max(
            egui::pos2(min_x, min_y),
            egui::pos2(max_x, max_y)
        )
    }

    fn create_layer_node(&mut self, base_id: &str, pos: egui::Pos2) {
        if !self.new_node_title.is_empty() {
            let (x, y) = self.screen_to_world_pos(pos);
            self.graph.evolve_node(
                base_id,
                self.new_node_title.clone(),
                Some(Position2D::new(x, y))
            );
            self.show_node_creator = false;
            self.new_node_title.clear();
        }
    }

    fn create_new_universe(&mut self) {
        self.graph = Graph::new();
        self.dragging = None;
        self.selected_node = None;
        self.show_node_creator = false;  // 직접 노드 생성하지 않고 용의 더블클릭을 기림
        self.new_node_title.clear();
        self.view_offset = egui::Vec2::ZERO;
    }

    fn get_saved_universes(&self) -> Vec<UniverseInfo> {
        // storage에서 저장 마인드맵 목록 불러오
        self.storage.get_universe_list()
    }

    fn load_universe(&mut self, universe_id: &str) {
        // storage에서 선택한 마인드맵 불러오
        if let Some(universe) = self.storage.load_universe(universe_id) {
            self.graph = universe;
        }
    }

    fn find_node_at_pos(&self, pos: egui::Pos2) -> Option<Node> {
        for node in self.graph.get_nodes() {
            let node_pos = self.world_to_screen_pos(node.position.x, node.position.y);
            let (size, _) = self.get_node_style(node);
            let rect = egui::Rect::from_center_size(
                node_pos,
                egui::vec2(size * 2.0, size * 2.0),
            );
            if rect.contains(pos) {
                return Some(node.clone());
            }
        }
        None
    }

    fn handle_node_interaction(&mut self, node_id: &str, response: &egui::Response) {
        if response.double_clicked() {
            self.selected_node = Some(node_id.to_string());
            self.editing_node = true;
            self.show_node_content = true;
        } else if response.clicked() {
            self.selected_node = Some(node_id.to_string());
            self.show_node_content = true;
            self.editing_node = false;
        }
    }

    fn show_node_content_window(&mut self, ui: &mut egui::Ui) {
        if let Some(node_id) = &self.selected_node.clone() {  // clone 추가
            let node_title = self.graph.get_node(node_id)
                .map(|n| n.title.clone())
                .unwrap_or_default();
            let mut node_description = self.graph.get_node(node_id)
                .and_then(|n| n.description.clone())
                .unwrap_or_default();

            egui::Window::new(&node_title)
                .default_size([300.0, 200.0])
                .resizable(true)
                .collapsible(true)
                .show(ui.ctx(), |ui| {
                    if self.editing_node {
                        // 편집 모드
                        ui.vertical(|ui| {
                            ui.label("Description:");
                            ui.add_space(5.0);
                            
                            if ui.text_edit_multiline(&mut node_description).changed() {
                                // 텍스트가 변경될 마다 저장
                                if let Some(node) = self.graph.get_node_mut(node_id) {
                                    node.set_description(node_description.clone());
                                    self.auto_save();
                                }
                            }
                            
                            ui.add_space(10.0);
                            ui.horizontal(|ui| {
                                if ui.button("Done").clicked() {
                                    self.editing_node = false;
                                }
                                if ui.button("Cancel").clicked() {
                                    self.editing_node = false;
                                }
                            });
                        });
                    } else {
                        // 표시 모드
                        ui.vertical(|ui| {
                            if !node_description.is_empty() {
                                ui.label(&node_description);
                            } else {
                                ui.label("No description");
                            }
                            
                            ui.add_space(10.0);
                            ui.horizontal(|ui| {
                                if ui.button("Edit").clicked() {
                                    self.editing_node = true;
                                }
                                if ui.button("Close").clicked() {
                                    self.show_node_content = false;
                                }
                            });
                        });
                    }
                });
        }
    }

    fn auto_save(&mut self) {
        const SAVE_INTERVAL: std::time::Duration = std::time::Duration::from_secs(5);  // 5마다 저장
        
        let now = std::time::Instant::now();
        if now.duration_since(self.last_save_time) >= SAVE_INTERVAL {
            if let Some(universe_id) = &self.current_universe_id {
                self.storage.save_universe(&self.graph, universe_id);
                self.last_save_time = now;
            }
        }
    }
} 