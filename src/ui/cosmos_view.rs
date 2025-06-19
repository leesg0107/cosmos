use eframe::egui;
use rand::Rng;
use crate::celestial::{
    Graph, Node, NodeType, RelationType, Position2D, Position3D
};
use crate::storage::{Storage, UniverseInfo};
use super::{Renderer3D, Scene3D};

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
enum ViewMode {
    Mode3D,     // 3D 케이크 뷰
    Mode2D,     // 기존 2D 뷰
    ModeHybrid, // 2D/3D 혼합 뷰
}

#[derive(Debug, PartialEq)]
enum DragMode {
    None,
    ViewPan,           // 빈 공간 왼쪽 클릭 드래그로 화면 이동
    CreateNode,        // 오른쪽 클릭 드래그로 새 노드 생성
    MoveNode,          // 왼쪽 클릭 드래그로 노드 이동
    Camera3D,          // 3D 카메라 회전
}

pub struct CosmosView {
    graph: Graph,
    storage: Storage,
    
    // 3D 시스템
    renderer_3d: Renderer3D,
    scene_3d: Scene3D,
    view_mode: ViewMode,
    
    // 기존 2D 시스템 (호환성)
    dragging: Option<(String, egui::Pos2)>,
    view_offset: egui::Vec2,
    
    // UI 상태
    selected_node: Option<String>,
    show_node_creator: bool,
    show_evolution_view: bool,
    new_node_title: String,
    new_node_type: Option<NodeType>,
    hover_pos: Option<egui::Pos2>,
    node_creation: Option<NodeCreationInfo>,
    show_node_content: bool,
    editing_node: bool,
    drag_mode: DragMode,
    
    // 애니메이션과 효과
    transition_progress: f32,
    auto_arrange_cake: bool,
    show_layer_info: bool,
    
    // 메타데이터
    current_universe_id: Option<String>,
    last_save_time: std::time::Instant,
}

impl CosmosView {
    pub fn new() -> Self {
        let mut graph = Graph::new();
        let mut scene_3d = Scene3D::new();
        
        // 초기 데모 노드들 생성 (케이크 구조 예시)
        let root_id = graph.create_node(
            "🌌 Cosmos Root".to_string(),
            NodeType::Root,
            Position2D::new(0.0, 0.0)
        );

        // 2단계 노드들
        let concept1_id = graph.create_node(
            "💭 Main Concept".to_string(),
            NodeType::Concept,
            Position2D::new(100.0, -100.0)
        );
        graph.create_relation(&root_id, &concept1_id, RelationType::Parent);

        let concept2_id = graph.create_node(
            "🎯 Core Idea".to_string(),
            NodeType::Concept,
            Position2D::new(-100.0, -100.0)
        );
        graph.create_relation(&root_id, &concept2_id, RelationType::Parent);

        // 3단계 노드들
        let task1_id = graph.create_node(
            "📋 Task A".to_string(),
            NodeType::Task,
            Position2D::new(150.0, -200.0)
        );
        graph.create_relation(&concept1_id, &task1_id, RelationType::Parent);

        let task2_id = graph.create_node(
            "📝 Task B".to_string(),
            NodeType::Task,
            Position2D::new(50.0, -200.0)
        );
        graph.create_relation(&concept1_id, &task2_id, RelationType::Parent);

        let note1_id = graph.create_node(
            "📓 Note 1".to_string(),
            NodeType::Note,
            Position2D::new(-50.0, -200.0)
        );
        graph.create_relation(&concept2_id, &note1_id, RelationType::Parent);

        // 케이크 구조로 자동 배치
        scene_3d.arrange_graph_as_cake(&mut graph);

        Self {
            graph,
            storage: Storage::new(),
            renderer_3d: Renderer3D::new(),
            scene_3d,
            view_mode: ViewMode::Mode3D, // 기본적으로 3D 모드
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
            transition_progress: 1.0,
            auto_arrange_cake: true,
            show_layer_info: true,
            current_universe_id: Some(uuid::Uuid::new_v4().to_string()),
            last_save_time: std::time::Instant::now(),
        }
    }

