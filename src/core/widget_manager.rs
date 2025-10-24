use anyhow::{Context, Result};
use std::collections::HashMap;
use std::path::PathBuf;
use log::{info, warn};

use crate::utils::config_parser::{ConfigParser, MainConfig, WidgetConfig};
use crate::renderer::html_renderer::HtmlRenderer;
use crate::core::scheduler::Scheduler;

pub struct Widget {
    pub config: WidgetConfig,
    pub renderer: HtmlRenderer,
    pub html_path: PathBuf,
    pub css_path: PathBuf,
}

pub struct WidgetManager {
    main_config: MainConfig,
    widgets: HashMap<String, Widget>,
    scheduler: Scheduler,
}

impl WidgetManager {
    pub fn new(config: MainConfig) -> Result<Self> {
        Ok(Self {
            main_config: config,
            widgets: HashMap::new(),
            scheduler: Scheduler::new(),
        })
    }

    pub fn load_widgets(&mut self) -> Result<()> {
        let configs = ConfigParser::list_widget_configs(&self.main_config.config_dir)?;
        
        info!("Encontrados {} widgets habilitados", configs.len());

        for widget_config in configs {
            match self.load_widget(widget_config) {
                Ok(_) => info!("Widget '{}' cargado correctamente", &widget_config.name),
                Err(e) => warn!("Error al cargar widget '{}': {}", &widget_config.name, e),
            }
        }

        Ok(())
    }

    fn load_widget(&mut self, config: WidgetConfig) -> Result<()> {
        let widget_name = config.name.clone();
        let widgets_dir = PathBuf::from(&self.main_config.widgets_dir);
        
        // Construir paths para HTML y CSS
        let widget_dir = widgets_dir.join(&widget_name);
        let html_path = widget_dir.join(format!("{}.html", widget_name));
        let css_path = widget_dir.join(format!("{}.css", widget_name));

        // Verificar que existan los archivos
        if !html_path.exists() {
            return Err(anyhow::anyhow!("No se encontró el archivo HTML: {:?}", html_path));
        }

        // Crear el renderer
        let renderer = HtmlRenderer::new(&config)?;

        // Cargar el HTML inicial
        let html_content = std::fs::read_to_string(&html_path)
            .context("Error al leer el archivo HTML")?;

        let css_content = if css_path.exists() {
            std::fs::read_to_string(&css_path).ok()
        } else {
            None
        };

        // Renderizar el widget
        renderer.render(&html_content, css_content.as_deref())?;

        // Crear el widget
        let widget = Widget {
            config: config.clone(),
            renderer,
            html_path,
            css_path,
        };

        // Programar actualizaciones si es necesario
        if let Some(interval) = config.update_interval {
            let widget_name_clone = widget_name.clone();
            self.scheduler.schedule_repeating(
                widget_name_clone,
                interval,
                Box::new(move || {
                    info!("Actualizando widget: {}", widget_name_clone);
                    // Aquí se ejecutaría la lógica de actualización
                })
            );
        }

        self.widgets.insert(widget_name, widget);

        Ok(())
    }

    pub fn reload_widget(&mut self, name: &str) -> Result<()> {
        if let Some(widget) = self.widgets.get(name) {
            let html_content = std::fs::read_to_string(&widget.html_path)?;
            let css_content = if widget.css_path.exists() {
                Some(std::fs::read_to_string(&widget.css_path)?)
            } else {
                None
            };

            widget.renderer.render(&html_content, css_content.as_deref())?;
            info!("Widget '{}' recargado", name);
        }
        Ok(())
    }

    pub fn get_widget(&self, name: &str) -> Option<&Widget> {
        self.widgets.get(name)
    }
}