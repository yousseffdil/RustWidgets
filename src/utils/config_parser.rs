use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct MainConfig {
    pub widgets_dir: String,
    pub config_dir: String,
    pub auto_reload: bool,
    pub log_level: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct WidgetConfig {
    pub name: String,
    pub enabled: bool,
    pub position: Position,
    pub size: Size,
    pub update_interval: Option<u64>, // en milisegundos
    pub transparent: Option<bool>,
    pub decorations: Option<bool>,
    pub monitor: Option<i32>,
    pub layer: Option<String>, // "background", "normal", "top"
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Position {
    pub x: i32,
    pub y: i32,
    pub anchor: Option<String>, // "top-left", "top-right", "bottom-left", "bottom-right", "center"
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Size {
    pub width: i32,
    pub height: i32,
}

pub struct ConfigParser;

impl ConfigParser {
    pub fn load_main_config(path: &Path) -> Result<MainConfig> {
        let content = fs::read_to_string(path)
            .context(format!("No se pudo leer el archivo de configuración: {:?}", path))?;
        
        let config: MainConfig = toml::from_str(&content)
            .context("Error al parsear la configuración principal")?;
        
        Ok(config)
    }

    pub fn load_widget_config(path: &Path) -> Result<WidgetConfig> {
        let content = fs::read_to_string(path)
            .context(format!("No se pudo leer el widget config: {:?}", path))?;
        
        let config: WidgetConfig = toml::from_str(&content)
            .context("Error al parsear la configuración del widget")?;
        
        Ok(config)
    }

    pub fn list_widget_configs(config_dir: &str) -> Result<Vec<WidgetConfig>> {
        let widgets_path = Path::new(config_dir).join("widgets");
        let mut configs = Vec::new();

        if !widgets_path.exists() {
            return Ok(configs);
        }

        for entry in fs::read_dir(widgets_path)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.extension().and_then(|s| s.to_str()) == Some("toml") {
                match Self::load_widget_config(&path) {
                    Ok(config) => {
                        if config.enabled {
                            configs.push(config);
                        }
                    }
                    Err(e) => {
                        log::warn!("No se pudo cargar widget config {:?}: {}", path, e);
                    }
                }
            }
        }

        Ok(configs)
    }
}