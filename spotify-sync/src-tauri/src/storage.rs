use std::fs;
use std::path::PathBuf;

#[derive(Clone)]
pub struct LocalStorage {
    storage_dir: PathBuf,
}

impl LocalStorage {
    pub fn new() -> Self {
        let storage_dir = dirs::home_dir()
            .map(|h| h.join(".spotify_sync"))
            .unwrap_or_else(|| PathBuf::from(".spotify_sync"));

        // Create dir if it doesn't exist
        let _ = fs::create_dir_all(&storage_dir);

        LocalStorage { storage_dir }
    }

    fn get_token_path(&self, panel: &str) -> Result<PathBuf, Box<dyn std::error::Error>> {
        Ok(self.storage_dir.join(format!("{}_token.txt", panel)))
    }

    pub fn save_token(&self, panel: &str, token: &str) -> Result<(), Box<dyn std::error::Error>> {
        let file_path = self.get_token_path(panel)?;
        fs::write(file_path, token)?;
        Ok(())
    }

    pub fn load_token(&self, panel: &str) -> Result<String, Box<dyn std::error::Error>> {
        let file_path = self.get_token_path(panel)?;
        match fs::read_to_string(file_path) {
            Ok(token) => Ok(token),
            Err(_) => Ok(String::new()),
        }
    }
#[allow(dead_code)]
    pub fn clear_token(&self, panel: &str) -> Result<(), Box<dyn std::error::Error>> {
        let file_path = self.get_token_path(panel)?;
        fs::remove_file(file_path)?;
        Ok(())
    }
}