use eframe::egui;
use crate::celestial::{Node, Graph};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum DragMode {
    None,
    ViewPan,           // 화면 이동
    CreateNode,        // 새 노드 생성
    MoveNode,         // 노드 이동
    DrawConnection,   // 연결선 그리기
}

pub struct DragHandler {
    pub drag_mode: DragMode,
    pub dragging: Option<(String, egui::Pos2)>,
}

impl DragHandler {
    pub fn new() -> Self {
        Self {
            drag_mode: DragMode::None,
            dragging: None,
        }
    }

    pub fn handle_drag(&mut self, response: &egui::Response, ui: &mut egui::Ui, graph: &mut Graph) -> Option<DragAction> {
        let pos = response.hover_pos()?;

        // 1. 더블 클릭 처리 (최우선)
        if response.double_clicked() {
            if let Some(node) = graph.get_nodes().find(|n| self.is_pos_in_node(n, pos)) {
                return Some(DragAction::NodeDoubleClicked(node.id.clone()));
            } else {
                return Some(DragAction::RequestCreateNode(pos));
            }
        }

        // 2. 드래그 시작 처리
        if response.drag_started() {
            if ui.input(|i| i.pointer.secondary_down()) {
                // 오른쪽 드래그: 연결선 그리기
                if let Some(node) = graph.get_nodes().find(|n| self.is_pos_in_node(n, pos)) {
                    self.drag_mode = DragMode::DrawConnection;
                    self.dragging = Some((node.id.clone(), pos));
                    return Some(DragAction::StartDrawConnection(node.id.clone(), pos));
                }
            } else if ui.input(|i| i.pointer.primary_down()) {
                // 왼쪽 드래그: 노드 이동 또는 화면 이동
                if let Some(node) = graph.get_nodes().find(|n| self.is_pos_in_node(n, pos)) {
                    self.drag_mode = DragMode::MoveNode;
                    self.dragging = Some((node.id.clone(), pos));
                    return Some(DragAction::StartMoveNode(node.id.clone()));
                } else {
                    self.drag_mode = DragMode::ViewPan;
                    return Some(DragAction::StartViewPan);
                }
            }
        }

        // 3. 드래그 중 처리
        if response.dragged() {
            let current_pos = response.hover_pos()?;
            match self.drag_mode {
                DragMode::ViewPan => {
                    return Some(DragAction::ViewPan(response.drag_delta()));
                }
                DragMode::DrawConnection => {
                    if let Some((source_id, _)) = &self.dragging {
                        return Some(DragAction::DrawingConnection {
                            source_id: source_id.clone(),
                            current_pos,
                        });
                    }
                }
                DragMode::MoveNode => {
                    if let Some((node_id, _)) = &self.dragging {
                        return Some(DragAction::Dragging {
                            node_id: node_id.clone(),
                            mode: self.drag_mode,
                            current_pos,
                        });
                    }
                }
                _ => {}
            }
        }

        // 4. 드래그 종료 처리
        if response.drag_released() {
            let end_pos = response.hover_pos()?;
            let result = match self.drag_mode {
                DragMode::DrawConnection => {
                    if let Some((source_id, _)) = &self.dragging {
                        Some(DragAction::CreateChildNode {
                            parent_id: source_id.clone(),
                            position: end_pos,
                        })
                    } else {
                        None
                    }
                }
                DragMode::MoveNode => {
                    if let Some((node_id, _)) = &self.dragging {
                        Some(DragAction::EndMoveNode {
                            node_id: node_id.clone(),
                            end_pos,
                        })
                    } else {
                        None
                    }
                }
                _ => None,
            };
            self.drag_mode = DragMode::None;
            self.dragging = None;
            return result;
        }

        // 5. 일반 클릭 처리 (드래그가 아닌 경우)
        if response.clicked() {
            if let Some(node) = graph.get_nodes().find(|n| self.is_pos_in_node(n, pos)) {
                return Some(DragAction::SelectNode(node.id.clone()));
            } else {
                return Some(DragAction::Deselect);
            }
        }

        None
    }

    fn is_pos_in_node(&self, node: &Node, pos: egui::Pos2) -> bool {
        let node_pos = egui::pos2(node.position.x, node.position.y);
        let distance = node_pos.distance(pos);
        distance < 20.0  // 노드 크기에 따라 조정 필요
    }
}

#[derive(Debug)]
pub enum DragAction {
    SelectNode(String),  // 노드 선택
    Deselect,           // 선택 해제
    StartViewPan,
    StartDrawConnection(String, egui::Pos2),
    StartMoveNode(String),
    ViewPan(egui::Vec2),
    DrawingConnection {
        source_id: String,
        current_pos: egui::Pos2,
    },
    Dragging {
        node_id: String,
        mode: DragMode,
        current_pos: egui::Pos2,
    },
    EndDrawConnection {
        source_id: String,
        end_pos: egui::Pos2,
    },
    EndMoveNode {
        node_id: String,
        end_pos: egui::Pos2,
    },
    RequestCreateNode(egui::Pos2),  // 새 노드 생성 요청 (더블 클릭)
    NodeDoubleClicked(String),  // 노드 더블클릭
    CreateChildNode {
        parent_id: String,
        position: egui::Pos2,
    },
} 