const COMMANDS: &[&str] = &["get_system_fonts"];

fn main() {
    tauri_plugin::Builder::new(COMMANDS).build();
}
