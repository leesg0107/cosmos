use eframe::egui;

// 3D 케이크 구조의 노드
#[derive(Clone, Debug)]
struct CakeNode {
    id: String,
    title: String,
    layer: usize,
    angle: f32,
    radius: f32,
    node_type: NodeType,
    cake_id: String,  // 어느 케이크에 속하는지
    selected: bool,
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum NodeType {
    Root,    // 🌌
    Concept, // 💭
    Task,    // 📋
    Note,    // 📝
}

impl NodeType {
    fn emoji(&self) -> &'static str {
        match self {
            NodeType::Root => "🌌",
            NodeType::Concept => "💭",
            NodeType::Task => "📋",
            NodeType::Note => "📝",
        }
    }

    fn color(&self) -> egui::Color32 {
        match self {
            NodeType::Root => egui::Color32::from_rgb(255, 215, 0),   // 골드
            NodeType::Concept => egui::Color32::from_rgb(100, 149, 237), // 블루
            NodeType::Task => egui::Color32::from_rgb(255, 105, 180), // 핑크
            NodeType::Note => egui::Color32::from_rgb(50, 205, 50),   // 그린
        }
    }

    fn layer(&self) -> usize {
        match self {
            NodeType::Root => 0,
            NodeType::Concept => 1,
            NodeType::Task => 2,
            NodeType::Note => 3,
        }
    }
}

// 케이크 구조 정의
#[derive(Clone, Debug)]
struct CakeStructure {
    id: String,
    title: String,
    center: egui::Pos2,
    size_scale: f32,
    color_theme: usize, // 0: 기본, 1: 따뜻한톤, 2: 차가운톤, 3: 자연톤
    max_layer: usize,   // 동적 레이어 관리
}

impl CakeStructure {
    fn new(id: String, title: String, center: egui::Pos2) -> Self {
        Self {
            id,
            title,
            center,
            size_scale: 1.0,
            color_theme: 0,
            max_layer: 0,  // 기본 1레이어(0부터 시작)
        }
    }

    fn get_layer_color(&self, layer: usize) -> egui::Color32 {
        match self.color_theme {
            0 => match layer { // 기본 테마
                0 => egui::Color32::from_rgba_unmultiplied(255, 215, 0, 120),
                1 => egui::Color32::from_rgba_unmultiplied(173, 216, 230, 120),
                2 => egui::Color32::from_rgba_unmultiplied(255, 182, 193, 120),
                3 => egui::Color32::from_rgba_unmultiplied(144, 238, 144, 120),
                4 => egui::Color32::from_rgba_unmultiplied(255, 160, 122, 120),
                5 => egui::Color32::from_rgba_unmultiplied(221, 160, 221, 120),
                _ => egui::Color32::from_rgba_unmultiplied(200, 200, 200, 120),
            },
            1 => match layer { // 따뜻한 테마
                0 => egui::Color32::from_rgba_unmultiplied(255, 140, 0, 120),
                1 => egui::Color32::from_rgba_unmultiplied(255, 160, 122, 120),
                2 => egui::Color32::from_rgba_unmultiplied(255, 192, 203, 120),
                3 => egui::Color32::from_rgba_unmultiplied(255, 218, 185, 120),
                4 => egui::Color32::from_rgba_unmultiplied(255, 228, 196, 120),
                5 => egui::Color32::from_rgba_unmultiplied(255, 239, 213, 120),
                _ => egui::Color32::from_rgba_unmultiplied(255, 245, 230, 120),
            },
            2 => match layer { // 차가운 테마
                0 => egui::Color32::from_rgba_unmultiplied(70, 130, 180, 120),
                1 => egui::Color32::from_rgba_unmultiplied(135, 206, 250, 120),
                2 => egui::Color32::from_rgba_unmultiplied(173, 216, 230, 120),
                3 => egui::Color32::from_rgba_unmultiplied(224, 255, 255, 120),
                4 => egui::Color32::from_rgba_unmultiplied(240, 248, 255, 120),
                5 => egui::Color32::from_rgba_unmultiplied(248, 248, 255, 120),
                _ => egui::Color32::from_rgba_unmultiplied(250, 250, 255, 120),
            },
            _ => match layer { // 자연 테마
                0 => egui::Color32::from_rgba_unmultiplied(139, 69, 19, 120),
                1 => egui::Color32::from_rgba_unmultiplied(34, 139, 34, 120),
                2 => egui::Color32::from_rgba_unmultiplied(154, 205, 50, 120),
                3 => egui::Color32::from_rgba_unmultiplied(240, 230, 140, 120),
                4 => egui::Color32::from_rgba_unmultiplied(255, 255, 224, 120),
                5 => egui::Color32::from_rgba_unmultiplied(250, 240, 230, 120),
                _ => egui::Color32::from_rgba_unmultiplied(245, 245, 220, 120),
            }
        }
    }
    
