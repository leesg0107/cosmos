use crate::celestial::{Graph, Node, NodeType, Position3D, LayerPosition};
use rand::Rng;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Layer {
    pub level: usize,
    pub height: f32,
    pub radius: f32,
    pub node_count: usize,
    pub max_nodes: usize,
    pub color: [f32; 4],
}

impl Layer {
    pub fn new(level: usize, base_radius: f32, max_nodes: usize) -> Self {
        let height = level as f32 * 100.0;
        // 위로 갈수록 작아지는 케이크 구조
        let radius = base_radius * (0.8_f32).powi(level as i32).max(0.3);
        
        let color = match level {
            0 => [1.0, 0.9, 0.7, 1.0], // 골드
            1 => [0.8, 0.8, 1.0, 1.0], // 라이트 블루
            2 => [1.0, 0.8, 0.8, 1.0], // 라이트 핑크
            3 => [0.8, 1.0, 0.8, 1.0], // 라이트 그린
            _ => [0.9, 0.9, 0.9, 1.0], // 그레이
        };

        Self {
            level,
            height,
            radius,
            node_count: 0,
            max_nodes,
            color,
        }
    }

    pub fn has_space(&self) -> bool {
        self.node_count < self.max_nodes
    }

    pub fn get_next_angle(&self) -> f32 {
        if self.node_count == 0 {
            0.0
        } else {
            (self.node_count as f32) * (360.0 / self.max_nodes as f32)
        }
    }
}

pub struct Scene3D {
    pub layers: Vec<Layer>,
    pub max_layers: usize,
    pub base_radius: f32,
    node_positions: HashMap<String, LayerPosition>,
}

impl Scene3D {
    pub fn new() -> Self {
        let max_layers = 5;
        let base_radius = 200.0;
        let mut layers = Vec::new();

        // 케이크 층들 초기화
        for level in 0..max_layers {
            let max_nodes = match level {
                0 => 8,  // 최하위층이 가장 많은 노드
                1 => 6,
                2 => 4,
                3 => 3,
                _ => 2,
            };
            layers.push(Layer::new(level, base_radius, max_nodes));
        }

        Self {
            layers,
            max_layers,
            base_radius,
            node_positions: HashMap::new(),
        }
    }

    /// 그래프를 3D 케이크 구조로 재배치
    pub fn arrange_graph_as_cake(&mut self, graph: &mut Graph) {
        // 기존 위치 정보 초기화
        self.node_positions.clear();
        for layer in &mut self.layers {
            layer.node_count = 0;
        }

        let nodes: Vec<_> = graph.get_nodes().cloned().collect();
        
        // 1. Root 노드들을 최하위층에 배치
        let root_nodes: Vec<_> = nodes.iter()
            .filter(|n| n.node_type == NodeType::Root)
            .collect();

        for (i, root_node) in root_nodes.iter().enumerate() {
            self.place_node_in_layer(graph, &root_node.id, 0, i);
        }

        // 2. 계층적으로 자식 노드들 배치
        self.arrange_children_hierarchically(graph, &nodes);

        // 3. Evolution 노드들은 같은 층에 추가 배치
        self.arrange_evolution_nodes(graph, &nodes);

        // 4. 독립 노드들을 적절한 층에 배치
        self.arrange_orphan_nodes(graph, &nodes);
    }

    /// 노드를 특정 계층에 배치
    fn place_node_in_layer(&mut self, graph: &mut Graph, node_id: &str, layer_level: usize, position_in_layer: usize) {
        if layer_level >= self.layers.len() {
            return;
        }

        let layer = &mut self.layers[layer_level];
        
        if !layer.has_space() {
            // 다음 층으로 올려보내기
            if layer_level + 1 < self.max_layers {
                self.place_node_in_layer(graph, node_id, layer_level + 1, 0);
            }
            return;
        }

        let angle = if layer.max_nodes > 1 {
            (position_in_layer as f32 / layer.max_nodes as f32) * 360.0
        } else {
            0.0
        };

        // 약간의 랜덤성 추가 (더 자연스러운 배치)
        let mut rng = rand::thread_rng();
        let angle_variation = rng.gen_range(-15.0..15.0);
        let radius_variation = rng.gen_range(0.8..1.2);
        
        let final_angle = angle + angle_variation;
        let final_radius = layer.radius * radius_variation;

        let layer_position = LayerPosition::new(layer_level, final_radius, final_angle);
        
        // 그래프의 노드 업데이트
        if let Some(node) = graph.get_node_mut(node_id) {
            node.set_layer_position(layer_level, final_radius, final_angle);
        }

        self.node_positions.insert(node_id.to_string(), layer_position);
        layer.node_count += 1;
    }

    /// 자식 노드들을 계층적으로 배치
    fn arrange_children_hierarchically(&mut self, graph: &mut Graph, nodes: &[Node]) {
        let mut processed = std::collections::HashSet::new();
        
        // Root 노드들부터 시작해서 BFS로 배치
        let root_nodes: Vec<_> = nodes.iter()
            .filter(|n| n.node_type == NodeType::Root)
            .map(|n| n.id.clone())
            .collect();

        let mut queue = std::collections::VecDeque::new();
        
        for root_id in root_nodes {
            queue.push_back((root_id, 0));
        }

        while let Some((parent_id, parent_layer)) = queue.pop_front() {
            if processed.contains(&parent_id) {
                continue;
            }
            processed.insert(parent_id.clone());

            // 자식 노드들 찾기
            let children: Vec<_> = nodes.iter()
                .filter(|n| n.parent_id.as_ref() == Some(&parent_id))
                .collect();

            let child_layer = (parent_layer + 1).min(self.max_layers - 1);
            
            for (i, child) in children.iter().enumerate() {
                if !processed.contains(&child.id) {
                    self.place_node_in_layer(graph, &child.id, child_layer, i);
                    queue.push_back((child.id.clone(), child_layer));
                }
            }
        }
    }

