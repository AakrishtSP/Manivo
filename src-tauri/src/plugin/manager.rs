use crate::types::*;
use log::info;
use mlua::prelude::*;
use std::env;
use std::path::PathBuf;

pub fn get_plugin_list() -> Vec<Plugin> {
    let mut plugins: Vec<Plugin> = Vec::new();
    let path = env::current_dir()
        .unwrap_or(PathBuf::from("."))
        .join("..")
        .join("plugins")
        .canonicalize()
        .unwrap();
    info!("Looking for plugins in: {}", path.display());

    for entry in std::fs::read_dir(path).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        info!("Found entry: {}", path.display());
        if !path.is_dir() {
            break;
        }
        let manifest_path = path.join("manifest.json");

        if !manifest_path.exists() {
            break;
        }
        info!("Found manifest: {}", manifest_path.display());
        if let Ok(manifest_content) = std::fs::read_to_string(&manifest_path) {
            if let Ok(manifest) = serde_json::from_str::<serde_json::Value>(&manifest_content) {
                let plugin = Plugin {
                    // Create a unique ID for the plugin, e.g., using its folder name
                    id: path
                        .file_name()
                        .and_then(|n| n.to_str())
                        .unwrap_or("unknown_plugin")
                        .to_string(),
                    name: manifest
                        .get("name")
                        .and_then(|v| v.as_str())
                        .unwrap_or("Unnamed Plugin")
                        .to_string(),
                    source: path.to_string_lossy().to_string(),
                    entry_point: manifest
                        .get("entry_point")
                        .and_then(|v| v.as_str())
                        .unwrap_or("main.lua")
                        .to_string(),
                    version: manifest
                        .get("version")
                        .and_then(|v| v.as_str())
                        .unwrap_or("0.1.0")
                        .to_string(),
                    author: manifest
                        .get("author")
                        .and_then(|v| v.as_str())
                        .map(|s| s.to_string()),
                    description: manifest
                        .get("description")
                        .and_then(|v| v.as_str())
                        .map(|s| s.to_string()),
                    enabled: manifest
                        .get("enabled")
                        .and_then(|v| v.as_bool())
                        .unwrap_or(true),
                    icon_path: manifest
                        .get("icon")
                        .and_then(|v| v.as_str())
                        .map(|s| path.join(s)),

                    supported_content_types: vec![], // This would need to be parsed
                };

                plugins.push(plugin);
            }
        }
    }
    return plugins;
}
