use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::fs;
use anyhow::Result;

const CONFIG_DIR: &str = ".local/config/vortex";
const CONFIG_FILE: &str = "config.json";

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VortexConfig {
    pub single_click_to_open: bool,
    pub show_hidden_files: bool,
    pub default_view_mode: String, // "grid" or "list"
    pub window_width: i32,
    pub window_height: i32,
    pub sidebar_width: i32,
    pub default_icon_size: i32, // Default icon size in pixels
}

impl Default for VortexConfig {
    fn default() -> Self {
        Self {
            single_click_to_open: true,
            show_hidden_files: false,
            default_view_mode: "grid".to_string(),
            window_width: 1200,
            window_height: 800,
            sidebar_width: 250,
            default_icon_size: 32, // Default to 32px
        }
    }
}

impl VortexConfig {
    pub fn load() -> Self {
        let config_path = Self::get_config_path();
        
        if let Ok(config_data) = fs::read_to_string(&config_path) {
            if let Ok(config) = serde_json::from_str(&config_data) {
                return config;
            }
        }
        
        // If loading fails, create default config
        let default_config = Self::default();
        let _ = default_config.save();
        default_config
    }
    
    pub fn save(&self) -> Result<()> {
        let config_path = Self::get_config_path();
        
        // Create config directory if it doesn't exist
        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent)?;
        }
        
        let config_data = serde_json::to_string_pretty(self)?;
        fs::write(&config_path, config_data)?;
        Ok(())
    }
    
    fn get_config_path() -> PathBuf {
        let home = std::env::var("HOME").unwrap_or_else(|_| "/home".to_string());
        PathBuf::from(home).join(CONFIG_DIR).join(CONFIG_FILE)
    }
}
