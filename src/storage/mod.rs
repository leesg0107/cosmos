use std::fs;
use std::path::PathBuf;
use crate::core::universe::Universe;

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

    pub fn load_universe(&self, id: &str) -> Option<Universe> {
        let file_path = self.data_dir.join(format!("{}.json", id));
        
        // JSON 파일 읽기 및 역직렬화
        fs::read_to_string(&file_path)
            .ok()
            .and_then(|json| serde_json::from_str(&json).ok())
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
        fs::remove_file(file_path).is_ok()
    }
} 