    // 레이어 확장
    fn expand_to_layer(&mut self, layer: usize) {
        if layer > self.max_layer {
            self.max_layer = layer;
        }
    }
}

// 연결선 (케이크 간 연결 포함)
#[derive(Clone, Debug)]
struct Connection {
    from_id: String,
    to_id: String,
    connection_type: ConnectionType,
}

#[derive(Clone, Debug, PartialEq)]
enum ConnectionType {
    IntraCake,  // 케이크 내부 연결
    InterCake,  // 케이크 간 연결
}

#[derive(Debug)]
enum InteractionMode {
    None,
    DraggingNode(String),
    CreatingConnection(String),  // 드래그로 연결 생성
    DraggingCake(String),
    PanningView,  // 두 손가락 팬
}

// 뷰포트 관리 구조체
#[derive(Clone, Debug)]
struct Viewport {
    offset: egui::Vec2,
    zoom: f32,
}

impl Default for Viewport {
    fn default() -> Self {
        Self {
            offset: egui::Vec2::ZERO,
            zoom: 1.0,
        }
    }
}

impl Viewport {
    fn screen_to_world(&self, screen_pos: egui::Pos2, canvas_rect: egui::Rect) -> egui::Pos2 {
        let canvas_center = canvas_rect.center();
        let relative_pos = (screen_pos - canvas_center) / self.zoom;
        canvas_center + relative_pos - self.offset
    }

    fn world_to_screen(&self, world_pos: egui::Pos2, canvas_rect: egui::Rect) -> egui::Pos2 {
        let canvas_center = canvas_rect.center();
        let relative_pos = (world_pos - canvas_center + self.offset) * self.zoom;
        canvas_center + relative_pos
    }
}

// 다차원 케이크 그래프 앱
struct Cosmos3DApp {
    // 케이크 구조들
    cakes: Vec<CakeStructure>,
    nodes: Vec<CakeNode>,
    connections: Vec<Connection>,
    
    // 뷰포트 및 네비게이션
    viewport: Viewport,
    
    // 선택 상태
    selected_node: Option<String>,
    selected_cake: Option<String>,
    
    // 인터랙션
    interaction_mode: InteractionMode,
    drag_start_pos: Option<egui::Pos2>,
    
    // UI 상태
    show_create_menu: bool,
    create_menu_pos: egui::Pos2,
    new_node_title: String,
    new_node_type: NodeType,
    
    // 케이크 생성
    show_cake_creator: bool,
    new_cake_title: String,
    new_cake_theme: usize,
    
    // 편집 상태
    editing_node: Option<String>,
    edit_title: String,
    
    // 뷰 설정
    show_layers: bool,
    show_connections: bool,
    show_cake_titles: bool,
}