    pub fn show(&mut self, ui: &mut egui::Ui) {
        // 상단 메뉴바
        self.show_menu_bar(ui);

        // 메인 콘텐츠 영역
        egui::CentralPanel::default().show(ui.ctx(), |ui| {
            match self.view_mode {
                ViewMode::Mode3D => self.show_3d_view(ui),
                ViewMode::Mode2D => self.show_2d_view(ui),
                ViewMode::ModeHybrid => self.show_hybrid_view(ui),
            }
        });

        // 사이드바 (layer 정보, 컨트롤 등)
        if self.show_layer_info {
            self.show_layer_sidebar(ui);
        }

        // 노드 생성 UI
        if self.show_node_creator {
            self.show_node_creator_ui(ui);
        }

        // 노드 내용 창
        if self.show_node_content {
            self.show_node_content_window(ui);
        }
    }

    fn show_menu_bar(&mut self, ui: &mut egui::Ui) {
        egui::TopBottomPanel::top("menu_bar").show(ui.ctx(), |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("🌌 BigBang", |ui| {
                    if ui.button("New Universe").clicked() {
                        self.create_new_universe();
                    }
                    ui.separator();
                    if ui.button("Arrange as Cake 🎂").clicked() {
                        self.scene_3d.arrange_graph_as_cake(&mut self.graph);
                    }
                    if ui.button("Rebalance Layers").clicked() {
                        self.scene_3d.rebalance_cake(&mut self.graph);
                    }
                });

                ui.menu_button("🎭 ViewMode", |ui| {
                    ui.radio_value(&mut self.view_mode, ViewMode::Mode3D, "🎂 3D Cake");
                    ui.radio_value(&mut self.view_mode, ViewMode::Mode2D, "📋 2D Traditional");
                    ui.radio_value(&mut self.view_mode, ViewMode::ModeHybrid, "🔀 Hybrid");
                });

                ui.menu_button("⏰ TimeLog", |ui| {
                    for universe in self.get_saved_universes() {
                        if ui.button(&universe.title).clicked() {
                            self.load_universe(&universe.id);
                        }
                    }
                });

                ui.menu_button("🛠️ Tools", |ui| {
                    ui.checkbox(&mut self.auto_arrange_cake, "Auto Arrange");
                    ui.checkbox(&mut self.show_layer_info, "Layer Info");
                    ui.separator();
                    
                    if ui.button("Add Random Node").clicked() {
                        self.add_random_demo_node();
                    }
                });

                if ui.button("🕳️ BlackHole").clicked() {
                    std::process::exit(0);
                }
            });
        });
    }

    fn show_3d_view(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.label("🎂 3D Cake Graph View");
            ui.separator();
            
            // 3D 컨트롤
            self.renderer_3d.show_controls(ui);
        });

        ui.separator();

        // 3D 장면 렌더링
        let nodes: Vec<_> = self.graph.get_nodes().cloned().collect();
        let response = self.renderer_3d.render_scene(ui, &nodes, self.selected_node.as_ref());

        // 3D 뷰에서의 상호작용 처리
        self.handle_3d_interactions(&response, &nodes);

        // 자동 케이크 재배치
        if self.auto_arrange_cake && nodes.len() > 0 {
            // 노드가 추가/삭제되었을 때 자동으로 재배치
            let current_node_count = nodes.len();
            static mut LAST_NODE_COUNT: usize = 0;
            
            unsafe {
                if current_node_count != LAST_NODE_COUNT {
                    self.scene_3d.rebalance_cake(&mut self.graph);
                    LAST_NODE_COUNT = current_node_count;
                }
            }
        }
    }

    fn show_2d_view(&mut self, ui: &mut egui::Ui) {
        ui.label("📋 Traditional 2D View");
        ui.separator();
        
        // 기존 2D 렌더링 로직
        let (response, painter) = ui.allocate_painter(
            ui.available_size(),
            egui::Sense::click_and_drag()
        );

        self.draw_background(&painter, &response);
        self.draw_relations(&painter, &response);
        self.draw_nodes_2d(ui, &response, &painter);
        self.handle_2d_interactions(&response, ui, &painter);
    }

    fn show_hybrid_view(&mut self, ui: &mut egui::Ui) {
        ui.label("🔀 Hybrid 2D/3D View");
        ui.separator();

        // 화면을 반반 나누어서 2D와 3D를 동시에 보여줌
        ui.columns(2, |columns| {
            columns[0].vertical(|ui| {
                ui.label("2D View");
                self.show_2d_view(ui);
            });

            columns[1].vertical(|ui| {
                ui.label("3D Cake View");
                self.show_3d_view(ui);
            });
        });
    }

    fn show_layer_sidebar(&mut self, ui: &mut egui::Ui) {
        egui::SidePanel::right("layer_info").show(ui.ctx(), |ui| {
            ui.heading("🎂 Cake Layers");
            ui.separator();

            let stats = self.scene_3d.get_layer_stats();
            
            for (layer, node_count, max_nodes) in stats {
                let layer_color = self.scene_3d.get_layer_color(layer);
                let color = egui::Color32::from_rgba_premultiplied(
                    (layer_color[0] * 255.0) as u8,
                    (layer_color[1] * 255.0) as u8,
                    (layer_color[2] * 255.0) as u8,
                    255,
                );

                ui.horizontal(|ui| {
                    ui.colored_label(color, format!("L{}", layer));
                    ui.label(format!("{}/{}", node_count, max_nodes));
                    
                    if ui.small_button("Focus").clicked() {
                        self.renderer_3d.camera_mut().focus_on_layer(layer);
                    }
                });

                // 진행률 바
                let progress = node_count as f32 / max_nodes as f32;
                let progress_bar = egui::ProgressBar::new(progress)
                    .fill(color)
                    .animate(true);
                ui.add(progress_bar);

                ui.separator();
            }

            ui.heading("🎮 Controls");
            ui.label("Left Drag: Rotate Camera");
            ui.label("Right Drag: Pan View");
            ui.label("Scroll: Zoom In/Out");
            ui.label("Click Node: Select");
            ui.label("Double Click: Edit");
        });
    }

    fn handle_3d_interactions(&mut self, response: &egui::Response, nodes: &[Node]) {
        // 노드 클릭 감지
        if response.clicked() {
            if let Some(clicked_node) = self.find_clicked_node_3d(response, nodes) {
                self.selected_node = Some(clicked_node.id.clone());
            } else {
                self.selected_node = None;
            }
        }

        // 더블클릭으로 노드 편집
        if response.double_clicked() {
            if let Some(clicked_node) = self.find_clicked_node_3d(response, nodes) {
                self.selected_node = Some(clicked_node.id.clone());
                self.show_node_content = true;
            }
        }

        // 우클릭 메뉴
        if response.secondary_clicked() {
            if let Some(clicked_node) = self.find_clicked_node_3d(response, nodes) {
                // 노드 우클릭 메뉴
                self.show_node_context_menu(&clicked_node.id, response.interact_pointer_pos());
            } else {
                // 빈 공간 우클릭 - 새 노드 생성
                self.show_create_node_menu(response.interact_pointer_pos());
            }
        }
    }

    fn find_clicked_node_3d<'a>(&self, response: &egui::Response, nodes: &'a [Node]) -> Option<&'a Node> {
        if let Some(click_pos) = response.interact_pointer_pos() {
            let camera = self.renderer_3d.camera();
            let view_matrix = camera.view_matrix();
            let projection_matrix = camera.projection_matrix();
            let viewport = glam::Vec2::new(response.rect.width(), response.rect.height());

            for node in nodes {
                let screen_pos = node.position_3d.project_to_screen(
                    &view_matrix,
                    &projection_matrix,
                    viewport,
                );

                let node_screen_pos = egui::pos2(screen_pos.x, screen_pos.y);
                let distance = click_pos.distance(node_screen_pos);
                let node_size = node.get_layer_size();

                if distance <= node_size {
                    return Some(node);
                }
            }
        }
        None
    }

    fn add_random_demo_node(&mut self) {
        let node_types = [NodeType::Concept, NodeType::Task, NodeType::Note];
        let names = ["Idea", "Thought", "Plan", "Goal", "Memory", "Insight"];
        let emojis = ["💡", "🎯", "📝", "🌟", "🔍", "💭"];
        
        let mut rng = rand::thread_rng();
        let node_type = node_types[rng.gen_range(0..node_types.len())].clone();
        let name = names[rng.gen_range(0..names.len())];
        let emoji = emojis[rng.gen_range(0..emojis.len())];
        let title = format!("{} {}", emoji, name);

        let node_id = self.graph.create_node(
            title,
            node_type,
            Position2D::new(0.0, 0.0)
        );

        // 부모 노드 선택 (랜덤으로)
        let nodes: Vec<_> = self.graph.get_nodes().cloned().collect();
        if !nodes.is_empty() {
            let parent = &nodes[rng.gen_range(0..nodes.len())];
            self.graph.create_relation(&parent.id, &node_id, RelationType::Parent);
        }

        // 케이크 구조에 추가
        self.scene_3d.add_node_to_cake(&mut self.graph, &node_id, None);
    }

    // 기존 2D 메서드들 유지 (호환성)
    fn draw_background(&self, painter: &egui::Painter, response: &egui::Response) {
        painter.rect_filled(
            response.rect,
            0.0,
            egui::Color32::from_rgb(16, 16, 24)
        );
    }

    fn draw_relations(&mut self, painter: &egui::Painter, response: &egui::Response) {
        // 2D 관계선 그리기 로직
    }

    fn draw_nodes_2d(&mut self, ui: &mut egui::Ui, response: &egui::Response, painter: &egui::Painter) {
        // 2D 노드 그리기 로직
    }

    fn handle_2d_interactions(&mut self, response: &egui::Response, ui: &mut egui::Ui, painter: &egui::Painter) {
        // 2D 상호작용 처리
    }

    fn show_node_context_menu(&mut self, node_id: &str, pos: Option<egui::Pos2>) {
        // 노드 컨텍스트 메뉴
    }

    fn show_create_node_menu(&mut self, pos: Option<egui::Pos2>) {
        // 노드 생성 메뉴
        self.show_node_creator = true;
    }

    // 기존 메서드들도 유지
    fn show_node_creator_ui(&mut self, ui: &mut egui::Ui) {
        egui::Window::new("🎂 Create New Node")
            .default_width(300.0)
            .show(ui.ctx(), |ui| {
                ui.horizontal(|ui| {
                    ui.label("Title:");
                    ui.text_edit_singleline(&mut self.new_node_title);
                });

                ui.horizontal(|ui| {
                    ui.label("Type:");
                    let current_type = self.new_node_type.unwrap_or(NodeType::Concept);
                    egui::ComboBox::from_label("")
                        .selected_text(format!("{:?}", current_type))
                        .show_ui(ui, |ui| {
                            ui.selectable_value(&mut self.new_node_type, Some(NodeType::Root), "Root 🌌");
                            ui.selectable_value(&mut self.new_node_type, Some(NodeType::Concept), "Concept 💭");
                            ui.selectable_value(&mut self.new_node_type, Some(NodeType::Task), "Task 📋");
                            ui.selectable_value(&mut self.new_node_type, Some(NodeType::Note), "Note 📝");
                        });
                });

                ui.horizontal(|ui| {
                    if ui.button("Create").clicked() {
                        if let Some(node_type) = self.new_node_type {
                            let node_id = self.graph.create_node(
                                self.new_node_title.clone(),
                                node_type,
                                Position2D::new(0.0, 0.0)
                            );
                            
                            self.scene_3d.add_node_to_cake(&mut self.graph, &node_id, None);
                            
                            self.new_node_title.clear();
                            self.new_node_type = None;
                            self.show_node_creator = false;
                        }
                    }
                    
                    if ui.button("Cancel").clicked() {
                        self.show_node_creator = false;
                    }
                });
            });
    }

    fn show_node_content_window(&mut self, ui: &mut egui::Ui) {
        if let Some(node_id) = &self.selected_node.clone() {
            if let Some(node) = self.graph.get_node(node_id) {
                egui::Window::new(format!("📝 {}", node.title))
                    .default_width(400.0)
                    .default_height(300.0)
                    .show(ui.ctx(), |ui| {
                        ui.label(format!("Layer: {}", node.layer));
                        ui.label(format!("Type: {:?}", node.node_type));
                        ui.separator();
                        
                        if let Some(desc) = &node.description {
                            ui.label("Description:");
                            ui.text_edit_multiline(&mut desc.clone());
                        }
                        
                        ui.horizontal(|ui| {
                            if ui.button("Close").clicked() {
                                self.show_node_content = false;
                            }
                        });
                    });
            }
        }
    }

    // 기존 메서드들 유지
    fn create_new_universe(&mut self) {
        self.graph = Graph::new();
        self.scene_3d = Scene3D::new();
        self.current_universe_id = Some(uuid::Uuid::new_v4().to_string());
    }

    fn get_saved_universes(&self) -> Vec<UniverseInfo> {
        self.storage.list_universes()
    }

    fn load_universe(&mut self, universe_id: &str) {
        if let Ok(graph) = self.storage.load_universe(universe_id) {
            self.graph = graph;
            self.scene_3d.arrange_graph_as_cake(&mut self.graph);
            self.current_universe_id = Some(universe_id.to_string());
        }
    }
} 