use fontdb::{Database, Source, Style};
use serde::Serialize;
use std::collections::HashSet;
use tauri::command;

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SystemFont {
    pub id: String,
    pub name: String,
    pub font_name: String,
    pub path: String,
    pub weight: u16,
    pub style: String,
    pub monospaced: bool,
}

/// Get all fonts installed on the system.
///
/// # Example
/// ```
/// use tauri_plugin_system_fonts::get_system_fonts;
///
/// let fonts = get_system_fonts().await;
/// ```
#[command]
pub async fn get_system_fonts() -> Vec<SystemFont> {
    let mut db = Database::new();

    db.load_system_fonts();

    let mut seen = HashSet::new();

    db.faces()
        .filter_map(|font| match &font.source {
            Source::File(path) => {
                let name = font.families[0].0.clone();

                if name.starts_with('.') {
                    return None;
                }

                let style = match font.style {
                    Style::Normal => "normal",
                    Style::Italic => "italic",
                    Style::Oblique => "oblique",
                };

                let font = SystemFont {
                    id: font.id.to_string(),
                    name,
                    font_name: font.post_script_name.clone(),
                    path: path.to_string_lossy().to_string(),
                    weight: font.weight.0,
                    style: style.to_string(),
                    monospaced: font.monospaced,
                };

                if seen.insert((font.name.clone(), font.path.clone())) {
                    Some(font)
                } else {
                    None
                }
            }
            Source::Binary(_) => None,
            Source::SharedFile(..) => None,
        })
        .collect()
}
