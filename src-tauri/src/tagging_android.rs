use image::DynamicImage;
use std::sync::Mutex;
use tauri::{AppHandle, State};

use crate::AppState;

pub const COLOR_TAG_PREFIX: &str = "color:";
pub const USER_TAG_PREFIX: &str = "user:";

pub fn extract_color_tags(_image: &DynamicImage) -> Vec<String> {
    Vec::new()
}

pub fn generate_tags_with_clip(
    _image: &DynamicImage,
    _clip_session: &Mutex<()>,
    _tokenizer: &(),
    _custom_tags: Option<Vec<String>>,
    _tag_count: usize,
) -> anyhow::Result<Vec<String>> {
    Ok(Vec::new())
}

#[tauri::command]
pub async fn start_background_indexing(
    _folder_path: String,
    _app_handle: AppHandle,
    _state: State<'_, AppState>,
) -> Result<(), String> {
    Ok(())
}

#[tauri::command]
pub fn add_tag_for_paths(_paths: Vec<String>, _tag: String) -> Result<(), String> {
    Ok(())
}

#[tauri::command]
pub fn remove_tag_for_paths(_paths: Vec<String>, _tag: String) -> Result<(), String> {
    Ok(())
}

#[tauri::command]
pub fn clear_ai_tags(_root_path: String) -> Result<usize, String> {
    Ok(0)
}

#[tauri::command]
pub fn clear_all_tags(_root_path: String) -> Result<usize, String> {
    Ok(0)
}
