use std::fs;
use std::path::PathBuf;
use crate::core::universe::Universe;
use crate::celestial::Graph;

#[derive(Clone, Debug)]
pub struct UniverseInfo {
    pub id: String,
    pub title: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub node_count: usize,
}

pub struct Storage {
    data_dir: PathBuf,
}

impl Storage {
    pub fn new() -> Self {
        let data_dir = dirs::data_local_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("cosmos");
        
        // 데이터 디렉토리 생성
        fs::create_dir_all(&data_dir).unwrap_or_else(|e| {
            eprintln!("Failed to create data directory: {}", e);
        });

        Self { data_dir }
    }

    pub fn save_universe(&self, universe: &Universe, id: &str) {
        let file_path = self.data_dir.join(format!("{}.json", id));
        
        // Universe를 JSON으로 직렬화
        if let Ok(json) = serde_json::to_string_pretty(universe) {
            if let Err(e) = fs::write(&file_path, json) {
                eprintln!("Failed to save universe: {}", e);
            }
        }
    }

    /// Graph 저장 (새로운 메서드)
    pub fn save_graph(&self, graph: &Graph, id: &str) {
        let file_path = self.data_dir.join(format!("graph_{}.json", id));
        
        if let Ok(json) = serde_json::to_string_pretty(graph) {
            if let Err(e) = fs::write(&file_path, json) {
                eprintln!("Failed to save graph: {}", e);
            }
        }
    }

    pub fn load_universe(&self, id: &str) -> Result<Graph, Box<dyn std::error::Error>> {
        // 먼저 Graph 파일 형식으로 시도
        let graph_path = self.data_dir.join(format!("graph_{}.json", id));
        if graph_path.exists() {
            let json = fs::read_to_string(&graph_path)?;
            let graph: Graph = serde_json::from_str(&json)?;
            return Ok(graph);
        }
        
        // 기존 Universe 파일 형식으로 시도 (하위 호환성)
        let universe_path = self.data_dir.join(format!("{}.json", id));
        if universe_path.exists() {
            let json = fs::read_to_string(&universe_path)?;
            let _universe: Universe = serde_json::from_str(&json)?;
            // Universe를 Graph로 변환하는 로직이 필요하지만, 임시로 새 Graph 반환
            return Ok(Graph::new());
        }
        
        Err("Universe not found".into())
    }

    /// UniverseInfo 목록 반환
    pub fn list_universes(&self) -> Vec<UniverseInfo> {
        let mut universes = Vec::new();
        
        if let Ok(entries) = fs::read_dir(&self.data_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if let Some(extension) = path.extension() {
                    if extension == "json" {
                        if let Some(file_name) = path.file_stem() {
                            let file_name_str = file_name.to_string_lossy();
                            
                            // graph_ 접두사 제거
                            let id = if file_name_str.starts_with("graph_") {
                                file_name_str.strip_prefix("graph_").unwrap_or(&file_name_str)
                            } else {
                                &file_name_str
                            };
                            
                            // 메타데이터 수집
                            let node_count = self.get_node_count(&path);
                            let created_at = self.get_file_created_time(&path);
                            
                            universes.push(UniverseInfo {
                                id: id.to_string(),
                                title: format!("Universe {}", id),
                                created_at,
                                node_count,
                            });
                        }
                    }
                }
            }
        }
        
        // 생성일 기준 내림차순 정렬
        universes.sort_by(|a, b| b.created_at.cmp(&a.created_at));
        universes
    }

    pub fn get_universe_list(&self) -> impl Iterator<Item = Universe> {
        fs::read_dir(&self.data_dir)
            .into_iter()
            .flatten()
            .filter_map(|entry| {
                let entry = entry.ok()?;
                let path = entry.path();
                if path.extension()? == "json" {
                    fs::read_to_string(path)
                        .ok()
                        .and_then(|json| serde_json::from_str(&json).ok())
                } else {
                    None
                }
            })
    }

    pub fn delete_universe(&self, id: &str) -> bool {
        let file_path = self.data_dir.join(format!("{}.json", id));
        let graph_path = self.data_dir.join(format!("graph_{}.json", id));
        
        let mut success = false;
        if file_path.exists() {
            success |= fs::remove_file(file_path).is_ok();
        }
        if graph_path.exists() {
            success |= fs::remove_file(graph_path).is_ok();
        }
        
        success
    }

    /// 파일에서 노드 개수 추출 (근사치)
    fn get_node_count(&self, path: &PathBuf) -> usize {
        if let Ok(content) = fs::read_to_string(path) {
            // JSON에서 "nodes" 필드의 개수를 대략적으로 계산
            content.matches("\"id\":").count()
        } else {
            0
        }
    }

    /// 파일 생성 시간 가져오기
    fn get_file_created_time(&self, path: &PathBuf) -> chrono::DateTime<chrono::Utc> {
        if let Ok(metadata) = fs::metadata(path) {
            if let Ok(created) = metadata.created() {
                return chrono::DateTime::from(created);
            }
        }
        chrono::Utc::now()
    }
} 