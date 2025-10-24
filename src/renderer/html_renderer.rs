use anyhow::Result;
use gtk::prelude::*;
use webkit2gtk::{WebView, WebViewExt};
use std::rc::Rc;

use crate::utils::config_parser::WidgetConfig;

pub struct HtmlRenderer {
    window: gtk::Window,
    webview: WebView,
}

impl HtmlRenderer {
    pub fn new(config: &WidgetConfig) -> Result<Self> {
        // Crear ventana
        let window = gtk::Window::new(gtk::WindowType::Toplevel);
        
        // Configurar propiedades de la ventana
        window.set_title(&config.name);
        window.set_default_size(config.size.width, config.size.height);
        window.move_(config.position.x, config.position.y);
        
        // Decoraciones
        if let Some(decorations) = config.decorations {
            window.set_decorated(decorations);
        } else {
            window.set_decorated(false);
        }

        // Transparencia
        if config.transparent.unwrap_or(true) {
            if let Some(screen) = window.screen() {
                if let Some(visual) = screen.rgba_visual() {
                    window.set_visual(Some(&visual));
                }
            }
            window.set_app_paintable(true);
        }

        // Tipo de ventana (para que sea ignorada por el WM si es necesario)
        window.set_type_hint(gdk::WindowTypeHint::Dock);
        window.set_keep_below(true); // Mantener debajo de otras ventanas por defecto
        
        // Layer
        if let Some(layer) = &config.layer {
            match layer.as_str() {
                "top" => window.set_keep_above(true),
                "background" => window.set_keep_below(true),
                _ => {}
            }
        }

        // Configurar ancla/posición
        if let Some(anchor) = &config.position.anchor {
            Self::apply_anchor(&window, anchor, config);
        }

        // Crear WebView
        let webview = WebView::new();
        
        // Configurar WebView para transparencia
        webview.set_background_color(&gdk::RGBA::new(0.0, 0.0, 0.0, 0.0));
        
        // Agregar WebView a la ventana
        window.add(&webview);
        
        // Mostrar todo
        window.show_all();

        // Manejar cierre de ventana
        let window_clone = window.clone();
        window.connect_delete_event(move |_, _| {
            // No cerrar la aplicación completa, solo ocultar
            window_clone.hide();
            gtk::Inhibit(true)
        });

        Ok(Self { window, webview })
    }

    fn apply_anchor(window: &gtk::Window, anchor: &str, config: &WidgetConfig) {
        if let Some(screen) = window.screen() {
            let screen_width = screen.width();
            let screen_height = screen.height();

            let (x, y) = match anchor {
                "top-left" => (config.position.x, config.position.y),
                "top-right" => (
                    screen_width - config.size.width - config.position.x,
                    config.position.y,
                ),
                "bottom-left" => (
                    config.position.x,
                    screen_height - config.size.height - config.position.y,
                ),
                "bottom-right" => (
                    screen_width - config.size.width - config.position.x,
                    screen_height - config.size.height - config.position.y,
                ),
                "center" => (
                    (screen_width - config.size.width) / 2 + config.position.x,
                    (screen_height - config.size.height) / 2 + config.position.y,
                ),
                _ => (config.position.x, config.position.y),
            };

            window.move_(x, y);
        }
    }

    pub fn render(&self, html: &str, css: Option<&str>) -> Result<()> {
        // Construir el HTML completo con CSS
        let full_html = if let Some(css_content) = css {
            format!(
                r#"<!DOCTYPE html>
<html>
<head>
    <meta charset="UTF-8">
    <style>
        body {{
            margin: 0;
            padding: 0;
            background: transparent;
            overflow: hidden;
        }}
        {}
    </style>
</head>
<body>
    {}
</body>
</html>"#,
                css_content, html
            )
        } else {
            format!(
                r#"<!DOCTYPE html>
<html>
<head>
    <meta charset="UTF-8">
    <style>
        body {{
            margin: 0;
            padding: 0;
            background: transparent;
            overflow: hidden;
        }}
    </style>
</head>
<body>
    {}
</body>
</html>"#,
                html
            )
        };

        // Cargar el HTML en el WebView
        self.webview.load_html(&full_html, None);

        Ok(())
    }

    pub fn execute_script(&self, script: &str) -> Result<()> {
        self.webview.run_javascript(script, None::<&gio::Cancellable>, |_| {});
        Ok(())
    }

    pub fn update_data(&self, data: &serde_json::Value) -> Result<()> {
        let script = format!(
            "if (typeof updateWidget === 'function') {{ updateWidget({}); }}",
            data
        );
        self.execute_script(&script)
    }

    pub fn show(&self) {
        self.window.show_all();
    }

    pub fn hide(&self) {
        self.window.hide();
    }

    pub fn window(&self) -> &gtk::Window {
        &self.window
    }
}