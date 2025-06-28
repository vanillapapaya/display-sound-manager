// src-tauri/src/main.rs
#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use tauri::Manager;

#[derive(Debug, Serialize, Deserialize, Clone)]
struct DisplayInfo {
    id: u32,
    name: String,
    width: u32,
    height: u32,
    x: i32,
    y: i32,
    scale_factor: f64,
    is_primary: bool,
    rotation: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct AudioDevice {
    id: String,
    name: String,
    is_default: bool,
    device_type: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct AudioSettings {
    output_device: Option<String>,
    input_device: Option<String>,
    output_volume: u32,
    input_volume: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Profile {
    id: String,
    name: String,
    displays: Vec<DisplayInfo>,
    audio_settings: AudioSettings,
    created_at: String,
}

struct AppState {
    profiles: Mutex<Vec<Profile>>,
}

// 디스플레이 정보 가져오기 (임시 구현)
#[tauri::command]
async fn get_displays() -> Result<Vec<DisplayInfo>, String> {
    // 임시 데이터
    Ok(vec![
        DisplayInfo {
            id: 1,
            name: "Display 1".to_string(),
            width: 1920,
            height: 1080,
            x: 0,
            y: 0,
            scale_factor: 1.0,
            is_primary: true,
            rotation: 0,
        },
        DisplayInfo {
            id: 2,
            name: "Display 2".to_string(),
            width: 1920,
            height: 1080,
            x: 1920,
            y: 0,
            scale_factor: 1.0,
            is_primary: false,
            rotation: 0,
        },
    ])
}

// 오디오 장치 정보 가져오기 (임시 구현)
#[tauri::command]
async fn get_audio_devices() -> Result<Vec<AudioDevice>, String> {
    Ok(vec![
        AudioDevice {
            id: "speakers".to_string(),
            name: "스피커".to_string(),
            is_default: true,
            device_type: "output".to_string(),
        },
        AudioDevice {
            id: "headphones".to_string(),
            name: "헤드폰".to_string(),
            is_default: false,
            device_type: "output".to_string(),
        },
        AudioDevice {
            id: "mic".to_string(),
            name: "마이크".to_string(),
            is_default: true,
            device_type: "input".to_string(),
        },
    ])
}

// 프로필 저장
#[tauri::command]
async fn save_profile(
    state: tauri::State<'_, AppState>,
    profile: Profile,
) -> Result<(), String> {
    let mut profiles = state.profiles.lock().unwrap();
    
    if let Some(pos) = profiles.iter().position(|p| p.id == profile.id) {
        profiles[pos] = profile;
    } else {
        profiles.push(profile);
    }
    
    Ok(())
}

// 프로필 목록 가져오기
#[tauri::command]
async fn get_profiles(state: tauri::State<'_, AppState>) -> Result<Vec<Profile>, String> {
    let profiles = state.profiles.lock().unwrap();
    Ok(profiles.clone())
}

// 프로필 삭제
#[tauri::command]
async fn delete_profile(
    state: tauri::State<'_, AppState>,
    profile_id: String,
) -> Result<(), String> {
    let mut profiles = state.profiles.lock().unwrap();
    profiles.retain(|p| p.id != profile_id);
    Ok(())
}

// 프로필 적용
#[tauri::command]
async fn apply_profile(
    state: tauri::State<'_, AppState>,
    profile_id: String,
) -> Result<(), String> {
    let profiles = state.profiles.lock().unwrap();
    
    if let Some(_profile) = profiles.iter().find(|p| p.id == profile_id) {
        // 실제 구현에서는 여기서 시스템 설정을 변경
        Ok(())
    } else {
        Err("프로필을 찾을 수 없습니다.".to_string())
    }
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            // 앱 상태 초기화
            app.manage(AppState {
                profiles: Mutex::new(Vec::new()),
            });

            // 창 표시
            if let Some(window) = app.get_webview_window("main") {
                window.show()?;
            }

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_displays,
            get_audio_devices,
            save_profile,
            get_profiles,
            delete_profile,
            apply_profile,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}