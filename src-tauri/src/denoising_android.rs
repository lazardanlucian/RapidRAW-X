use image::DynamicImage;
use std::sync::{Arc, Mutex};
use tauri::AppHandle;

pub fn denoise_image(
    _path_str: String,
    _intensity: f32,
    _method: String,
    _app_handle: AppHandle,
    _ai_session: Option<Arc<Mutex<()>>>,
) -> Result<(DynamicImage, String), String> {
    Err("Denoising is not available on Android yet".to_string())
}
