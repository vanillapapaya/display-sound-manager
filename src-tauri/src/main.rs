// src-tauri/src/main.rs
#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use std::process::Command;
use std::fs;
use std::path::PathBuf;
use tauri::Manager;

#[cfg(target_os = "macos")]
use core_graphics::display::{CGDirectDisplayID, CGDisplayBounds, CGGetActiveDisplayList, CGMainDisplayID, CGDisplayPixelsWide, CGDisplayPixelsHigh};

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
    profiles_file: PathBuf,
}

impl AppState {
    fn load_profiles(&self) -> Result<Vec<Profile>, String> {
        if self.profiles_file.exists() {
            let content = fs::read_to_string(&self.profiles_file)
                .map_err(|e| format!("Failed to read profiles file: {}", e))?;
            let profiles: Vec<Profile> = serde_json::from_str(&content)
                .map_err(|e| format!("Failed to parse profiles: {}", e))?;
            Ok(profiles)
        } else {
            Ok(Vec::new())
        }
    }
    
    fn save_profiles(&self, profiles: &[Profile]) -> Result<(), String> {
        if let Some(parent) = self.profiles_file.parent() {
            fs::create_dir_all(parent)
                .map_err(|e| format!("Failed to create profiles directory: {}", e))?;
        }
        
        let content = serde_json::to_string_pretty(profiles)
            .map_err(|e| format!("Failed to serialize profiles: {}", e))?;
        fs::write(&self.profiles_file, content)
            .map_err(|e| format!("Failed to write profiles file: {}", e))?;
        Ok(())
    }
}

// 디스플레이 정보 가져오기
#[tauri::command]
async fn get_displays() -> Result<Vec<DisplayInfo>, String> {
    #[cfg(target_os = "macos")]
    {
        get_displays_macos()
    }
    #[cfg(not(target_os = "macos"))]
    {
        // 다른 OS용 기본 구현
        Ok(vec![
            DisplayInfo {
                id: 1,
                name: "Primary Display".to_string(),
                width: 1920,
                height: 1080,
                x: 0,
                y: 0,
                scale_factor: 1.0,
                is_primary: true,
                rotation: 0,
            },
        ])
    }
}

#[cfg(target_os = "macos")]
fn get_displays_macos() -> Result<Vec<DisplayInfo>, String> {
    let mut displays = Vec::new();
    let mut display_count: u32 = 0;
    let max_displays = 32;
    let mut display_ids: Vec<CGDirectDisplayID> = vec![0; max_displays];
    
    unsafe {
        let result = CGGetActiveDisplayList(
            max_displays as u32,
            display_ids.as_mut_ptr(),
            &mut display_count,
        );
        
        if result != 0 {
            return Err("Failed to get display list".to_string());
        }
        
        let main_display_id = CGMainDisplayID();
        
        for i in 0..display_count {
            let display_id = display_ids[i as usize];
            let bounds = CGDisplayBounds(display_id);
            let width = CGDisplayPixelsWide(display_id);
            let height = CGDisplayPixelsHigh(display_id);
            
            displays.push(DisplayInfo {
                id: display_id,
                name: format!("Display {}", i + 1),
                width: width.try_into().unwrap(),
                height: height.try_into().unwrap(),
                x: bounds.origin.x as i32,
                y: bounds.origin.y as i32,
                scale_factor: 1.0, // TODO: 실제 스케일 팩터 구하기
                is_primary: display_id == main_display_id,
                rotation: 0, // TODO: 실제 회전 값 구하기
            });
        }
    }
    
    Ok(displays)
}

// 오디오 장치 정보 가져오기
#[tauri::command]
async fn get_audio_devices() -> Result<Vec<AudioDevice>, String> {
    #[cfg(target_os = "macos")]
    {
        get_audio_devices_macos()
    }
    #[cfg(not(target_os = "macos"))]
    {
        // 다른 OS용 기본 구현
        Ok(vec![
            AudioDevice {
                id: "default_output".to_string(),
                name: "Default Output".to_string(),
                is_default: true,
                device_type: "output".to_string(),
            },
            AudioDevice {
                id: "default_input".to_string(),
                name: "Default Input".to_string(),
                is_default: true,
                device_type: "input".to_string(),
            },
        ])
    }
}

#[cfg(target_os = "macos")]
fn get_audio_devices_macos() -> Result<Vec<AudioDevice>, String> {
    let mut devices = Vec::new();
    
    // SwitchAudioSource를 사용해서 오디오 장치 목록 가져오기
    match Command::new("SwitchAudioSource")
        .arg("-a")
        .output()
    {
        Ok(output) => {
            let output_str = String::from_utf8_lossy(&output.stdout);
            for line in output_str.lines() {
                if !line.trim().is_empty() {
                    devices.push(AudioDevice {
                        id: line.trim().to_string(),
                        name: line.trim().to_string(),
                        is_default: false, // TODO: 기본 장치 확인
                        device_type: "output".to_string(),
                    });
                }
            }
        }
        Err(_) => {
            // SwitchAudioSource가 없는 경우 기본 장치만 반환
            devices.push(AudioDevice {
                id: "default_output".to_string(),
                name: "기본 출력 장치".to_string(),
                is_default: true,
                device_type: "output".to_string(),
            });
        }
    }
    
    // 입력 장치도 추가
    devices.push(AudioDevice {
        id: "default_input".to_string(),
        name: "기본 입력 장치".to_string(),
        is_default: true,
        device_type: "input".to_string(),
    });
    
    Ok(devices)
}

