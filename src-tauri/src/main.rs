#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![cfg_attr(target_os = "android", allow(dead_code, unused_imports))]

fn main() {
    #[cfg(not(target_os = "android"))]
    rapidraw::run();
}
