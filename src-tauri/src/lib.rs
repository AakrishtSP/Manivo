// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
//
use log::info;
use std::path::PathBuf;
use std::{env, path};
mod plugin;
mod types;
#[tauri::command]
fn greet(name: &str) -> String {
    let path = env::current_dir().unwrap_or(PathBuf::from("."));
    info!("Current directory: {}", path.display());
    let plugin_list = plugin::manager::get_plugin_list();
    let lua = mlua::Lua::new();
    let code_path = path
        .join(plugin_list[0].source.clone())
        .join(plugin_list[0].entry_point.clone());
    info!("Loading Lua code from: {}", code_path.display());
    let code = std::fs::read_to_string(code_path).ok();
    if let Some(code) = code {
        if let Err(e) = lua.load(&code).exec() {
            info!("Error executing Lua code: {}", e);
        }
    } else {
        info!("Failed to read Lua code.");
    }
    let globals = lua.globals();
    let lua_greet: mlua::Function = globals.get("greet").unwrap();
    let lua_result: String = lua_greet.call(name).unwrap();
    info!("Lua greet result: {}", lua_result);
    let rust_result = format!("Hello, {}! You've been greeted from Rust!", name);
    let lua_combined = format!("{} Also, {}", lua_result, rust_result);
    return lua_combined;
}

#[tauri::command]
fn get_plugins_name() -> String {
    let plugins = plugin::manager::get_plugin_list();
    let names: Vec<String> = plugins.iter().map(|p| p.name.clone()).collect();
    info!("Found plugins: {:?}", names);
    return names.join(", ");
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_log::Builder::default().build())
        .invoke_handler(tauri::generate_handler![greet, get_plugins_name])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
