use serde::{Deserialize, Serialize};
use std::path::PathBuf;

//////////////////////////
// Library & Content Types
//////////////////////////

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ContentType {
    Anime,
    Manga,
    Novel,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LibraryItem {
    pub id: String, // Unique ID for item
    pub title: String,
    pub description: Option<String>,
    pub content_type: ContentType,
    pub author: Option<String>,
    pub source: Option<String>,      // URL or extension source
    pub cover_path: Option<PathBuf>, // Cached cover image path
    pub chapters: Vec<Chapter>,
    pub progress: Option<ReadingProgress>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Chapter {
    pub id: String,
    pub title: String,
    pub number: f32,                 // Chapter number (can be decimal)
    pub url: Option<String>,         // Remote URL
    pub local_path: Option<PathBuf>, // Cached local file/folder
    pub status: ChapterStatus,
    pub downloaded: bool,
    pub release_date: Option<u64>, // timestamp
    pub total_pages: Option<u32>,
    pub current_page: Option<u32>,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum ChapterStatus {
    NotStarted,
    InProgress,
    Completed,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ReadingProgress {
    pub last_read_chapter: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Category {
    pub id: String,
    pub name: String,
    pub item_ids: Vec<String>, // IDs of LibraryItems in this category
}

//////////////////////////
// Plugin Types
//////////////////////////

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Plugin {
    pub id: String,
    pub name: String,
    pub source: String,      // URL or path to plugiin folder
    pub entry_point: String, // Main script file
    pub version: String,
    pub author: Option<String>,
    pub description: Option<String>,
    pub enabled: bool,
    pub icon_path: Option<PathBuf>, // Path to plugin icon
    pub supported_content_types: Vec<ContentType>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum PluginEvent {
    FetchChapter { source: String, chapter_id: String },
    UpdateLibrary,
    Custom(String), // For plugin-defined events
}

//////////////////////////
// Cache & Files
//////////////////////////

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CacheItem {
    pub content_id: String,
    pub chapter_id: Option<String>,
    pub file_path: PathBuf,
    pub downloaded_at: Option<u64>, // timestamp
}

//////////////////////////
// Config / Settings
//////////////////////////

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AppSettings {
    pub theme: String, // light/dark
    pub font_size: u8,
    pub download_dir: PathBuf,
    pub max_cache_size_mb: u64,
}
