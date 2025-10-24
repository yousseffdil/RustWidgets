mod core;
mod renderer;
mod utils;

use anyhow::Result;
use gtk::prelude::*;
use log::{info, error};
use std::path::PathBuf;

use crate::core::widget_manager::WidgetManager;
use crate::utils::config_parser::ConfigParser;

fn main() -> Result<()> {
    env_logger::init();
    
    info!("Iniciando RustyWidgets...");

    gtk::init()?;

    let config_path = PathBuf::from("config/config.toml");
    let config = ConfigParser::load_main_config(&config_path)?;
    
    info!("Configuración cargada: {:?}", config);

    let mut widget_manager = WidgetManager::new(config)?;

    widget_manager.load_widgets()?;

    info!("RustyWidgets iniciado correctamente");
    gtk::main();

    Ok(())
}