// Módulo para gestión de estilos
// Por ahora es un placeholder para futuras extensiones

pub struct StyleManager;

impl StyleManager {
    pub fn inject_global_styles() -> String {
        r#"
        * {
            box-sizing: border-box;
        }
        
        body {
            margin: 0;
            padding: 0;
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen, Ubuntu, Cantarell, sans-serif;
        }
        
        ::-webkit-scrollbar {
            width: 8px;
            height: 8px;
        }
        
        ::-webkit-scrollbar-track {
            background: transparent;
        }
        
        ::-webkit-scrollbar-thumb {
            background: rgba(255, 255, 255, 0.3);
            border-radius: 4px;
        }
        
        ::-webkit-scrollbar-thumb:hover {
            background: rgba(255, 255, 255, 0.5);
        }
        "#.to_string()
    }

    pub fn parse_css_variables(css: &str) -> std::collections::HashMap<String, String> {
        // Simple parser de variables CSS
        let mut variables = std::collections::HashMap::new();
        
        for line in css.lines() {
            let line = line.trim();
            if line.starts_with("--") {
                if let Some(colon_pos) = line.find(':') {
                    let name = line[..colon_pos].trim();
                    let value = line[colon_pos + 1..].trim().trim_end_matches(';');
                    variables.insert(name.to_string(), value.to_string());
                }
            }
        }
        
        variables
    }
}