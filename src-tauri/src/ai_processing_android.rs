use anyhow::{Result, anyhow};
use image::{DynamicImage, GrayImage, Rgb32FImage};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use tauri::AppHandle;
use tokio::sync::Mutex as TokioMutex;

pub struct AiModels {
    pub sam_encoder: Mutex<()>,
    pub sam_decoder: Mutex<()>,
    pub u2netp: Mutex<()>,
    pub sky_seg: Mutex<()>,
}

pub struct ClipModels {
    pub model: Mutex<()>,
}

#[derive(Clone)]
pub struct ImageEmbeddings {
    pub path_hash: String,
    pub original_size: (u32, u32),
}

pub struct AiState {
    pub models: Option<Arc<AiModels>>,
    pub denoise_model: Option<Arc<Mutex<()>>>,
    pub clip_models: Option<Arc<ClipModels>>,
    pub embeddings: Option<ImageEmbeddings>,
}

pub async fn get_or_init_ai_models(
    _app_handle: &AppHandle,
    _ai_state_mutex: &Mutex<Option<AiState>>,
    _ai_init_lock: &TokioMutex<()>,
) -> Result<Arc<AiModels>> {
    Err(anyhow!("AI models are not available on Android yet"))
}

pub async fn get_or_init_denoise_model(
    _app_handle: &AppHandle,
    _ai_state_mutex: &Mutex<Option<AiState>>,
    _ai_init_lock: &TokioMutex<()>,
) -> Result<Arc<Mutex<()>>> {
    Err(anyhow!("AI denoise model is not available on Android yet"))
}

pub async fn get_or_init_clip_models(
    _app_handle: &AppHandle,
    _ai_state_mutex: &Mutex<Option<AiState>>,
    _ai_init_lock: &TokioMutex<()>,
) -> Result<Arc<ClipModels>> {
    Err(anyhow!("CLIP model is not available on Android yet"))
}

pub fn run_ai_denoise(
    _rgb_img: &Rgb32FImage,
    _intensity: f32,
    _session: &Mutex<()>,
    _app_handle: &AppHandle,
) -> Result<DynamicImage> {
    Err(anyhow!("AI denoising is not available on Android yet"))
}

pub fn generate_image_embeddings(
    _image: &DynamicImage,
    _encoder: &Mutex<()>,
) -> Result<ImageEmbeddings> {
    Err(anyhow!("AI subject mask embeddings are not available on Android yet"))
}

pub fn run_sam_decoder(
    _decoder: &Mutex<()>,
    _embeddings: &ImageEmbeddings,
    _start_point: (f64, f64),
    _end_point: (f64, f64),
) -> Result<GrayImage> {
    Err(anyhow!("AI subject mask decoder is not available on Android yet"))
}

pub fn run_sky_seg_model(_image: &DynamicImage, _sky_seg: &Mutex<()>) -> Result<GrayImage> {
    Err(anyhow!("AI sky mask is not available on Android yet"))
}

pub fn run_u2netp_model(_image: &DynamicImage, _u2netp: &Mutex<()>) -> Result<GrayImage> {
    Err(anyhow!("AI foreground mask is not available on Android yet"))
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct AiSubjectMaskParameters {
    pub start_x: f64,
    pub start_y: f64,
    pub end_x: f64,
    pub end_y: f64,
    #[serde(default)]
    pub mask_data_base64: Option<String>,
    #[serde(default)]
    pub rotation: Option<f32>,
    #[serde(default)]
    pub flip_horizontal: Option<bool>,
    #[serde(default)]
    pub flip_vertical: Option<bool>,
    #[serde(default)]
    pub orientation_steps: Option<u8>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct AiSkyMaskParameters {
    #[serde(default)]
    pub mask_data_base64: Option<String>,
    #[serde(default)]
    pub rotation: Option<f32>,
    #[serde(default)]
    pub flip_horizontal: Option<bool>,
    #[serde(default)]
    pub flip_vertical: Option<bool>,
    #[serde(default)]
    pub orientation_steps: Option<u8>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct AiForegroundMaskParameters {
    #[serde(default)]
    pub mask_data_base64: Option<String>,
    #[serde(default)]
    pub rotation: Option<f32>,
    #[serde(default)]
    pub flip_horizontal: Option<bool>,
    #[serde(default)]
    pub flip_vertical: Option<bool>,
    #[serde(default)]
    pub orientation_steps: Option<u8>,
}