impl Cosmos3DApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let mut app = Self {
            cakes: Vec::new(),
            nodes: Vec::new(),
            connections: Vec::new(),
            viewport: Viewport::default(),
            selected_node: None,
            selected_cake: None,
            interaction_mode: InteractionMode::None,
            drag_start_pos: None,
            show_create_menu: false,
            create_menu_pos: egui::Pos2::ZERO,
            new_node_title: String::new(),
            new_node_type: NodeType::Root,
            show_cake_creator: false,
            new_cake_title: String::new(),
            new_cake_theme: 0,
            editing_node: None,
            edit_title: String::new(),
            show_layers: true,
            show_connections: true,
            show_cake_titles: true,
        };
        
        // 초기 데모 케이크들 생성
        app.create_demo_structures();
        app
    }

    fn create_demo_structures(&mut self) {
        // 첫 번째 케이크 (메인 프로젝트) - 1레이어로 시작
        let cake1 = CakeStructure::new(
            "main_project".to_string(),
            "Main Project".to_string(),
            egui::Pos2::new(300.0, 300.0)
        );
        self.cakes.push(cake1);
        
        // 두 번째 케이크 (연구 영역)
        let mut cake2 = CakeStructure::new(
            "research".to_string(), 
            "Research Area".to_string(),
            egui::Pos2::new(700.0, 200.0)
        );
        cake2.color_theme = 1; // 따뜻한 테마
        self.cakes.push(cake2);

        // 기본 루트 노드들만 생성 (1레이어)
        self.add_node_to_cake("Main Goal", NodeType::Root, "main_project", 0, 0.0);
        self.add_node_to_cake("Research", NodeType::Root, "research", 0, 0.0);
    }

    fn add_node_to_cake(&mut self, title: &str, node_type: NodeType, cake_id: &str, layer: usize, angle: f32) {
        let radius = 80.0 * (0.8_f32.powi(layer as i32)).max(0.3);
        let node = CakeNode {
            id: format!("{}_{}", title.replace(" ", "_"), self.nodes.len()),
            title: title.to_string(),
            layer,
            angle,
            radius,
            node_type,
            cake_id: cake_id.to_string(),
            selected: false,
        };
        
        // 케이크의 최대 레이어 업데이트 (강제로 확장)
        if let Some(cake) = self.cakes.iter_mut().find(|c| c.id == cake_id) {
            cake.expand_to_layer(layer);
            println!("Expanded cake '{}' to layer {}, max_layer now: {}", cake.title, layer, cake.max_layer);
        }
        
        self.nodes.push(node);
        println!("Added node '{}' to cake '{}' at layer {}", title, cake_id, layer);
    }

    fn add_connection(&mut self, from_title: &str, to_title: &str, connection_type: ConnectionType) {
        if let (Some(from_node), Some(to_node)) = (
            self.nodes.iter().find(|n| n.title == from_title),
            self.nodes.iter().find(|n| n.title == to_title)
        ) {
            self.connections.push(Connection {
                from_id: from_node.id.clone(),
                to_id: to_node.id.clone(),
                connection_type,
            });
        }
    }

    fn get_node_screen_pos(&self, node: &CakeNode, canvas_rect: egui::Rect) -> egui::Pos2 {
        if let Some(cake) = self.cakes.iter().find(|c| c.id == node.cake_id) {
            let layer_height = 60.0 * cake.size_scale;
            let y_offset = -(node.layer as f32 * layer_height);
            
            let angle = node.angle;
            let radius = node.radius * cake.size_scale;
            let x = cake.center.x + radius * angle.cos();
            let y = cake.center.y + y_offset + radius * angle.sin() * 0.3;
            
            let world_pos = egui::Pos2::new(x, y);
            self.viewport.world_to_screen(world_pos, canvas_rect)
        } else {
            egui::Pos2::ZERO
        }
    }

    fn find_node_at_pos(&self, pos: egui::Pos2, canvas_rect: egui::Rect) -> Option<&CakeNode> {
        for node in &self.nodes {
            let node_pos = self.get_node_screen_pos(node, canvas_rect);
            let node_size = (15.0 + node.layer as f32 * 3.0) * self.viewport.zoom;
            
            if pos.distance(node_pos) <= node_size + 5.0 {
                return Some(node);
            }
        }
        None
    }

    fn find_cake_at_pos(&self, pos: egui::Pos2, canvas_rect: egui::Rect) -> Option<&CakeStructure> {
        let world_pos = self.viewport.screen_to_world(pos, canvas_rect);
        
        for cake in &self.cakes {
            let distance = world_pos.distance(cake.center);
            let cake_radius = 120.0 * cake.size_scale;
            
            if distance <= cake_radius {
                return Some(cake);
            }
        }
        None
    }

    fn create_new_cake(&mut self, pos: egui::Pos2, canvas_rect: egui::Rect) {
        let world_pos = self.viewport.screen_to_world(pos, canvas_rect);
        let cake_id = format!("cake_{}", self.cakes.len());
        let mut new_cake = CakeStructure::new(
            cake_id.clone(),
            self.new_cake_title.clone(),
            world_pos
        );
        new_cake.color_theme = self.new_cake_theme;
        
        self.cakes.push(new_cake);
        
        // 새 케이크에 기본 루트 노드 추가 (1레이어)
        self.add_node_to_cake("Root", NodeType::Root, &cake_id, 0, 0.0);
    }
}