    /// Evolution 노드들 배치
    fn arrange_evolution_nodes(&mut self, graph: &mut Graph, nodes: &[Node]) {
        let evolution_nodes: Vec<_> = nodes.iter()
            .filter(|n| n.node_type == NodeType::Evolution)
            .collect();

        for evo_node in evolution_nodes {
            // 원본 노드 찾기 (임시로 같은 이름 prefix 사용)
            let base_name = evo_node.title.replace(" (Evolution)", "");
            if let Some(base_node) = nodes.iter().find(|n| n.title == base_name) {
                let layer = base_node.layer;
                let position = self.layers[layer].node_count;
                self.place_node_in_layer(graph, &evo_node.id, layer, position);
            } else {
                // 기본적으로 마지막 층에 배치
                let last_layer = self.max_layers - 1;
                let position = self.layers[last_layer].node_count;
                self.place_node_in_layer(graph, &evo_node.id, last_layer, position);
            }
        }
    }

    /// 부모가 없는 독립 노드들 배치
    fn arrange_orphan_nodes(&mut self, graph: &mut Graph, nodes: &[Node]) {
        let orphan_nodes: Vec<_> = nodes.iter()
            .filter(|n| {
                n.parent_id.is_none() && 
                n.node_type != NodeType::Root &&
                !self.node_positions.contains_key(&n.id)
            })
            .collect();

        for (i, orphan) in orphan_nodes.iter().enumerate() {
            // 타입에 따라 적절한 층 선택
            let preferred_layer = match orphan.node_type {
                NodeType::Concept => 1,
                NodeType::Task => 2,
                NodeType::Note => 3,
                _ => 2,
            };

            self.place_node_in_layer(graph, &orphan.id, preferred_layer, i);
        }
    }

    /// 새 노드를 케이크 구조에 추가
    pub fn add_node_to_cake(&mut self, graph: &mut Graph, node_id: &str, parent_id: Option<&str>) {
        let target_layer = if let Some(parent_id) = parent_id {
            // 부모 노드의 다음 층
            if let Some(parent_pos) = self.node_positions.get(parent_id) {
                (parent_pos.layer + 1).min(self.max_layers - 1)
            } else {
                1
            }
        } else {
            0 // Root 노드는 최하위층
        };

        let position = self.layers[target_layer].node_count;
        self.place_node_in_layer(graph, node_id, target_layer, position);
    }

    /// 노드를 다른 층으로 이동
    pub fn move_node_to_layer(&mut self, graph: &mut Graph, node_id: &str, target_layer: usize) {
        if target_layer >= self.max_layers {
            return;
        }

        // 현재 위치에서 제거
        if let Some(current_pos) = self.node_positions.get(node_id) {
            let current_layer = current_pos.layer;
            if current_layer < self.layers.len() {
                self.layers[current_layer].node_count = 
                    self.layers[current_layer].node_count.saturating_sub(1);
            }
        }

        // 새 위치에 배치
        let position = self.layers[target_layer].node_count;
        self.place_node_in_layer(graph, node_id, target_layer, position);
    }

    /// 케이크 구조 애니메이션을 위한 타겟 위치 계산
    pub fn get_target_positions(&self) -> HashMap<String, Position3D> {
        let mut targets = HashMap::new();
        
        for (node_id, layer_pos) in &self.node_positions {
            targets.insert(node_id.clone(), layer_pos.position);
        }
        
        targets
    }

    /// 층별 통계 정보
    pub fn get_layer_stats(&self) -> Vec<(usize, usize, usize)> {
        self.layers.iter()
            .map(|layer| (layer.level, layer.node_count, layer.max_nodes))
            .collect()
    }

    /// 3D 뷰에서 최적의 카메라 위치 계산
    pub fn get_optimal_camera_distance(&self) -> f32 {
        let max_radius = self.layers.iter()
            .map(|layer| layer.radius)
            .fold(0.0, f32::max);
        
        let total_height = self.max_layers as f32 * 100.0;
        
        // 모든 층이 보이도록 거리 계산
        (max_radius * 2.5 + total_height).max(400.0)
    }

    /// 특정 층에 포커스하기 위한 카메라 타겟
    pub fn get_layer_center(&self, layer: usize) -> Position3D {
        if layer < self.layers.len() {
            Position3D::new(0.0, self.layers[layer].height, 0.0)
        } else {
            Position3D::zero()
        }
    }

    /// 케이크 구조 재정렬 (노드가 추가/삭제된 후)
    pub fn rebalance_cake(&mut self, graph: &mut Graph) {
        // 현재 그래프 상태를 기반으로 케이크 재배치
        self.arrange_graph_as_cake(graph);
    }

    /// 층별 색상 가져오기
    pub fn get_layer_color(&self, layer: usize) -> [f32; 4] {
        if layer < self.layers.len() {
            self.layers[layer].color
        } else {
            [1.0, 1.0, 1.0, 1.0]
        }
    }
} 