// 프로필 저장
#[tauri::command]
async fn save_profile(
    state: tauri::State<'_, AppState>,
    profile: Profile,
) -> Result<(), String> {
    let mut profiles = state.profiles.lock().unwrap();
    
    if let Some(pos) = profiles.iter().position(|p| p.id == profile.id) {
        profiles[pos] = profile.clone();
    } else {
        profiles.push(profile.clone());
    }
    
    // 파일에 저장
    state.save_profiles(&profiles)?;
    
    Ok(())
}

// 프로필 목록 가져오기
#[tauri::command]
async fn get_profiles(state: tauri::State<'_, AppState>) -> Result<Vec<Profile>, String> {
    let mut profiles = state.profiles.lock().unwrap();
    
    // 파일에서 프로필 로드
    let loaded_profiles = state.load_profiles()?;
    *profiles = loaded_profiles;
    
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
    
    // 파일에 저장
    state.save_profiles(&profiles)?;
    
    Ok(())
}

// 프로필 적용
#[tauri::command]
async fn apply_profile(
    state: tauri::State<'_, AppState>,
    profile_id: String,
) -> Result<(), String> {
    let profiles = state.profiles.lock().unwrap();
    
    if let Some(profile) = profiles.iter().find(|p| p.id == profile_id) {
        // 디스플레이 설정 적용
        apply_display_settings(&profile.displays)?;
        
        // 오디오 설정 적용
        apply_audio_settings(&profile.audio_settings)?;
        
        Ok(())
    } else {
        Err("프로필을 찾을 수 없습니다.".to_string())
    }
}

// 디스플레이 설정 적용
fn apply_display_settings(displays: &[DisplayInfo]) -> Result<(), String> {
    #[cfg(target_os = "macos")]
    {
        apply_display_settings_macos(displays)
    }
    #[cfg(not(target_os = "macos"))]
    {
        Ok(()) // 다른 OS에서는 아직 미구현
    }
}

#[cfg(target_os = "macos")]
fn apply_display_settings_macos(displays: &[DisplayInfo]) -> Result<(), String> {
    // displayplacer를 사용해서 디스플레이 설정 적용
    let mut args = Vec::new();
    
    for display in displays {
        let display_arg = format!(
            "id:{} res:{}x{} origin:({},{}) degree:{}",
            display.id,
            display.width,
            display.height,
            display.x,
            display.y,
            display.rotation
        );
        args.push(display_arg);
    }
    
    match Command::new("displayplacer")
        .args(&args)
        .output()
    {
        Ok(output) => {
            if output.status.success() {
                Ok(())
            } else {
                let error = String::from_utf8_lossy(&output.stderr);
                Err(format!("디스플레이 설정 실패: {}", error))
            }
        }
        Err(e) => {
            Err(format!("displayplacer 실행 실패: {}. displayplacer가 설치되어 있는지 확인하세요.", e))
        }
    }
}

// 오디오 설정 적용
fn apply_audio_settings(audio_settings: &AudioSettings) -> Result<(), String> {
    #[cfg(target_os = "macos")]
    {
        apply_audio_settings_macos(audio_settings)
    }
    #[cfg(not(target_os = "macos"))]
    {
        Ok(()) // 다른 OS에서는 아직 미구현
    }
}

#[cfg(target_os = "macos")]
fn apply_audio_settings_macos(audio_settings: &AudioSettings) -> Result<(), String> {
    // 출력 장치 설정
    if let Some(output_device) = &audio_settings.output_device {
        match Command::new("SwitchAudioSource")
            .arg("-s")
            .arg(output_device)
            .output()
        {
            Ok(output) => {
                if !output.status.success() {
                    let error = String::from_utf8_lossy(&output.stderr);
                    return Err(format!("오디오 출력 장치 설정 실패: {}", error));
                }
            }
            Err(e) => {
                return Err(format!("SwitchAudioSource 실행 실패: {}. SwitchAudioSource가 설치되어 있는지 확인하세요.", e));
            }
        }
    }
    
    // TODO: 입력 장치 및 볼륨 설정 구현
    
    Ok(())
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            // 프로필 파일 경로 설정
            let app_data_dir = app.path().app_data_dir()
                .expect("Failed to get app data directory");
            let profiles_file = app_data_dir.join("profiles.json");
            
            // 앱 상태 초기화
            let app_state = AppState {
                profiles: Mutex::new(Vec::new()),
                profiles_file,
            };
            
            // 기존 프로필 로드
            if let Ok(profiles) = app_state.load_profiles() {
                *app_state.profiles.lock().unwrap() = profiles;
            }
            
            app.manage(app_state);

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