impl eframe::App for Cosmos3DApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // 메인 다차원 케이크 뷰
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("🎂 Multi-Dimensional Cake Graph");
            
            // 컨트롤 패널
            ui.horizontal(|ui| {
                ui.checkbox(&mut self.show_layers, "Show Layers");
                ui.checkbox(&mut self.show_connections, "Show Connections");
                ui.checkbox(&mut self.show_cake_titles, "Show Cake Titles");
                
                ui.separator();
                
                if ui.button("🎂 New Cake").clicked() {
                    // 빠른 생성: 기본 케이크 즉시 생성
                    let cake_id = format!("cake_{}", self.cakes.len());
                    let center = egui::Pos2::new(300.0 + self.cakes.len() as f32 * 300.0, 300.0);
                    let mut new_cake = CakeStructure::new(
                        cake_id.clone(),
                        format!("Cake {}", self.cakes.len() + 1),
                        center
                    );
                    new_cake.color_theme = self.cakes.len() % 4;
                    self.cakes.push(new_cake);
                    
                    // 기본 루트 노드 추가
                    self.add_node_to_cake("Root", NodeType::Root, &cake_id, 0, 0.0);
                    println!("Quick-created new cake: '{}'", cake_id);
                }
                
                if ui.button("🎂 Custom Cake").clicked() {
                    self.show_cake_creator = true;
                }
                
                if ui.button("🌌 Add Root (Layer 1)").clicked() {
                    self.new_node_type = NodeType::Root;
                    self.show_create_menu = true;
                }
                if ui.button("💭 Add Concept (Layer 2)").clicked() {
                    self.new_node_type = NodeType::Concept;
                    self.show_create_menu = true;
                }
                if ui.button("📋 Add Task (Layer 3)").clicked() {
                    self.new_node_type = NodeType::Task;
                    self.show_create_menu = true;
                }
                if ui.button("📝 Add Note (Layer 4)").clicked() {
                    self.new_node_type = NodeType::Note;
                    self.show_create_menu = true;
                }
                
                ui.separator();
                
                // 뷰포트 컨트롤
                ui.label(format!("Zoom: {:.1}x", self.viewport.zoom));
                if ui.button("Reset View").clicked() {
                    self.viewport = Viewport::default();
                }
                
                ui.separator();
                
                // 케이크 상태 정보
                ui.label(format!("Cakes: {}", self.cakes.len()));
                ui.label(format!("Nodes: {}", self.nodes.len()));
                if let Some(selected_cake_id) = &self.selected_cake {
                    if let Some(cake) = self.cakes.iter().find(|c| c.id == *selected_cake_id) {
                        ui.label(format!("Selected: '{}' (Layers: {})", cake.title, cake.max_layer + 1));
                    }
                }
            });

            ui.separator();

            // 3D 뷰 영역 (팬/줌 지원)
            let (response, painter) = ui.allocate_painter(
                ui.available_size(),
                egui::Sense::click_and_drag(),
            );

            let canvas_rect = response.rect;

            // 줌 처리 (두 손가락 스크롤/트랙패드)
            if response.hovered() {
                let scroll_delta = ui.input(|i| i.scroll_delta);
                if scroll_delta.y != 0.0 {
                    let zoom_delta = 1.0 + scroll_delta.y * 0.001;
                    self.viewport.zoom = (self.viewport.zoom * zoom_delta).clamp(0.1, 5.0);
                }
            }

            // 마우스 인터랙션 처리
            if response.clicked() {
                let click_pos = response.interact_pointer_pos().unwrap_or_default();
                
                // 노드 클릭 우선 확인
                let clicked_node_id = if let Some(clicked_node) = self.find_node_at_pos(click_pos, canvas_rect) {
                    Some(clicked_node.id.clone())
                } else {
                    None
                };
                
                if let Some(node_id) = clicked_node_id {
                    self.selected_node = Some(node_id.clone());
                    
                    // 더블클릭으로 편집 모드
                    if response.double_clicked() {
                        if let Some(node) = self.nodes.iter().find(|n| n.id == node_id) {
                            self.editing_node = Some(node_id);
                            self.edit_title = node.title.clone();
                        }
                    }
                } else {
                    // 케이크 클릭 확인
                    if let Some(clicked_cake) = self.find_cake_at_pos(click_pos, canvas_rect) {
                        self.selected_cake = Some(clicked_cake.id.clone());
                    } else {
                        self.selected_node = None;
                        self.selected_cake = None;
                    }
                }
            }

            // 우클릭 메뉴
            if response.secondary_clicked() {
                if let Some(pos) = response.interact_pointer_pos() {
                    self.create_menu_pos = pos;
                    self.show_create_menu = true;
                }
            }

            // 드래그 시작 처리
            if response.drag_started() {
                if let Some(drag_pos) = response.interact_pointer_pos() {
                    self.drag_start_pos = Some(drag_pos);
                    
                    // 노드에서 드래그 시작 = 연결 생성 모드
                    if let Some(node) = self.find_node_at_pos(drag_pos, canvas_rect) {
                        self.interaction_mode = InteractionMode::CreatingConnection(node.id.clone());
                    }
                    // 케이크에서 드래그 시작 = 케이크 이동
                    else if let Some(cake) = self.find_cake_at_pos(drag_pos, canvas_rect) {
                        self.interaction_mode = InteractionMode::DraggingCake(cake.id.clone());
                    }
                    // 빈 공간에서 드래그 = 팬 모드 (두 손가락 또는 일반 드래그)
                    else {
                        self.interaction_mode = InteractionMode::PanningView;
                    }
                }
            }

            // 드래그 처리
            if response.dragged() {
                let drag_pos = response.interact_pointer_pos().unwrap_or_default();
                let drag_delta = if let Some(start_pos) = self.drag_start_pos {
                    drag_pos - start_pos
                } else {
                    egui::Vec2::ZERO
                };
                
                match &self.interaction_mode {
                    InteractionMode::CreatingConnection(_) => {
                        // 연결 생성 중 - 선 그리기는 렌더링에서 처리
                    }
                    InteractionMode::DraggingCake(cake_id) => {
                        // 케이크 전체 이동
                        if let Some(cake) = self.cakes.iter_mut().find(|c| c.id == *cake_id) {
                            let world_delta = drag_delta / self.viewport.zoom;
                            cake.center += world_delta;
                        }
                        self.drag_start_pos = Some(drag_pos);
                    }
                    InteractionMode::PanningView => {
                        // 뷰 팬 (두 손가락 스크롤 효과)
                        self.viewport.offset -= drag_delta / self.viewport.zoom;
                        self.drag_start_pos = Some(drag_pos);
                    }
                    _ => {}
                }
            }

            // 드래그 종료
            if response.drag_released() {
                if let InteractionMode::CreatingConnection(from_id) = &self.interaction_mode {
                    if let Some(pos) = response.interact_pointer_pos() {
                        if let Some(target_node) = self.find_node_at_pos(pos, canvas_rect) {
                            // 연결 타입 결정 (같은 케이크인지 다른 케이크인지)
                            let from_cake = self.nodes.iter().find(|n| n.id == *from_id).map(|n| &n.cake_id);
                            let to_cake = Some(&target_node.cake_id);
                            
                            let connection_type = if from_cake == to_cake {
                                ConnectionType::IntraCake
                            } else {
                                ConnectionType::InterCake
                            };
                            
                            // 중복 연결 방지
                            let connection_exists = self.connections.iter().any(|c| {
                                (c.from_id == *from_id && c.to_id == target_node.id) ||
                                (c.from_id == target_node.id && c.to_id == *from_id)
                            });
                            
                            if !connection_exists && *from_id != target_node.id {
                                self.connections.push(Connection {
                                    from_id: from_id.clone(),
                                    to_id: target_node.id.clone(),
                                    connection_type,
                                });
                            }
                        }
                    }
                }
                self.interaction_mode = InteractionMode::None;
                self.drag_start_pos = None;
            }

            // 케이크들 그리기 (동적 레이어)
            for cake in &self.cakes {
                let is_selected = self.selected_cake.as_ref() == Some(&cake.id);
                
                // 케이크 층들 그리기 (동적으로 확장된 레이어까지)
                if self.show_layers {
                    for layer in 0..=cake.max_layer {
                        let layer_radius = ((80.0 - layer as f32 * 15.0) * cake.size_scale).max(20.0);
                        let y_offset = -(layer as f32 * 60.0 * cake.size_scale);
                        let world_center = egui::Pos2::new(cake.center.x, cake.center.y + y_offset);
                        let screen_center = self.viewport.world_to_screen(world_center, canvas_rect);
                        let screen_radius = layer_radius * self.viewport.zoom;
                        
                        painter.circle_filled(
                            screen_center,
                            screen_radius,
                            cake.get_layer_color(layer),
                        );
                        
                        if is_selected {
                            painter.circle_stroke(
                                screen_center,
                                screen_radius,
                                egui::Stroke::new(2.0 * self.viewport.zoom, egui::Color32::YELLOW),
                            );
                        }
                    }
                }
                
                // 케이크 제목
                if self.show_cake_titles {
                    let title_world_pos = cake.center + egui::Vec2::new(0.0, 100.0 * cake.size_scale);
                    let title_screen_pos = self.viewport.world_to_screen(title_world_pos, canvas_rect);
                    
                    painter.text(
                        title_screen_pos,
                        egui::Align2::CENTER_CENTER,
                        &cake.title,
                        egui::FontId::proportional(16.0 * self.viewport.zoom),
                        if is_selected { egui::Color32::YELLOW } else { egui::Color32::WHITE },
                    );
                }
            }

            // 연결선 그리기
            if self.show_connections {
                for connection in &self.connections {
                    if let (Some(from_node), Some(to_node)) = (
                        self.nodes.iter().find(|n| n.id == connection.from_id),
                        self.nodes.iter().find(|n| n.id == connection.to_id)
                    ) {
                        let from_pos = self.get_node_screen_pos(from_node, canvas_rect);
                        let to_pos = self.get_node_screen_pos(to_node, canvas_rect);
                        
                        let (color, width) = match connection.connection_type {
                            ConnectionType::IntraCake => (egui::Color32::from_rgba_unmultiplied(255, 255, 255, 150), 2.0),
                            ConnectionType::InterCake => (egui::Color32::from_rgba_unmultiplied(255, 100, 100, 200), 3.0),
                        };
                        
                        painter.line_segment([from_pos, to_pos], egui::Stroke::new(width * self.viewport.zoom, color));
                    }
                }
            }

            // 연결 생성 중인 경우 임시 선 그리기
            if let InteractionMode::CreatingConnection(from_id) = &self.interaction_mode {
                if let Some(from_node) = self.nodes.iter().find(|n| n.id == *from_id) {
                    if let Some(mouse_pos) = response.interact_pointer_pos() {
                        let from_pos = self.get_node_screen_pos(from_node, canvas_rect);
                        painter.line_segment(
                            [from_pos, mouse_pos],
                            egui::Stroke::new(3.0 * self.viewport.zoom, egui::Color32::YELLOW),
                        );
                    }
                }
            }

            // 노드들 그리기
            for node in &self.nodes {
                let node_pos = self.get_node_screen_pos(node, canvas_rect);
                let node_size = (15.0 + node.layer as f32 * 3.0) * self.viewport.zoom;
                
                let is_selected = self.selected_node.as_ref() == Some(&node.id);
                let border_color = if is_selected {
                    egui::Color32::YELLOW
                } else {
                    egui::Color32::WHITE
                };
                let border_width = if is_selected { 3.0 } else { 2.0 };
                
                painter.circle_filled(node_pos, node_size, node.node_type.color());
                painter.circle_stroke(node_pos, node_size, egui::Stroke::new(border_width * self.viewport.zoom, border_color));
                
                // 노드 아이콘
                painter.text(
                    node_pos,
                    egui::Align2::CENTER_CENTER,
                    node.node_type.emoji(),
                    egui::FontId::proportional(12.0 * self.viewport.zoom),
                    egui::Color32::WHITE,
                );
                
                // 노드 제목 (선택된 경우)
                if is_selected {
                    painter.text(
                        node_pos + egui::Vec2::new(0.0, node_size + 12.0 * self.viewport.zoom),
                        egui::Align2::CENTER_TOP,
                        &node.title,
                        egui::FontId::proportional(10.0 * self.viewport.zoom),
                        egui::Color32::WHITE,
                    );
                }
            }
        });

        // 케이크 생성 창
        if self.show_cake_creator {
            egui::Window::new("🎂 Create New Cake")
                .show(ctx, |ui| {
                    ui.horizontal(|ui| {
                        ui.label("Title:");
                        ui.text_edit_singleline(&mut self.new_cake_title);
                    });
                    
                    ui.horizontal(|ui| {
                        ui.label("Theme:");
                        egui::ComboBox::from_label("")
                            .selected_text(match self.new_cake_theme {
                                0 => "🌈 Default",
                                1 => "🔥 Warm",
                                2 => "❄️ Cool", 
                                _ => "🌱 Nature",
                            })
                            .show_ui(ui, |ui| {
                                ui.selectable_value(&mut self.new_cake_theme, 0, "🌈 Default");
                                ui.selectable_value(&mut self.new_cake_theme, 1, "🔥 Warm");
                                ui.selectable_value(&mut self.new_cake_theme, 2, "❄️ Cool");
                                ui.selectable_value(&mut self.new_cake_theme, 3, "🌱 Nature");
                            });
                    });
                    
                    ui.horizontal(|ui| {
                        if ui.button("Create").clicked() {
                            if !self.new_cake_title.is_empty() {
                                let center = egui::Pos2::new(400.0 + self.cakes.len() as f32 * 200.0, 300.0);
                                let canvas_rect = egui::Rect::from_center_size(egui::Pos2::new(400.0, 300.0), egui::Vec2::new(800.0, 600.0));
                                self.create_new_cake(center, canvas_rect);
                                self.new_cake_title.clear();
                            }
                            self.show_cake_creator = false;
                        }
                        
                        if ui.button("Cancel").clicked() {
                            self.show_cake_creator = false;
                        }
                    });
                });
        }

        // 노드 생성 메뉴
        if self.show_create_menu {
            egui::Window::new("Create Node")
                .fixed_pos(self.create_menu_pos)
                .show(ctx, |ui| {
                    ui.horizontal(|ui| {
                        ui.label("Title:");
                        ui.text_edit_singleline(&mut self.new_node_title);
                    });
                    
                    ui.horizontal(|ui| {
                        ui.label("Type:");
                        egui::ComboBox::from_label("")
                            .selected_text(format!("{} {:?}", self.new_node_type.emoji(), self.new_node_type))
                            .show_ui(ui, |ui| {
                                ui.selectable_value(&mut self.new_node_type, NodeType::Root, "🌌 Root");
                                ui.selectable_value(&mut self.new_node_type, NodeType::Concept, "💭 Concept");
                                ui.selectable_value(&mut self.new_node_type, NodeType::Task, "📋 Task");
                                ui.selectable_value(&mut self.new_node_type, NodeType::Note, "📝 Note");
                            });
                    });
                    
                    // 케이크 선택
                    if !self.cakes.is_empty() {
                        ui.horizontal(|ui| {
                            ui.label("Cake:");
                            let selected_cake_id = self.selected_cake.as_deref().unwrap_or(&self.cakes[0].id);
                            let selected_cake_title = self.cakes.iter()
                                .find(|c| c.id == selected_cake_id)
                                .map(|c| c.title.as_str())
                                .unwrap_or("Unknown");
                                
                            egui::ComboBox::from_label("")
                                .selected_text(selected_cake_title)
                                .show_ui(ui, |ui| {
                                    for cake in &self.cakes {
                                        ui.selectable_value(&mut self.selected_cake, Some(cake.id.clone()), &cake.title);
                                    }
                                });
                        });
                    }
                    
                    ui.horizontal(|ui| {
                        if ui.button("Create").clicked() {
                            if !self.new_node_title.is_empty() {
                                let cake_id = if let Some(selected) = &self.selected_cake {
                                    selected.clone()
                                } else if !self.cakes.is_empty() {
                                    self.cakes[0].id.clone()
                                } else {
                                    println!("No cakes available for node creation");
                                    self.show_create_menu = false;
                                    return; // 케이크가 없으면 생성하지 않음
                                };
                                
                                let layer = self.new_node_type.layer();
                                let angle = self.nodes.iter()
                                    .filter(|n| n.cake_id == cake_id && n.layer == layer)
                                    .count() as f32 * 1.2;
                                
                                let title = self.new_node_title.clone();
                                let node_type = self.new_node_type;
                                println!("Creating node '{}' of type {:?} at layer {} in cake '{}'", title, node_type, layer, cake_id);
                                self.add_node_to_cake(&title, node_type, &cake_id, layer, angle);
                                self.new_node_title.clear();
                            }
                            self.show_create_menu = false;
                        }
                        
                        if ui.button("Cancel").clicked() {
                            self.show_create_menu = false;
                        }
                    });
                });
        }

        // 노드 편집 창
        if let Some(editing_id) = &self.editing_node.clone() {
            egui::Window::new("Edit Node")
                .show(ctx, |ui| {
                    ui.horizontal(|ui| {
                        ui.label("Title:");
                        ui.text_edit_singleline(&mut self.edit_title);
                    });
                    
                    ui.horizontal(|ui| {
                        if ui.button("Save").clicked() {
                            if let Some(node) = self.nodes.iter_mut().find(|n| n.id == *editing_id) {
                                node.title = self.edit_title.clone();
                            }
                            self.editing_node = None;
                        }
                        
                        if ui.button("Delete").clicked() {
                            self.nodes.retain(|n| n.id != *editing_id);
                            self.connections.retain(|c| c.from_id != *editing_id && c.to_id != *editing_id);
                            self.editing_node = None;
                            self.selected_node = None;
                        }
                        
                        if ui.button("Cancel").clicked() {
                            self.editing_node = None;
                        }
                    });
                });
        }

        // 도움말 패널
        egui::Window::new("🎮 Enhanced Multi-Dimensional Controls")
            .default_pos(egui::Pos2::new(10.0, 10.0))
            .show(ctx, |ui| {
                ui.colored_label(egui::Color32::YELLOW, "🎂 Cake Operations:");
                ui.label("• Click cake: Select");
                ui.label("• Drag cake: Move entire structure");
                ui.label("• 'New Cake' button: Create cake");
                ui.label("• Dynamic layers: Auto-expand as needed");
                
                ui.separator();
                
                ui.colored_label(egui::Color32::LIGHT_BLUE, "🔵 Node Operations:");
                ui.label("• Click node: Select");
                ui.label("• Double-click: Edit");
                ui.label("• Drag from node: Create connection");
                ui.label("• Right-click: Create menu");
                
                ui.separator();
                
                ui.colored_label(egui::Color32::LIGHT_GREEN, "🔗 Connections:");
                ui.label("• Drag node to node: Connect");
                ui.label("• White lines: Intra-cake");
                ui.label("• Red lines: Inter-cake");
                
                ui.separator();
                
                ui.colored_label(egui::Color32::from_rgb(255, 165, 0), "🖱️ Navigation (MacBook):");
                ui.label("• Two-finger scroll: Pan view");
                ui.label("• Scroll wheel: Zoom in/out");
                ui.label("• Drag empty space: Pan");
                ui.label("• Reset View: Return to origin");
            });
    }
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1400.0, 900.0])
            .with_min_inner_size([1000.0, 700.0])
            .with_title("🎂 Enhanced Multi-Dimensional Cosmos Cake Graph"),
        vsync: false,
        multisampling: 0,
        renderer: eframe::Renderer::Glow,
        ..Default::default()
    };

    eframe::run_native(
        "🌌 Enhanced Multi-Dimensional Cosmos",
        options,
        Box::new(|cc| {
            cc.egui_ctx.set_visuals(egui::Visuals::dark());
            Box::new(Cosmos3DApp::new(cc))
        }),
    )
} 