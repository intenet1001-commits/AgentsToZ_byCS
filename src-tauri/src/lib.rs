use std::fs;
use std::process::Command;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use tauri::{State, Manager};
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

#[derive(Debug, Serialize, Deserialize, Clone)]
struct PortInfo {
    id: String,
    name: String,
    #[serde(default)]
    port: Option<u16>,
    #[serde(rename = "commandPath")]
    command_path: Option<String>,
    #[serde(rename = "folderPath")]
    folder_path: Option<String>,
    #[serde(rename = "deployUrl")]
    deploy_url: Option<String>,
    #[serde(rename = "githubUrl")]
    github_url: Option<String>,
    #[serde(rename = "worktreePath", default)]
    worktree_path: Option<String>,
    #[serde(default)]
    category: Option<String>,
    #[serde(default)]
    description: Option<String>,
    #[serde(rename = "aiName", default, skip_serializing_if = "Option::is_none")]
    ai_name: Option<String>,
    #[serde(rename = "isRunning", default)]
    is_running: bool,
    #[serde(default)]
    favorite: bool,
    #[serde(rename = "terminalCommand", default, skip_serializing_if = "Option::is_none")]
    terminal_command: Option<String>,
    #[serde(rename = "sourceDeviceId", default, skip_serializing_if = "Option::is_none")]
    source_device_id: Option<String>,
}

struct AppState {
    processes: Mutex<HashMap<String, u32>>,
}

#[tauri::command]
fn load_ports(app_handle: tauri::AppHandle) -> Result<Vec<PortInfo>, String> {
    // Tauri app data 디렉토리 사용
    let app_data_dir = app_handle.path().app_data_dir()
        .map_err(|e| e.to_string())?;

    // 디렉토리가 없으면 생성
    if !app_data_dir.exists() {
        fs::create_dir_all(&app_data_dir)
            .map_err(|e| format!("Failed to create app data directory: {}", e))?;
    }

    let ports_file = app_data_dir.join("ports.json");

    if ports_file.exists() {
        let content = fs::read_to_string(&ports_file)
            .map_err(|e| e.to_string())?;
        let ports: Vec<PortInfo> = serde_json::from_str(&content)
            .map_err(|e| e.to_string())?;
        return Ok(ports);
    }

    Ok(Vec::new())
}

#[tauri::command]
fn save_ports(app_handle: tauri::AppHandle, ports: Vec<PortInfo>) -> Result<(), String> {
    // Tauri app data 디렉토리 사용
    let app_data_dir = app_handle.path().app_data_dir()
        .map_err(|e| e.to_string())?;

    // 디렉토리가 없으면 생성
    if !app_data_dir.exists() {
        fs::create_dir_all(&app_data_dir)
            .map_err(|e| format!("Failed to create app data directory: {}", e))?;
    }

    let ports_file = app_data_dir.join("ports.json");
    println!("[SavePorts] Saving {} ports to: {:?}", ports.len(), ports_file);

    let content = serde_json::to_string_pretty(&ports)
        .map_err(|e| e.to_string())?;

    fs::write(&ports_file, content)
        .map_err(|e| e.to_string())?;

    println!("[SavePorts] Successfully saved ports");
    Ok(())
}

#[tauri::command]
fn scan_command_files(folder_path: String) -> Result<Vec<String>, String> {
    let path = std::path::Path::new(&folder_path);
    if !path.exists() {
        return Ok(vec![]);
    }
    let exec_exts = [".command", ".bat", ".cmd", ".sh", ".html"];
    let entries = fs::read_dir(path).map_err(|e| e.to_string())?;
    let files: Vec<String> = entries
        .filter_map(|e| e.ok())
        .filter(|e| {
            let name = e.file_name();
            let name_str = name.to_string_lossy();
            exec_exts.iter().any(|ext| name_str.ends_with(ext))
        })
        .map(|e| e.path().to_string_lossy().to_string())
        .collect();
    Ok(files)
}

#[tauri::command]
fn open_app_data_dir(app_handle: tauri::AppHandle) -> Result<(), String> {
    let app_data_dir = app_handle.path().app_data_dir().map_err(|e| e.to_string())?;
    if !app_data_dir.exists() {
        fs::create_dir_all(&app_data_dir).map_err(|e| e.to_string())?;
    }
    std::process::Command::new("open")
        .arg(&app_data_dir)
        .spawn()
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
fn load_portal(app_handle: tauri::AppHandle) -> Result<serde_json::Value, String> {
    let app_data_dir = app_handle.path().app_data_dir().map_err(|e| e.to_string())?;
    if !app_data_dir.exists() {
        fs::create_dir_all(&app_data_dir).map_err(|e| e.to_string())?;
    }
    let file = app_data_dir.join("portal.json");
    if file.exists() {
        let content = fs::read_to_string(&file).map_err(|e| e.to_string())?;
        let val: serde_json::Value = serde_json::from_str(&content).map_err(|e| e.to_string())?;
        return Ok(val);
    }
    Ok(serde_json::json!({ "items": [], "categories": [] }))
}

#[tauri::command]
fn save_portal(app_handle: tauri::AppHandle, data: serde_json::Value) -> Result<(), String> {
    let app_data_dir = app_handle.path().app_data_dir().map_err(|e| e.to_string())?;
    if !app_data_dir.exists() {
        fs::create_dir_all(&app_data_dir).map_err(|e| e.to_string())?;
    }
    let file = app_data_dir.join("portal.json");
    let content = serde_json::to_string_pretty(&data).map_err(|e| e.to_string())?;
    fs::write(&file, content).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
fn load_workspace_roots(app_handle: tauri::AppHandle) -> Result<serde_json::Value, String> {
    let app_data_dir = app_handle.path().app_data_dir().map_err(|e| e.to_string())?;
    if !app_data_dir.exists() {
        fs::create_dir_all(&app_data_dir).map_err(|e| e.to_string())?;
    }
    let file = app_data_dir.join("workspace-roots.json");
    if file.exists() {
        let content = fs::read_to_string(&file).map_err(|e| e.to_string())?;
        let val: serde_json::Value = serde_json::from_str(&content).map_err(|e| e.to_string())?;
        return Ok(val);
    }
    Ok(serde_json::Value::Array(vec![]))
}

#[tauri::command]
fn save_workspace_roots(app_handle: tauri::AppHandle, roots: serde_json::Value) -> Result<(), String> {
    let app_data_dir = app_handle.path().app_data_dir().map_err(|e| e.to_string())?;
    if !app_data_dir.exists() {
        fs::create_dir_all(&app_data_dir).map_err(|e| e.to_string())?;
    }
    let file = app_data_dir.join("workspace-roots.json");
    let content = serde_json::to_string_pretty(&roots).map_err(|e| e.to_string())?;
    fs::write(&file, content).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
fn create_folder(folder_path: String) -> Result<String, String> {
    let path = std::path::Path::new(&folder_path);
    if path.exists() {
        return Err("이미 존재하는 폴더입니다".to_string());
    }
    fs::create_dir_all(path).map_err(|e| e.to_string())?;
    println!("[CreateFolder] Created: {}", folder_path);
    std::process::Command::new("open")
        .arg(&folder_path)
        .spawn()
        .ok();
    Ok(folder_path)
}

#[tauri::command]
fn execute_command(
    port_id: String,
    command_path: String,
    folder_path: Option<String>,
    state: State<AppState>,
    app_handle: tauri::AppHandle,
) -> Result<String, String> {
    // 파일 경로인지 raw 커맨드인지 판별 (절대경로 = 파일, 아니면 shell 커맨드)
    let is_file_path = command_path.starts_with('/') || command_path.starts_with('~');
    let command_path_buf = std::path::PathBuf::from(&command_path);
    if is_file_path && !command_path_buf.exists() {
        println!("[ExecuteCommand] Command file not found: {}", command_path);
        return Err(format!("Command file not found: {}", command_path));
    }
    if is_file_path {
        println!("[ExecuteCommand] Command file exists: {}", command_path);
    } else {
        println!("[ExecuteCommand] Raw shell command: {}", command_path);
    }

    // .html 파일은 기본 브라우저로 열기 (open -a Chrome은 로컬 파일 경로에서 실패할 수 있음)
    if command_path.to_lowercase().ends_with(".html") {
        #[cfg(target_os = "macos")]
        {
            Command::new("open").arg(&command_path).spawn()
                .map_err(|e| e.to_string())?;
        }
        #[cfg(target_os = "windows")]
        {
            Command::new("cmd").args(["/C", "start", "", &command_path]).spawn()
                .map_err(|e| e.to_string())?;
        }
        #[cfg(not(any(target_os = "macos", target_os = "windows")))]
        {
            Command::new("xdg-open").arg(&command_path).spawn()
                .map_err(|e| e.to_string())?;
        }
        return Ok("Opened HTML file in browser".to_string());
    }

    // 로그 파일 경로 생성
    let app_data_dir = app_handle.path().app_data_dir()
        .map_err(|e| e.to_string())?;

    let logs_dir = app_data_dir.join("logs");

    // logs 디렉토리가 없으면 생성
    if !logs_dir.exists() {
        fs::create_dir_all(&logs_dir)
            .map_err(|e| format!("Failed to create logs directory: {}", e))?;
    }

    let log_file = logs_dir.join(format!("{}.log", port_id));
    println!("[ExecuteCommand] Log file: {:?}", log_file);

    // 로그 파일 열기 (append 모드)
    let log_out = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&log_file)
        .map_err(|e| format!("Failed to open log file: {}", e))?;

    let log_err = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&log_file)
        .map_err(|e| format!("Failed to open log file: {}", e))?;

    // .command 파일에 실행 권한 부여 (파일 경로인 경우만)
    if is_file_path {
        let chmod_result = Command::new("chmod")
            .arg("+x")
            .arg(&command_path)
            .output();

        match chmod_result {
            Ok(out) => {
                if out.status.success() {
                    println!("[ExecuteCommand] Successfully set execute permission");
                } else {
                    println!("[ExecuteCommand] Warning: chmod failed: {}", String::from_utf8_lossy(&out.stderr));
                }
            }
            Err(e) => {
                println!("[ExecuteCommand] Warning: chmod error: {}", e);
            }
        }
    }

    // 환경변수 설정 (GUI 앱에서 터미널 환경변수 상속)
    let home = std::env::var("HOME").unwrap_or_default();

    // PATH 환경변수에 일반적인 경로들 추가
    let path_additions = vec![
        format!("{}/.cargo/bin", home),
        format!("{}/.bun/bin", home),
        format!("{}/bin", home),
        "/usr/local/bin".to_string(),
        "/usr/bin".to_string(),
        "/bin".to_string(),
        "/usr/sbin".to_string(),
        "/sbin".to_string(),
        "/opt/homebrew/bin".to_string(),
        "/usr/local/go/bin".to_string(),
    ];

    let existing_path = std::env::var("PATH").unwrap_or_default();
    let new_path = if existing_path.is_empty() {
        path_additions.join(":")
    } else {
        format!("{}:{}", path_additions.join(":"), existing_path)
    };

    // 프로세스 실행 시 stdout, stderr를 로그 파일로 리다이렉트
    // setsid를 사용하여 새로운 세션으로 실행 (백그라운드 프로세스)
    if is_file_path {
        println!("[ExecuteCommand] Executing: bash {}", command_path);
    } else {
        println!("[ExecuteCommand] Executing: bash -c {}", command_path);
    }
    println!("[ExecuteCommand] PATH: {}", new_path);

    let mut cmd = Command::new("bash");
    if is_file_path {
        cmd.arg(&command_path);
    } else {
        cmd.arg("-c").arg(&command_path);
    }
    // raw 커맨드(terminalCommand)는 folderPath를 cwd로 설정
    if !is_file_path {
        if let Some(ref fp) = folder_path {
            if !fp.is_empty() {
                cmd.current_dir(fp);
            }
        }
    }
    cmd
        .stdout(log_out)
        .stderr(log_err)
        .env("PATH", &new_path)
        .env("HOME", &home);

    // 새로운 프로세스 그룹으로 실행 (백그라운드 데몬화) — Unix 전용
    #[cfg(unix)]
    {
        use std::os::unix::process::CommandExt;
        unsafe {
            cmd.pre_exec(|| {
                // 새로운 세션 리더가 되어 부모와 독립적으로 실행
                libc::setsid();
                Ok(())
            });
        }
    }

    let child = cmd.spawn()
        .map_err(|e| format!("Failed to spawn process: {}", e))?;

    let pid = child.id();

    let mut processes = state.processes.lock().unwrap();
    processes.insert(port_id.clone(), pid);

    println!("[ExecuteCommand] Started process with PID: {}", pid);

    Ok(format!("Started process with PID: {} (logs: {:?})", pid, log_file))
}

#[tauri::command]
fn stop_command(
    port_id: String,
    port: u16,
    state: State<AppState>,
) -> Result<String, String> {
    println!("[StopCommand] Starting stop for port_id: {}, port: {}", port_id, port);

    let mut processes = state.processes.lock().unwrap();

    // HashMap에서 PID 제거
    let pid_from_map = processes.remove(&port_id);
    drop(processes); // lock 해제

    // 포트로 실행 중인 모든 프로세스 찾기
    let mut killed_pids = Vec::new();

    #[cfg(target_os = "macos")]
    {
        let output = Command::new("lsof")
            .arg("-ti")
            .arg(format!(":{}", port))
            .output();

        match output {
            Ok(out) => {
                if out.status.success() && !out.stdout.is_empty() {
                    let pid_str = String::from_utf8_lossy(&out.stdout).trim().to_string();
                    let pids: Vec<&str> = pid_str.lines().collect();

                    if !pids.is_empty() {
                        println!("[StopCommand] Found {} PIDs on port {}: {:?}", pids.len(), port, pids);

                        // 모든 PID 종료
                        for pid_str in pids {
                            if let Ok(pid) = pid_str.parse::<u32>() {
                                println!("[StopCommand] Killing PID: {}", pid);

                                // 먼저 SIGTERM 시도
                                let term_result = Command::new("kill")
                                    .arg("-15")
                                    .arg(pid.to_string())
                                    .output();

                                match term_result {
                                    Ok(output) => {
                                        if output.status.success() {
                                            println!("[StopCommand] SIGTERM sent to PID: {}", pid);
                                            // 잠시 대기
                                            std::thread::sleep(std::time::Duration::from_millis(200));

                                            // 프로세스가 아직 살아있는지 확인
                                            let check = Command::new("kill")
                                                .arg("-0")
                                                .arg(pid.to_string())
                                                .output();

                                            if check.is_ok() && check.unwrap().status.success() {
                                                // 여전히 살아있으면 SIGKILL
                                                println!("[StopCommand] Process still alive, sending SIGKILL to PID: {}", pid);
                                                let _ = Command::new("kill")
                                                    .arg("-9")
                                                    .arg(pid.to_string())
                                                    .output();
                                            }
                                        } else {
                                            // SIGTERM 실패하면 바로 SIGKILL
                                            println!("[StopCommand] SIGTERM failed, sending SIGKILL to PID: {}", pid);
                                            let _ = Command::new("kill")
                                                .arg("-9")
                                                .arg(pid.to_string())
                                                .output();
                                        }
                                        killed_pids.push(pid);
                                    }
                                    Err(e) => {
                                        println!("[StopCommand] Error sending SIGTERM to PID {}: {}", pid, e);
                                        // 에러 나도 SIGKILL 시도
                                        let _ = Command::new("kill")
                                            .arg("-9")
                                            .arg(pid.to_string())
                                            .output();
                                        killed_pids.push(pid);
                                    }
                                }
                            }
                        }
                    } else {
                        println!("[StopCommand] No processes found on port {}", port);
                    }
                } else {
                    println!("[StopCommand] No processes found on port {} (lsof returned empty)", port);
                }
            }
            Err(e) => {
                println!("[StopCommand] Error running lsof: {}", e);
            }
        }
    }

    #[cfg(not(target_os = "macos"))]
    {
        if let Some(pid) = pid_from_map {
            let _ = Command::new("kill")
                .arg("-9")
                .arg(pid.to_string())
                .spawn();
            killed_pids.push(pid);
        }
    }

    if killed_pids.is_empty() {
        if pid_from_map.is_some() {
            println!("[StopCommand] Process from map was removed but not found on port");
            Ok(format!("Process stopped (was in tracking map)"))
        } else {
            println!("[StopCommand] No process found on port {} (already stopped)", port);
            Ok(format!("No process running on port {} (already stopped)", port))
        }
    } else {
        println!("[StopCommand] Successfully stopped {} process(es): {:?}", killed_pids.len(), killed_pids);
        Ok(format!("Stopped {} process(es) with PIDs: {:?}", killed_pids.len(), killed_pids))
    }
}

#[tauri::command]
fn force_restart_command(
    port_id: String,
    port: u16,
    command_path: String,
    state: State<AppState>,
    app_handle: tauri::AppHandle,
) -> Result<String, String> {
    println!("[ForceRestart] Starting force restart for port_id: {}, port: {}", port_id, port);

    // .html 파일은 기본 브라우저로 열기
    if command_path.to_lowercase().ends_with(".html") {
        #[cfg(target_os = "macos")]
        {
            Command::new("open").arg(&command_path).spawn()
                .map_err(|e| e.to_string())?;
        }
        #[cfg(target_os = "windows")]
        {
            Command::new("cmd").args(["/C", "start", "", &command_path]).spawn()
                .map_err(|e| e.to_string())?;
        }
        #[cfg(not(any(target_os = "macos", target_os = "windows")))]
        {
            Command::new("xdg-open").arg(&command_path).spawn()
                .map_err(|e| e.to_string())?;
        }
        return Ok("Opened HTML file in browser".to_string());
    }

    // 1단계: 포트로 실행 중인 모든 프로세스 강제 종료
    #[cfg(target_os = "macos")]
    {
        let output = Command::new("lsof")
            .arg("-ti")
            .arg(format!(":{}", port))
            .output();

        match output {
            Ok(out) => {
                if out.status.success() && !out.stdout.is_empty() {
                    let pid_str = String::from_utf8_lossy(&out.stdout).trim().to_string();
                    let pids: Vec<&str> = pid_str.lines().collect();

                    for pid_str in pids {
                        if let Ok(pid) = pid_str.parse::<u32>() {
                            println!("[ForceRestart] Force killing PID: {}", pid);
                            // SIGKILL로 즉시 강제 종료
                            let _ = Command::new("kill")
                                .arg("-9")
                                .arg(pid.to_string())
                                .output();
                        }
                    }
                }
            }
            Err(e) => {
                println!("[ForceRestart] Error running lsof: {}", e);
            }
        }
    }

    // HashMap에서도 제거
    let mut processes = state.processes.lock().unwrap();
    processes.remove(&port_id);
    drop(processes); // lock 해제

    // 잠시 대기 (프로세스가 완전히 종료될 시간)
    std::thread::sleep(std::time::Duration::from_millis(500));

    // 2단계: 새로운 프로세스 시작
    // 파일 경로인지 raw 커맨드인지 판별
    let is_file_path = command_path.starts_with('/') || command_path.starts_with('~');
    let command_path_buf = std::path::PathBuf::from(&command_path);
    if is_file_path && !command_path_buf.exists() {
        return Err(format!("Command file not found: {}", command_path));
    }
    if is_file_path {
        println!("[ForceRestart] Command file exists: {}", command_path);
    } else {
        println!("[ForceRestart] Raw shell command: {}", command_path);
    }

    let app_data_dir = app_handle.path().app_data_dir()
        .map_err(|e| e.to_string())?;

    let logs_dir = app_data_dir.join("logs");

    // logs 디렉토리가 없으면 생성
    if !logs_dir.exists() {
        fs::create_dir_all(&logs_dir)
            .map_err(|e| format!("Failed to create logs directory: {}", e))?;
    }

    let log_file = logs_dir.join(format!("{}.log", port_id));
    println!("[ForceRestart] Log file: {:?}", log_file);

    // 로그 파일 열기 (append 모드)
    let log_out = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&log_file)
        .map_err(|e| format!("Failed to open log file: {}", e))?;

    let log_err = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&log_file)
        .map_err(|e| format!("Failed to open log file: {}", e))?;

    // .command 파일에 실행 권한 부여
    let chmod_result = Command::new("chmod")
        .arg("+x")
        .arg(&command_path)
        .output();

    match chmod_result {
        Ok(out) => {
            if out.status.success() {
                println!("[ForceRestart] Successfully set execute permission");
            } else {
                println!("[ForceRestart] Warning: chmod failed: {}", String::from_utf8_lossy(&out.stderr));
            }
        }
        Err(e) => {
            println!("[ForceRestart] Warning: chmod error: {}", e);
        }
    }

    // 환경변수 설정 (GUI 앱에서 터미널 환경변수 상속)
    let home = std::env::var("HOME").unwrap_or_default();

    // PATH 환경변수에 일반적인 경로들 추가
    let path_additions = vec![
        format!("{}/.cargo/bin", home),
        format!("{}/.bun/bin", home),
        format!("{}/bin", home),
        "/usr/local/bin".to_string(),
        "/usr/bin".to_string(),
        "/bin".to_string(),
        "/usr/sbin".to_string(),
        "/sbin".to_string(),
        "/opt/homebrew/bin".to_string(),
        "/usr/local/go/bin".to_string(),
    ];

    let existing_path = std::env::var("PATH").unwrap_or_default();
    let new_path = if existing_path.is_empty() {
        path_additions.join(":")
    } else {
        format!("{}:{}", path_additions.join(":"), existing_path)
    };

    // 프로세스 실행 시 stdout, stderr를 로그 파일로 리다이렉트
    // setsid를 사용하여 새로운 세션으로 실행 (백그라운드 프로세스)
    if is_file_path {
        println!("[ForceRestart] Executing: bash {}", command_path);
    } else {
        println!("[ForceRestart] Executing: bash -c {}", command_path);
    }
    println!("[ForceRestart] PATH: {}", new_path);

    let mut cmd = Command::new("bash");
    if is_file_path {
        cmd.arg(&command_path);
    } else {
        cmd.arg("-c").arg(&command_path);
    }
    cmd
        .stdout(log_out)
        .stderr(log_err)
        .env("PATH", &new_path)
        .env("HOME", &home);

    // 새로운 프로세스 그룹으로 실행 (백그라운드 데몬화) — Unix 전용
    #[cfg(unix)]
    {
        use std::os::unix::process::CommandExt;
        unsafe {
            cmd.pre_exec(|| {
                // 새로운 세션 리더가 되어 부모와 독립적으로 실행
                libc::setsid();
                Ok(())
            });
        }
    }

    let child = cmd.spawn()
        .map_err(|e| format!("Failed to spawn process: {}", e))?;

    let new_pid = child.id();

    let mut processes = state.processes.lock().unwrap();
    processes.insert(port_id.clone(), new_pid);

    println!("[ForceRestart] Successfully restarted with new PID: {}", new_pid);

    Ok(format!("Force restarted on port {} with new PID: {}", port, new_pid))
}

#[tauri::command]
fn check_port_status(port: u16) -> Result<bool, String> {
    #[cfg(target_os = "macos")]
    {
        let output = Command::new("lsof")
            .arg("-ti")
            .arg(format!(":{}", port))
            .output();

        match output {
            Ok(out) => {
                let is_running = out.status.success() && !out.stdout.is_empty();
                println!("[CheckPort] Port {} is {}", port, if is_running { "RUNNING" } else { "NOT running" });
                Ok(is_running)
            }
            Err(e) => {
                println!("[CheckPort] Error checking port {}: {}", port, e);
                Ok(false) // 에러가 나면 실행 중이 아닌 것으로 간주
            }
        }
    }

    #[cfg(not(target_os = "macos"))]
    {
        Ok(false)
    }
}

#[tauri::command]
fn detect_port(file_path: String) -> Result<Option<u16>, String> {
    let content = fs::read_to_string(&file_path)
        .map_err(|e| e.to_string())?;

    // localhost:포트 패턴 검색
    if let Some(caps) = regex::Regex::new(r"localhost:(\d+)")
        .unwrap()
        .captures(&content) {
        if let Some(port_str) = caps.get(1) {
            if let Ok(port) = port_str.as_str().parse::<u16>() {
                return Ok(Some(port));
            }
        }
    }

    // PORT=포트 또는 port=포트 패턴 검색
    if let Some(caps) = regex::Regex::new(r"(?:PORT|port)\s*=\s*(\d+)")
        .unwrap()
        .captures(&content) {
        if let Some(port_str) = caps.get(1) {
            if let Ok(port) = port_str.as_str().parse::<u16>() {
                return Ok(Some(port));
            }
        }
    }

    Ok(None)
}

#[tauri::command]
fn check_file_exists(path: String) -> bool {
    std::path::Path::new(&path).exists()
}

#[tauri::command]
fn open_build_folder() -> Result<String, String> {
    let home = std::env::var("HOME").unwrap_or_default();
    // .cargo/config.toml의 target-dir 설정과 동일한 경로
    let dmg_folder = format!("{}/cargo-targets/portmanager/release/bundle/dmg", home);

    Command::new("open")
        .arg(&dmg_folder)
        .spawn()
        .map_err(|e| e.to_string())?;

    Ok("폴더를 열었습니다".to_string())
}

#[tauri::command]
fn export_dmg() -> Result<String, String> {
    use std::path::Path;

    let home = std::env::var("HOME").unwrap_or_default();
    // .cargo/config.toml의 target-dir 설정과 동일한 경로
    let bundle_dir = format!("{}/cargo-targets/portmanager/release/bundle", home);

    // DMG 파일 찾기
    let dmg_paths = vec![
        format!("{}/dmg 2", bundle_dir),
        format!("{}/dmg", bundle_dir),
        format!("{}/macos", bundle_dir),
    ];

    let mut dmg_file: Option<String> = None;

    'outer: for dmg_dir in dmg_paths {
        if let Ok(entries) = fs::read_dir(&dmg_dir) {
            let mut candidates: Vec<(std::time::SystemTime, String)> = entries
                .flatten()
                .filter_map(|e| {
                    let p = e.path();
                    let name = p.file_name()?.to_str()?.to_string();
                    if p.extension()? == "dmg" && !name.starts_with("rw.") {
                        let mtime = p.metadata().ok()?.modified().ok()?;
                        Some((mtime, p.to_string_lossy().to_string()))
                    } else {
                        None
                    }
                })
                .collect();
            if !candidates.is_empty() {
                candidates.sort_by(|a, b| b.0.cmp(&a.0)); // 최신순
                dmg_file = Some(candidates.remove(0).1);
                break 'outer;
            }
        }
    }

    match dmg_file {
        Some(dmg_path) => {
            let home = std::env::var("HOME").unwrap_or_default();
            let desktop = format!("{}/Desktop", home);

            // 원본 파일명 추출 후 vN 형식으로 단순화
            // "CS_Manager_51.0.0_aarch64.dmg" → "CS_Manager_v51.dmg"
            let dmg_filename = Path::new(&dmg_path)
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("CS_Manager.dmg");

            let release_name = {
                let base = dmg_filename.trim_end_matches(".dmg");
                let parts: Vec<&str> = base.split('_').collect();
                let mut found = None;
                let mut product_end = parts.len();
                for (i, part) in parts.iter().enumerate() {
                    let segs: Vec<&str> = part.split('.').collect();
                    if segs.len() == 3 && segs.iter().all(|s| s.parse::<u64>().is_ok()) {
                        found = Some(segs[0].to_string());
                        product_end = i;
                        break;
                    }
                }
                if let Some(major) = found {
                    format!("{}_v{}.dmg", parts[..product_end].join("_"), major)
                } else {
                    dmg_filename.to_string()
                }
            };

            let dest_path = format!("{}/{}", desktop, release_name);

            // 기존 파일이 있으면 삭제
            if Path::new(&dest_path).exists() {
                fs::remove_file(&dest_path)
                    .map_err(|e| format!("기존 파일 삭제 실패: {}", e))?;
            }

            // DMG 복사
            fs::copy(&dmg_path, &dest_path)
                .map_err(|e| format!("DMG 복사 실패: {}", e))?;

            // Desktop 폴더 열기
            Command::new("open")
                .arg(&desktop)
                .spawn()
                .map_err(|e| e.to_string())?;

            Ok(format!("DMG를 Desktop에 복사했습니다: {}", dest_path))
        }
        None => Err("DMG 파일을 찾을 수 없습니다. 먼저 빌드를 실행하세요.".to_string())
    }
}

#[tauri::command]
fn open_folder(folder_path: String) -> Result<String, String> {
    if folder_path.is_empty() {
        return Err("폴더 경로가 비어 있습니다".to_string());
    }
    if !is_absolute_path(&folder_path) {
        return Err(format!("절대 경로가 필요합니다: \"{}\"", folder_path));
    }
    if !std::path::Path::new(&folder_path).exists() {
        return Err(format!("폴더를 찾을 수 없습니다: \"{}\"", folder_path));
    }

    Command::new("open")
        .arg(&folder_path)
        .spawn()
        .map_err(|e| format!("폴더 열기 실패: {}", e))?;

    Ok(format!("폴더를 열었습니다: {}", folder_path))
}

#[tauri::command]
fn open_log(port_id: String, app_handle: tauri::AppHandle) -> Result<String, String> {
    // 로그 파일 경로 생성
    let app_data_dir = app_handle.path().app_data_dir()
        .map_err(|e| e.to_string())?;

    let logs_dir = app_data_dir.join("logs");

    // logs 디렉토리가 없으면 생성
    if !logs_dir.exists() {
        fs::create_dir_all(&logs_dir)
            .map_err(|e| format!("Failed to create logs directory: {}", e))?;
    }

    let log_file = logs_dir.join(format!("{}.log", port_id));

    // 로그 파일이 없으면 생성
    if !log_file.exists() {
        fs::write(&log_file, "로그가 아직 생성되지 않았습니다.\n")
            .map_err(|e| format!("Failed to create log file: {}", e))?;
    }

    println!("[OpenLog] Opening log file: {:?}", log_file);

    #[cfg(target_os = "macos")]
    {
        let log_path_str = log_file.to_string_lossy().to_string();
        // `create window with default profile command` 방식: write text와 달리 클립보드 미사용
        let sq_escaped = log_path_str.replace('\'', "'\\''");
        let script = format!(
            "tell application \"iTerm\"\n  activate\n  create window with default profile command \"tail -f '{}'\"\nend tell",
            sq_escaped
        );
        let result = Command::new("osascript")
            .arg("-e")
            .arg(&script)
            .output();
        match result {
            Ok(out) if !out.status.success() => {
                // iTerm 실패 시 Terminal.app으로 폴백
                let fallback = format!(
                    "tell application \"Terminal\"\n  do script \"tail -f '{}'\"\n  activate\nend tell",
                    sq_escaped
                );
                Command::new("osascript")
                    .arg("-e")
                    .arg(&fallback)
                    .spawn()
                    .map_err(|e| format!("Failed to open Terminal: {}", e))?;
            }
            Err(e) => return Err(format!("Failed to open iTerm: {}", e)),
            _ => {}
        }
    }

    #[cfg(target_os = "windows")]
    {
        // Windows: WSL bash로 tail -f (Windows Terminal 사용)
        let log_path_str = log_file.to_string_lossy().to_string();
        let wsl_path = win_to_wsl_path(&log_path_str);
        let bash_cmd = format!("tail -f '{}'", escape_sq(&wsl_path));
        spawn_wt_wsl(&bash_cmd, Some("Log Viewer"))?;
    }

    #[cfg(not(any(target_os = "macos", target_os = "windows")))]
    {
        Command::new("xdg-open")
            .arg(log_file.to_string_lossy().to_string())
            .spawn()
            .map_err(|e| format!("Failed to open log file: {}", e))?;
    }

    Ok(format!("로그 파일을 열었습니다: {:?}", log_file))
}

#[tauri::command]
fn read_log_content(port_id: String, offset: usize, app_handle: tauri::AppHandle) -> Result<serde_json::Value, String> {
    let app_data_dir = app_handle.path().app_data_dir()
        .map_err(|e| e.to_string())?;

    let logs_dir = app_data_dir.join("logs");
    let log_file = logs_dir.join(format!("{}.log", port_id));

    if !log_file.exists() {
        return Ok(serde_json::json!({
            "content": "",
            "size": 0,
            "exists": false
        }));
    }

    let content = fs::read_to_string(&log_file)
        .map_err(|e| format!("Failed to read log file: {}", e))?;

    let size = content.len();
    // Find safe UTF-8 char boundary at or after offset to avoid panic on multi-byte chars
    let safe_offset = if offset > 0 && offset < size {
        (offset..=size).find(|&i| content.is_char_boundary(i)).unwrap_or(size)
    } else {
        0
    };
    let sliced = if safe_offset > 0 { &content[safe_offset..] } else { &content };

    Ok(serde_json::json!({
        "content": sliced,
        "size": size,
        "exists": true,
        "offset": offset
    }))
}

/// Escape single quotes for use inside single-quoted shell strings.
/// ' → '\'' (end-quote, literal-apostrophe, re-open-quote)
fn escape_sq(s: &str) -> String {
    s.replace("'", "'\\''")
}

#[cfg(target_os = "windows")]
fn win_to_wsl_path(path: &str) -> String {
    let bytes = path.as_bytes();
    if bytes.len() >= 2 && bytes[1] == b':' {
        let drive = path.chars().next().unwrap().to_ascii_lowercase();
        let rest = path[2..].replace('\\', "/");
        format!("/mnt/{}{}", drive, rest)
    } else {
        path.replace('\\', "/")
    }
}

// Windows 레지스트리에서 WSL distro 목록 조회 (WSL 서비스 불필요 — 즉시 응답)
fn find_wsl_distro() -> Option<String> {
    let out = Command::new("powershell")
        .args(["-NoProfile", "-Command",
            "Get-ChildItem HKCU:/Software/Microsoft/Windows/CurrentVersion/Lxss | ForEach-Object { (Get-ItemProperty $_.PSPath).DistributionName }"])
        .output().ok()?;
    let text = String::from_utf8_lossy(&out.stdout).to_string();
    for line in text.lines() {
        let name = line.trim();
        if name.is_empty() || name.to_lowercase().contains("docker") { continue; }
        return Some(name.to_string());
    }
    None
}

fn spawn_wt_wsl(bash_cmd: &str, title: Option<&str>) -> Result<(), String> {
    let distro = find_wsl_distro().ok_or_else(|| "WSL Ubuntu distro를 찾을 수 없습니다.".to_string())?;
    let has_wt = Command::new("where")
        .args(["wt.exe"])
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status()
        .map(|s| s.success())
        .unwrap_or(false);
    if has_wt {
        let mut cmd = Command::new("wt.exe");
        if let Some(t) = title { cmd.args(["--title", t]); }
        cmd.args(["wsl", "-d", &distro, "--", "bash", "-c", bash_cmd])
            .spawn()
            .map_err(|e| format!("Windows Terminal 실행 실패: {}", e))?;
    } else {
        let mut cmd = Command::new("cmd.exe");
        if let Some(t) = title {
            // `start "title" ...` 에서 첫 인자는 창 타이틀로 취급됨
            cmd.args(["/c", "start", t, "wsl", "-d", &distro, "--", "bash", "-c", bash_cmd]);
        } else {
            cmd.args(["/c", "start", "wsl", "-d", &distro, "--", "bash", "-c", bash_cmd]);
        }
        cmd.spawn()
            .map_err(|e| format!("WSL 터미널 실행 실패: {}", e))?;
    }
    Ok(())
}

/// 창/탭 타이틀 빌더: 이모지 prefix + 프로젝트명 › 워크트리
/// ⚡️ tmux+bypass  🔷🆕 tmux+fresh  🔷 tmux  🛡️ bypass  🪟 normal
fn build_window_title(session: &str, worktree_path: Option<&str>, is_tmux: bool, is_bypass: bool, is_fresh: bool) -> String {
    let wt_name = worktree_path
        .and_then(|wt| wt.split(',').next())
        .map(|p| p.trim())
        .filter(|p| !p.is_empty())
        .map(|p| path_basename(p));
    let base = match wt_name {
        Some(n) => format!("{} \u{203A} {}", session, n),
        None => session.to_string(),
    };
    let prefix = match (is_tmux, is_bypass, is_fresh) {
        (true, true, _)      => "\u{26A1}\u{FE0F} ",
        (true, false, true)  => "\u{1F537}\u{1F195} ",
        (true, false, false) => "\u{1F537} ",
        (false, true, _)     => "\u{1F6E1}\u{FE0F} ",
        _                    => "\u{1FA9F} ",
    };
    format!("{}{}", prefix, base)
}

#[tauri::command]
fn check_wsl() -> Result<serde_json::Value, String> {
    #[cfg(target_os = "windows")]
    {
        let wsl_exists = Command::new("where")
            .arg("wsl.exe")
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .status()
            .map(|s| s.success())
            .unwrap_or(false);
        if !wsl_exists {
            return Ok(serde_json::json!({ "status": "not_installed" }));
        }
        // bash timeout이 Windows/WSL에서 불가능 → 목록 확인만으로 판단
        let distro = match find_wsl_distro() {
            Some(d) => d,
            None => return Ok(serde_json::json!({ "status": "no_distro" })),
        };
        let _ = distro;
        return Ok(serde_json::json!({ "status": "ready" }));
    }
    #[cfg(not(target_os = "windows"))]
    Ok(serde_json::json!({ "status": "ready" }))
}

#[tauri::command]
fn install_wsl() -> Result<String, String> {
    #[cfg(target_os = "windows")]
    {
        Command::new("powershell")
            .args([
                "-Command",
                "Start-Process powershell -Verb RunAs -ArgumentList '-NoExit', '-Command', 'wsl --install; Write-Host \"설치 완료. PC를 재시작하세요.\"; pause'"
            ])
            .spawn()
            .map_err(|e| format!("관리자 PowerShell 실행 실패: {}", e))?;
        return Ok("WSL2 설치 창이 열렸습니다. UAC 허용 후 설치가 시작됩니다.".to_string());
    }
    #[cfg(not(target_os = "windows"))]
    Ok("".to_string())
}

#[tauri::command]
fn install_wsl_tmux() -> Result<String, String> {
    #[cfg(target_os = "windows")]
    {
        let distro = find_wsl_distro().ok_or_else(|| "Ubuntu WSL distro를 찾을 수 없습니다.".to_string())?;
        // root인 경우 sudo 불필요
        let whoami = Command::new("wsl").args(["-d", &distro, "--", "bash", "-c", "whoami"]).output().ok();
        let is_root = whoami.as_ref().map(|o| String::from_utf8_lossy(&o.stdout).trim() == "root").unwrap_or(false);
        let install_cmd = if is_root {
            "apt-get update -qq && apt-get install -y tmux"
        } else {
            "sudo apt-get update -qq && sudo apt-get install -y tmux"
        };
        let out = Command::new("wsl")
            .args(["-d", &distro, "--", "bash", "-c", install_cmd])
            .output()
            .map_err(|e| e.to_string())?;
        if out.status.success() {
            return Ok("tmux 설치 완료".to_string());
        }
        let stderr = String::from_utf8_lossy(&out.stderr);
        return Err(format!("tmux 설치 실패: {}", stderr));
    }
    #[cfg(not(target_os = "windows"))]
    Ok("".to_string())
}

/// macOS: 임시 스크립트 파일로 iTerm을 열어 클립보드 오염 없이 명령 실행
#[cfg(target_os = "macos")]
fn open_iterm_with_script(cmd: &str) -> Result<(), String> {
    use std::time::{SystemTime, UNIX_EPOCH};
    let ts = SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_millis();
    let script_path = format!("/tmp/portmanager_{}.sh", ts);
    fs::write(&script_path, format!("#!/bin/zsh -l\n{}\n", cmd))
        .map_err(|e| format!("Failed to write script: {}", e))?;
    let _ = Command::new("chmod").args(["+x", &script_path]).output();
    let sq_path = script_path.replace('\'', "'\\''");
    let applescript = format!(
        "tell application \"iTerm\"\n  activate\n  create window with default profile command \"/bin/zsh -l '{}'\"\nend tell",
        sq_path
    );
    Command::new("osascript")
        .arg("-e")
        .arg(&applescript)
        .spawn()
        .map_err(|e| format!("Failed to open iTerm: {}", e))?;
    Ok(())
}

#[tauri::command]
fn open_tmux_claude(session_name: String, folder_path: Option<String>, worktree_path: Option<String>) -> Result<String, String> {
    #[cfg(target_os = "macos")]
    {
        let esc_session = escape_sq(&session_name);
        let esc_display = escape_sq(&session_name);
        let title = build_window_title(&session_name, worktree_path.as_deref(), true, false, false);
        let esc_title_sq = escape_sq(&title);
        let escaped_title = title.replace('\\', "\\\\").replace('"', "\\\"");
        let cd_target = worktree_path.as_ref()
            .and_then(|wt| wt.split(',').next().map(|p| p.trim().to_string()))
            .filter(|p| p.starts_with('/'))
            .or_else(|| folder_path.clone());
        let cmd = if let Some(ref cd) = cd_target {
            format!("cd '{}' && printf '\\033]0;{}\\007'; tmux new-session -d -s '{}' -n '{}' \"claude\" 2>/dev/null || true; tmux set-option -g set-titles on 2>/dev/null; tmux set-option -g set-titles-string '#W' 2>/dev/null; tmux set-window-option -t '{}' automatic-rename off 2>/dev/null; tmux rename-window -t '{}' '{}' 2>/dev/null; tmux attach-session -t '{}'", escape_sq(cd), esc_title_sq, esc_session, esc_display, esc_session, esc_session, esc_display, esc_session)
        } else {
            format!("printf '\\033]0;{}\\007'; tmux new-session -d -s '{}' -n '{}' \"claude\" 2>/dev/null || true; tmux set-option -g set-titles on 2>/dev/null; tmux set-option -g set-titles-string '#W' 2>/dev/null; tmux set-window-option -t '{}' automatic-rename off 2>/dev/null; tmux rename-window -t '{}' '{}' 2>/dev/null; tmux attach-session -t '{}'", esc_title_sq, esc_session, esc_display, esc_session, esc_session, esc_display, esc_session)
        };

        open_iterm_with_script(&cmd)?;
    }

    #[cfg(target_os = "windows")]
    {
        let cd_path = worktree_path.as_ref()
            .and_then(|wt| wt.split(',').next().map(|p| p.trim().to_string()))
            .or_else(|| folder_path.clone())
            .map(|p| win_to_wsl_path(&p));
        let cd_part = cd_path.map(|p| format!("cd '{}' && ", escape_sq(&p))).unwrap_or_default();
        let bash_cmd = format!("{}tmux new-session -A -s '{}' 'claude || bash -l'", cd_part, escape_sq(&session_name));
        let title = build_window_title(&session_name, worktree_path.as_deref(), true, false, false);
        spawn_wt_wsl(&bash_cmd, Some(&title))?;
    }

    Ok(format!("tmux + Claude 실행 중 (세션: {})", session_name))
}

#[tauri::command]
fn open_tmux_claude_fresh(session_name: String, folder_path: Option<String>, worktree_path: Option<String>, bypass: Option<bool>) -> Result<String, String> {
    let bypass = bypass.unwrap_or(false);
    let claude_cli = if bypass { "claude --dangerously-skip-permissions" } else { "claude" };
    #[cfg(target_os = "macos")]
    {
        let esc_session = escape_sq(&session_name);
        let esc_display = escape_sq(&session_name);
        let title = build_window_title(&session_name, worktree_path.as_deref(), true, bypass, true);
        let esc_title_sq = escape_sq(&title);
        let escaped_title = title.replace('\\', "\\\\").replace('"', "\\\"");
        let kill_cmd = format!("tmux kill-session -t '{}' 2>/dev/null || true", esc_session);
        let cd_target = worktree_path.as_ref()
            .and_then(|wt| wt.split(',').next().map(|p| p.trim().to_string()))
            .filter(|p| p.starts_with('/'))
            .or_else(|| folder_path.clone());
        let new_cmd = if let Some(ref cd) = cd_target {
            format!("cd '{}' && printf '\\033]0;{}\\007'; tmux new-session -d -s '{}' -n '{}' \"zsh -l -c '{}'\"; tmux set-option -g set-titles on 2>/dev/null; tmux set-option -g set-titles-string '#W' 2>/dev/null; tmux set-window-option -t '{}' automatic-rename off 2>/dev/null; tmux rename-window -t '{}' '{}' 2>/dev/null; tmux attach-session -t '{}'", escape_sq(cd), esc_title_sq, esc_session, esc_display, claude_cli, esc_session, esc_session, esc_display, esc_session)
        } else {
            format!("printf '\\033]0;{}\\007'; tmux new-session -d -s '{}' -n '{}' \"zsh -l -c '{}'\"; tmux set-option -g set-titles on 2>/dev/null; tmux set-option -g set-titles-string '#W' 2>/dev/null; tmux set-window-option -t '{}' automatic-rename off 2>/dev/null; tmux rename-window -t '{}' '{}' 2>/dev/null; tmux attach-session -t '{}'", esc_title_sq, esc_session, esc_display, claude_cli, esc_session, esc_session, esc_display, esc_session)
        };
        let cmd = format!("{}; {}", kill_cmd, new_cmd);
        open_iterm_with_script(&cmd)?;
    }

    #[cfg(target_os = "windows")]
    {
        let cd_path = worktree_path.as_ref()
            .and_then(|wt| wt.split(',').next().map(|p| p.trim().to_string()))
            .or_else(|| folder_path.clone())
            .map(|p| win_to_wsl_path(&p));
        let cd_part = cd_path.map(|p| format!("cd '{}' && ", escape_sq(&p))).unwrap_or_default();
        let claude_arg = if bypass { "claude --dangerously-skip-permissions || bash -l" } else { "claude || bash -l" };
        let bash_cmd = format!(
            "{}(tmux kill-session -t '{}' 2>/dev/null || :) && tmux new-session -s '{}' '{}'",
            cd_part, escape_sq(&session_name), escape_sq(&session_name), claude_arg
        );
        let title = build_window_title(&session_name, worktree_path.as_deref(), true, bypass, true);
        spawn_wt_wsl(&bash_cmd, Some(&title))?;
    }

    Ok(format!("tmux 새 세션 시작 (세션: {})", session_name))
}

#[tauri::command]
fn open_tmux_claude_bypass(session_name: String, folder_path: Option<String>, worktree_path: Option<String>) -> Result<String, String> {
    #[cfg(target_os = "macos")]
    {
        let esc_session = escape_sq(&session_name);
        let esc_display = escape_sq(&session_name);
        let title = build_window_title(&session_name, worktree_path.as_deref(), true, true, false);
        let esc_title_sq = escape_sq(&title);
        let escaped_title = title.replace('\\', "\\\\").replace('"', "\\\"");
        let cd_target = worktree_path.as_ref()
            .and_then(|wt| wt.split(',').next().map(|p| p.trim().to_string()))
            .filter(|p| p.starts_with('/'))
            .or_else(|| folder_path.clone());
        let cmd = if let Some(ref cd) = cd_target {
            format!("cd '{}' && printf '\\033]0;{}\\007'; tmux new-session -d -s '{}-bypass' -n '{}' \"zsh -l -c 'claude --dangerously-skip-permissions'\" 2>/dev/null || true; tmux set-option -g set-titles on 2>/dev/null; tmux set-option -g set-titles-string '#W' 2>/dev/null; tmux set-window-option -t '{}-bypass' automatic-rename off 2>/dev/null; tmux rename-window -t '{}-bypass' '{}' 2>/dev/null; tmux attach-session -t '{}-bypass'", escape_sq(cd), esc_title_sq, esc_session, esc_display, esc_session, esc_session, esc_display, esc_session)
        } else {
            format!("printf '\\033]0;{}\\007'; tmux new-session -d -s '{}-bypass' -n '{}' \"zsh -l -c 'claude --dangerously-skip-permissions'\" 2>/dev/null || true; tmux set-option -g set-titles on 2>/dev/null; tmux set-option -g set-titles-string '#W' 2>/dev/null; tmux set-window-option -t '{}-bypass' automatic-rename off 2>/dev/null; tmux rename-window -t '{}-bypass' '{}' 2>/dev/null; tmux attach-session -t '{}-bypass'", esc_title_sq, esc_session, esc_display, esc_session, esc_session, esc_display, esc_session)
        };
        open_iterm_with_script(&cmd)?;
    }

    #[cfg(target_os = "windows")]
    {
        let cd_path = worktree_path.as_ref()
            .and_then(|wt| wt.split(',').next().map(|p| p.trim().to_string()))
            .or_else(|| folder_path.clone())
            .map(|p| win_to_wsl_path(&p));
        let cd_part = cd_path.map(|p| format!("cd '{}' && ", escape_sq(&p))).unwrap_or_default();
        let bypass_session = format!("{}-bypass", escape_sq(&session_name));
        let bash_cmd = format!(
            "{}tmux new-session -A -s '{}' 'claude --dangerously-skip-permissions || bash -l'",
            cd_part, bypass_session
        );
        let title = build_window_title(&session_name, worktree_path.as_deref(), true, true, false);
        spawn_wt_wsl(&bash_cmd, Some(&title))?;
    }

    Ok(format!("tmux + Claude (bypass) 실행 중 (세션: {}-bypass)", session_name))
}

/// Windows: wt.exe + cmd.exe로 claude 실행 (tmux 없이). workDir이 있으면 -d로 시작 디렉터리 지정
#[cfg(target_os = "windows")]
fn spawn_wt_cmd(shell_cmd: &str, work_dir: Option<&str>, title: &str) -> Result<(), String> {
    let has_wt = Command::new("where")
        .args(["wt.exe"])
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status()
        .map(|s| s.success())
        .unwrap_or(false);
    if has_wt {
        let mut cmd = Command::new("cmd.exe");
        cmd.args(["/c", "start", "wt"]);
        if let Some(d) = work_dir { cmd.args(["-d", d]); }
        cmd.args(["--title", title, "--", "cmd", "/k", shell_cmd]);
        cmd.spawn().map_err(|e| format!("Windows Terminal 실행 실패: {}", e))?;
    } else {
        let cd_part = work_dir.map(|d| format!("cd /d \"{}\" && ", d)).unwrap_or_default();
        Command::new("cmd")
            .args(["/c", "start", title, "cmd", "/k", &format!("{}{}", cd_part, shell_cmd)])
            .spawn()
            .map_err(|e| format!("cmd 실행 실패: {}", e))?;
    }
    Ok(())
}

#[tauri::command]
fn open_terminal_claude_bypass(folder_path: Option<String>, name: Option<String>, worktree_path: Option<String>) -> Result<String, String> {
    #[cfg(target_os = "macos")]
    {
        let title = build_window_title(name.as_deref().unwrap_or("Claude"), worktree_path.as_deref(), false, true, false);
        let esc_title_sq = escape_sq(&title);
        let escaped_name = title.replace('\\', "\\\\").replace('"', "\\\"");
        let cd_target = worktree_path.as_ref()
            .and_then(|wt| wt.split(',').next().map(|p| p.trim().to_string()))
            .filter(|p| p.starts_with('/'))
            .or_else(|| folder_path.clone());
        let cmd = if let Some(ref cd) = cd_target {
            format!("cd '{}' && printf '\\033]0;{}\\007' && claude --dangerously-skip-permissions", escape_sq(cd), esc_title_sq)
        } else {
            format!("printf '\\033]0;{}\\007' && claude --dangerously-skip-permissions", esc_title_sq)
        };
        open_iterm_with_script(&cmd)?;
    }

    #[cfg(target_os = "windows")]
    {
        let wt_first = worktree_path.as_ref()
            .and_then(|wt| wt.split(',').next().map(|p| p.trim().to_string()))
            .filter(|p| !p.is_empty() && is_absolute_path(p));
        let work_dir = wt_first.or_else(|| folder_path.clone());
        let title = build_window_title(name.as_deref().unwrap_or("Claude"), worktree_path.as_deref(), false, true, false);
        spawn_wt_cmd("claude --dangerously-skip-permissions", work_dir.as_deref(), &title)?;
    }
    Ok("Claude (bypass) 실행".to_string())
}

#[tauri::command]
fn open_terminal_claude(folder_path: Option<String>, name: Option<String>, worktree_path: Option<String>) -> Result<String, String> {
    #[cfg(target_os = "macos")]
    {
        let title = build_window_title(name.as_deref().unwrap_or("Claude"), worktree_path.as_deref(), false, false, false);
        let esc_title_sq = escape_sq(&title);
        let escaped_name = title.replace('\\', "\\\\").replace('"', "\\\"");
        let cd_target = worktree_path.as_ref()
            .and_then(|wt| wt.split(',').next().map(|p| p.trim().to_string()))
            .filter(|p| p.starts_with('/'))
            .or_else(|| folder_path.clone());
        let cmd = if let Some(ref cd) = cd_target {
            format!("cd '{}' && printf '\\033]0;{}\\007' && claude", escape_sq(cd), esc_title_sq)
        } else {
            format!("printf '\\033]0;{}\\007' && claude", esc_title_sq)
        };
        open_iterm_with_script(&cmd)?;
    }

    #[cfg(target_os = "windows")]
    {
        let wt_first = worktree_path.as_ref()
            .and_then(|wt| wt.split(',').next().map(|p| p.trim().to_string()))
            .filter(|p| !p.is_empty() && is_absolute_path(p));
        let work_dir = wt_first.or_else(|| folder_path.clone());
        let title = build_window_title(name.as_deref().unwrap_or("Claude"), worktree_path.as_deref(), false, false, false);
        spawn_wt_cmd("claude", work_dir.as_deref(), &title)?;
    }
    Ok("Claude 실행".to_string())
}

#[tauri::command]
fn run_claude_with_prompt(folder_path: Option<String>, prompt: String) -> Result<String, String> {
    #[cfg(target_os = "macos")]
    {
        // Escape for shell single-quoted string
        let cd_part = folder_path
            .as_deref()
            .map(|fp| format!("cd '{}' && ", escape_sq(fp)))
            .unwrap_or_default();
        let cmd = format!("{}claude", cd_part);
        let escaped_cmd = cmd.replace('\\', "\\\\").replace('"', "\\\"");
        // Prompt: collapse newlines → spaces, escape for AppleScript
        let escaped_prompt = prompt
            .replace('\\', "\\\\")
            .replace('"', "\\\"")
            .replace('\n', " ");
        let script = format!(
            "tell application \"iTerm\"\n  activate\n  set newWindow to create window with default profile\n  tell current session of newWindow\n    write text \"{}\"\n    delay 4\n    write text \"{}\"\n  end tell\nend tell",
            escaped_cmd, escaped_prompt
        );
        Command::new("osascript")
            .arg("-e")
            .arg(&script)
            .spawn()
            .map_err(|e| format!("Failed to open iTerm: {}", e))?;
        Ok("iTerm에서 Claude 실행 + 프롬프트 전송".to_string())
    }
    #[cfg(not(target_os = "macos"))]
    {
        Err("macOS 전용 기능입니다".to_string())
    }
}

#[tauri::command]
fn open_in_chrome(url: String) -> Result<String, String> {
    if url.is_empty() {
        return Err("URL이 비어 있습니다".to_string());
    }

    #[cfg(target_os = "macos")]
    {
        Command::new("open")
            .arg("-a")
            .arg("Google Chrome")
            .arg(&url)
            .spawn()
            .map_err(|e| format!("Chrome 열기 실패: {}", e))?;
    }

    #[cfg(not(target_os = "macos"))]
    {
        Command::new("google-chrome")
            .arg(&url)
            .spawn()
            .map_err(|e| format!("Chrome 열기 실패: {}", e))?;
    }

    Ok(format!("Chrome에서 열었습니다: {}", url))
}

#[tauri::command]
fn import_ports_from_file(file_path: String) -> Result<Vec<PortInfo>, String> {
    // 파일이 존재하는지 확인
    let path = std::path::PathBuf::from(&file_path);
    if !path.exists() {
        return Err("파일이 존재하지 않습니다".to_string());
    }

    // 파일 읽기
    let content = fs::read_to_string(&path)
        .map_err(|e| format!("파일 읽기 실패: {}", e))?;

    // JSON 파싱
    let ports: Vec<PortInfo> = serde_json::from_str(&content)
        .map_err(|e| format!("JSON 파싱 실패: {}", e))?;

    Ok(ports)
}

#[tauri::command]
fn install_app_to_applications() -> Result<String, String> {
    let home = std::env::var("HOME").unwrap_or_default();
    // CARGO_TARGET_DIR 동적 설정 (build-macos.ts에서 $HOME/cargo-targets/portmanager로 설정)
    let app_path = format!("{}/cargo-targets/portmanager/release/bundle/macos/CS_Manager.app", home);
    let dest_path = "/Applications/CS_Manager.app";

    // 기존 앱이 있으면 삭제
    if std::path::Path::new(dest_path).exists() {
        Command::new("rm")
            .arg("-rf")
            .arg(dest_path)
            .spawn()
            .map_err(|e| format!("기존 앱 삭제 실패: {}", e))?
            .wait()
            .map_err(|e| format!("기존 앱 삭제 대기 실패: {}", e))?;
    }

    // 앱 복사
    Command::new("cp")
        .arg("-R")
        .arg(app_path)
        .arg(dest_path)
        .spawn()
        .map_err(|e| format!("앱 복사 실패: {}", e))?
        .wait()
        .map_err(|e| format!("앱 복사 대기 실패: {}", e))?;

    Ok("앱이 Applications 폴더에 설치되었습니다".to_string())
}

#[tauri::command]
async fn build_app(build_type: String, app_handle: tauri::AppHandle) -> Result<String, String> {
    let app_dir = app_handle.path().app_data_dir()
        .map_err(|e| e.to_string())?
        .parent()
        .ok_or("Cannot get parent directory")?
        .parent()
        .ok_or("Cannot get project directory")?
        .to_path_buf();

    let command = if build_type == "dmg" {
        vec!["bun", "run", "tauri:build:dmg"]
    } else {
        vec!["bun", "run", "tauri:build"]
    };

    std::thread::spawn(move || {
        let _ = Command::new(command[0])
            .args(&command[1..])
            .current_dir(app_dir)
            .spawn();
    });

    Ok(format!("{} 빌드가 백그라운드에서 시작되었습니다", build_type))
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct WorktreeInfo {
    path: String,
    branch: Option<String>,
    is_main: bool,
}

/// POSIX `/...` 과 Windows `C:\...` / `C:/...` 둘 다 절대경로로 인정
fn is_absolute_path(p: &str) -> bool {
    if p.starts_with('/') { return true; }
    let bytes = p.as_bytes();
    bytes.len() >= 3 && bytes[0].is_ascii_alphabetic() && bytes[1] == b':' && (bytes[2] == b'\\' || bytes[2] == b'/')
}

/// 경로 basename (Windows \ 와 POSIX / 둘 다 지원)
fn path_basename(p: &str) -> &str {
    p.trim_end_matches(|c| c == '/' || c == '\\')
        .rsplit(|c| c == '/' || c == '\\')
        .next()
        .unwrap_or("project")
}

#[tauri::command]
fn git_worktree_add(folder_path: String, branch_name: String, worktree_path: Option<String>) -> Result<String, String> {
    if !is_absolute_path(&folder_path) {
        return Err("folder_path must be absolute".to_string());
    }
    // Allow Unicode branch names — only strip truly invalid git branch chars
    let safe_branch: String = branch_name.chars()
        .map(|c| if c.is_whitespace() || matches!(c, '~' | '^' | ':' | '?' | '*' | '[' | '\\') { '-' } else { c })
        .collect();
    let safe_branch = safe_branch.trim_matches('-').to_string();
    // Directory name must be ASCII-only — claude -w rejects non-ASCII paths
    let dir_safe_branch: String = safe_branch.chars()
        .map(|c| if c.is_ascii_alphanumeric() || c == '.' || c == '_' || c == '-' { c } else { '-' })
        .collect();
    let dir_safe_branch = dir_safe_branch.trim_matches('-').to_string();
    let dir_safe_branch = if dir_safe_branch.is_empty() {
        format!("wt{}", std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_secs() % 1000000)
    } else { dir_safe_branch };
    let is_icloud = folder_path.contains("com~apple~CloudDocs") || folder_path.contains("Mobile Documents");
    // Windows: HOME 미설정 시 USERPROFILE 사용
    let home = std::env::var("HOME")
        .or_else(|_| std::env::var("USERPROFILE"))
        .unwrap_or_else(|_| if cfg!(windows) { "C:\\".to_string() } else { "/tmp".to_string() });
    let target = worktree_path.filter(|p| !p.is_empty()).unwrap_or_else(|| {
        let base = path_basename(&folder_path);
        format!("{}/worktrees/{}-{}", home, base, dir_safe_branch)
    });
    // Use --no-checkout on iCloud paths to avoid SIGBUS (signal 10)
    let is_icloud = folder_path.contains("com~apple~CloudDocs") || folder_path.contains("Mobile Documents");
    let mut base_args: Vec<&str> = vec!["worktree", "add"];
    if is_icloud { base_args.push("--no-checkout"); }
    // Try existing branch first
    let mut args1 = base_args.clone();
    args1.extend([target.as_str(), branch_name.as_str()]);
    let output = Command::new("git")
        .args(&args1)
        .current_dir(&folder_path)
        .output()
        .map_err(|e| format!("git not found: {}", e))?;
    if output.status.success() {
        return Ok(target);
    }
    // Fallback: create new branch
    let mut args2 = base_args.clone();
    args2.extend(["-b", branch_name.as_str(), target.as_str()]);
    let output2 = Command::new("git")
        .args(&args2)
        .current_dir(&folder_path)
        .output()
        .map_err(|e| format!("git not found: {}", e))?;
    if !output2.status.success() {
        return Err(String::from_utf8_lossy(&output2.stderr).trim().to_string());
    }
    Ok(target)
}

#[tauri::command]
fn git_worktree_remove(worktree_path: String) -> Result<(), String> {
    if !is_absolute_path(&worktree_path) {
        return Err("worktree_path must be absolute".to_string());
    }
    // Find main repo from the worktree's .git file (e.g. "gitdir: <path>/.git/worktrees/<name>")
    // Windows 경로는 / 와 \ 혼재 가능 — 두 구분자 모두 대응
    let git_file = format!("{}/.git", worktree_path);
    let main_repo_dir = std::fs::read_to_string(&git_file)
        .ok()
        .and_then(|content| {
            content.lines()
                .find_map(|l| l.strip_prefix("gitdir: ").map(|s| s.trim().to_string()))
        })
        .and_then(|gitdir| {
            // `/.git/worktrees/` 또는 `\.git\worktrees\` — 둘 다 찾아 더 앞선 것 선택
            let posix_idx = gitdir.find("/.git/worktrees/");
            let win_idx = gitdir.find("\\.git\\worktrees\\");
            let idx = match (posix_idx, win_idx) {
                (Some(a), Some(b)) => Some(a.min(b)),
                (a, b) => a.or(b),
            };
            idx.map(|i| gitdir[..i].to_string())
        })
        .unwrap_or_else(|| {
            std::path::Path::new(&worktree_path)
                .parent()
                .map(|p| p.to_string_lossy().to_string())
                .unwrap_or_else(|| if cfg!(windows) { "C:\\".to_string() } else { "/tmp".to_string() })
        });
    let run_remove = || -> Result<(bool, String), String> {
        let out = Command::new("git")
            .args(["worktree", "remove", "--force", &worktree_path])
            .current_dir(&main_repo_dir)
            .output()
            .map_err(|e| format!("git not found: {}", e))?;
        Ok((out.status.success(), String::from_utf8_lossy(&out.stderr).trim().to_string()))
    };
    let is_lock_error = |err: &str| {
        let low = err.to_lowercase();
        low.contains("permission denied") || low.contains("being used") || low.contains("ebusy")
            || low.contains("eperm") || low.contains("access is denied") || low.contains("cannot access")
            || low.contains("invalid argument") || low.contains("failed to delete")
    };

    let (mut ok, mut err) = run_remove()?;
    let mut attempts = 1;
    // 파일 락 에러면 최대 3회 재시도 (200/400/800 ms 점증)
    while !ok && is_lock_error(&err) && attempts < 3 {
        std::thread::sleep(std::time::Duration::from_millis(200u64 * (1u64 << (attempts - 1))));
        let (o, e) = run_remove()?;
        ok = o;
        err = e;
        attempts += 1;
    }

    if !ok {
        // 폴백: prune + 물리 디렉터리 강제 삭제 + 재prune
        let _ = Command::new("git").args(["worktree", "prune"]).current_dir(&main_repo_dir).output();
        if std::path::Path::new(&worktree_path).exists() {
            let _ = std::fs::remove_dir_all(&worktree_path);
        }
        let _ = Command::new("git").args(["worktree", "prune"]).current_dir(&main_repo_dir).output();
        // 검증: 메타 등록과 물리 디렉터리 모두 사라졌으면 성공 처리
        let list_out = Command::new("git")
            .args(["worktree", "list", "--porcelain"])
            .current_dir(&main_repo_dir)
            .output()
            .map_err(|e| format!("git not found: {}", e))?;
        let list_str = String::from_utf8_lossy(&list_out.stdout);
        let wt_posix = worktree_path.replace('\\', "/");
        let still_registered = list_str.contains(&format!("worktree {}", worktree_path))
            || list_str.contains(&format!("worktree {}", wt_posix));
        if !still_registered && !std::path::Path::new(&worktree_path).exists() {
            return Ok(());
        }
        // 부분 성공: git 메타는 정리됐지만 물리 디렉터리가 락 때문에 남음
        if !still_registered && std::path::Path::new(&worktree_path).exists() {
            // Tauri invoke는 Result<(), String>이라 성공으로 처리하되, 경고 로그
            eprintln!(
                "[git_worktree_remove] partial: registration removed, folder still exists (locked): {}",
                worktree_path
            );
            return Ok(());
        }
        if is_lock_error(&err) {
            return Err(format!(
                "파일이 사용 중이라 삭제하지 못했습니다. 해당 폴더를 열어둔 탐색기/터미널/에디터를 모두 닫은 뒤 다시 시도하세요.\n\n원본 에러: {}",
                err
            ));
        }
        return Err(err);
    }
    Ok(())
}

#[tauri::command]
fn git_merge_branch(folder_path: String, branch_name: String) -> Result<String, String> {
    if !is_absolute_path(&folder_path) {
        return Err("folder_path must be absolute".to_string());
    }
    // --autostash: 변경 사항 자동 스태시 후 머지, 이후 자동 팝
    let output = Command::new("git")
        .args(["merge", "--no-ff", "--no-edit", "--autostash", &branch_name])
        .current_dir(&folder_path)
        .env("GIT_EDITOR", "true")
        .env("GIT_TERMINAL_PROMPT", "0")
        .output()
        .map_err(|e| format!("git not found: {}", e))?;
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
        let msg = if stderr.contains("signal: 10") || stderr.contains("SIGBUS") {
            "iCloud 동기화로 머지 실패. Finder에서 iCloud 다운로드를 강제하거나 메인 레포를 iCloud 밖으로 이동하세요.".to_string()
        } else if stderr.contains("CONFLICT") {
            format!("충돌 발생: {}\n→ git merge --abort 로 취소 가능", stderr)
        } else {
            stderr
        };
        return Err(msg);
    }
    Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
}

#[tauri::command]
fn list_git_worktrees(folder_path: String) -> Result<Vec<WorktreeInfo>, String> {
    let output = std::process::Command::new("git")
        .args(["worktree", "list", "--porcelain"])
        .current_dir(&folder_path)
        .output()
        .map_err(|e| e.to_string())?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut worktrees = Vec::new();
    let mut current_path: Option<String> = None;
    let mut current_branch: Option<String> = None;
    let mut is_first = true;

    for line in stdout.lines() {
        if line.starts_with("worktree ") {
            if let Some(path) = current_path.take() {
                let is_main = is_first;
                if is_first { is_first = false; }
                worktrees.push(WorktreeInfo {
                    path,
                    branch: current_branch.take(),
                    is_main,
                });
            }
            current_path = Some(line["worktree ".len()..].to_string());
        } else if line.starts_with("branch refs/heads/") {
            current_branch = Some(line["branch refs/heads/".len()..].to_string());
        }
    }
    // flush last entry
    if let Some(path) = current_path {
        worktrees.push(WorktreeInfo {
            path,
            branch: current_branch,
            is_main: is_first,
        });
    }
    Ok(worktrees)
}

/// AI 이름 추천 (folderPath 기반, login shell에서 claude -p 호출)
#[tauri::command]
fn suggest_name(folder_path: String) -> Result<Vec<String>, String> {
    use std::fs;

    let path = std::path::Path::new(&folder_path);
    if !path.exists() {
        return Err(format!("폴더 없음: {}", folder_path));
    }

    // 디렉토리 파일 목록 (최대 30개)
    let files: Vec<String> = fs::read_dir(path)
        .map(|entries| {
            entries
                .filter_map(|e| e.ok())
                .map(|e| e.file_name().to_string_lossy().to_string())
                .take(30)
                .collect()
        })
        .unwrap_or_default();

    // package.json 내용 (있으면 최대 500자)
    let pkg_json = fs::read_to_string(path.join("package.json"))
        .map(|s| s.chars().take(500).collect::<String>())
        .unwrap_or_default();

    let prompt = format!(
        "Project files: {}\npackage.json: {}\n\nSuggest 3 concise project names (2-4 words, English). Reply with JSON array only: [\"name1\",\"name2\",\"name3\"]",
        files.join(", "),
        pkg_json
    );

    // login shell로 실행 — ~/.zshrc 소싱 → 올바른 PATH + claude 인증 토큰 자동 로드
    // (Tauri 직접 spawn은 Homebrew PATH / auth 환경이 없어서 claude를 못 찾거나 인증 실패)
    let escaped_prompt = prompt.replace('\'', "'\"'\"'"); // sh single-quote escape
    let shell_cmd = format!(
        "cd '{}' && claude -p '{}'",
        escape_sq(&folder_path),
        escaped_prompt
    );

    let out = std::process::Command::new("/bin/zsh")
        .args(["-l", "-c", &shell_cmd])
        .output()
        .map_err(|e| format!("shell 실행 실패: {}", e))?;

    let raw = String::from_utf8_lossy(&out.stdout).trim().to_string();
    let err_raw = String::from_utf8_lossy(&out.stderr).trim().to_string();

    // 실패 시 stderr/stdout 포함한 에러 반환 (디버깅용)
    if !out.status.success() || raw.is_empty() {
        return Err(format!("claude 실패 (exit={}) stdout='{}' stderr='{}'",
            out.status.code().unwrap_or(-1),
            &raw[..raw.len().min(300)],
            &err_raw[..err_raw.len().min(300)]));
    }

    // JSON 배열 추출
    if let Some(start) = raw.find('[') {
        if let Some(end) = raw.rfind(']') {
            let json_str = &raw[start..=end];
            if let Ok(suggestions) = serde_json::from_str::<Vec<String>>(json_str) {
                return Ok(suggestions);
            }
        }
    }
    // JSON 파싱 실패 시 raw 출력 포함 에러 (claude가 마크다운 등으로 응답했을 가능성)
    Err(format!("JSON 파싱 실패 (raw='{}')", &raw[..raw.len().min(300)]))
}

/// AI 이름 일괄 추천 (여러 포트를 한 번의 claude -p 호출로 처리)
#[tauri::command]
fn suggest_names_batch(ports: Vec<serde_json::Value>) -> Result<serde_json::Value, String> {
    use std::fs;

    if ports.is_empty() {
        return Ok(serde_json::json!({}));
    }

    let mut project_lines: Vec<String> = Vec::new();
    let mut valid_ids: Vec<String> = Vec::new();

    for port in &ports {
        let id = port.get("id").and_then(|v| v.as_str()).unwrap_or("").to_string();
        let folder_path = port.get("folderPath").and_then(|v| v.as_str()).unwrap_or("").to_string();

        if id.is_empty() || folder_path.is_empty() {
            continue;
        }
        let path = std::path::Path::new(&folder_path);
        if !path.exists() {
            continue;
        }

        let files: Vec<String> = fs::read_dir(path)
            .map(|entries| {
                entries
                    .filter_map(|e| e.ok())
                    .map(|e| e.file_name().to_string_lossy().to_string())
                    .take(20)
                    .collect()
            })
            .unwrap_or_default();

        let pkg_json = fs::read_to_string(path.join("package.json"))
            .map(|s| s.chars().take(300).collect::<String>())
            .unwrap_or_default();

        project_lines.push(format!(
            "id={} files=[{}] package.json={}",
            id,
            files.join(", "),
            if pkg_json.is_empty() { "none".to_string() } else { pkg_json }
        ));
        valid_ids.push(id);
    }

    if valid_ids.is_empty() {
        return Ok(serde_json::json!({}));
    }

    let prompt = format!(
        "For each project below, suggest 1 concise English project name (2-4 words).\nReply ONLY with a JSON object mapping each id to a name: {{\"id1\": \"Name One\", \"id2\": \"Name Two\"}}\n\n{}",
        project_lines.join("\n")
    );

    let escaped_prompt = prompt.replace('\'', "'\"'\"'");
    let shell_cmd = format!("claude -p '{}'", escaped_prompt);

    let out = std::process::Command::new("/bin/zsh")
        .args(["-l", "-c", &shell_cmd])
        .output()
        .map_err(|e| format!("shell 실행 실패: {}", e))?;

    let raw = String::from_utf8_lossy(&out.stdout).trim().to_string();
    let err_raw = String::from_utf8_lossy(&out.stderr).trim().to_string();

    if !out.status.success() || raw.is_empty() {
        return Err(format!("claude 실패 (exit={}) stdout='{}' stderr='{}'",
            out.status.code().unwrap_or(-1),
            &raw[..raw.len().min(300)],
            &err_raw[..err_raw.len().min(300)]));
    }

    if let Some(start) = raw.find('{') {
        if let Some(end) = raw.rfind('}') {
            let json_str = &raw[start..=end];
            if let Ok(result) = serde_json::from_str::<serde_json::Value>(json_str) {
                return Ok(result);
            }
        }
    }

    Err(format!("JSON 파싱 실패 (raw='{}')", &raw[..raw.len().min(300)]))
}

// ──────────────────── cmux (Mac-only terminal multiplexer) ────────────────────
// cmux invocation lives in Rust because the Bun api-server's long-running
// Bun.serve handler context degrades cmux subprocess calls over time
// (Broken pipe on every cmux ping after a few minutes), while identical calls
// from any other context — shell, nohup bash, standalone bun — remain reliable.

fn resolve_cmux_cli() -> Option<String> {
    use std::path::Path;
    if Path::new("/Applications/cmux.app/Contents/Resources/bin/cmux").exists() {
        return Some("/Applications/cmux.app/Contents/Resources/bin/cmux".into());
    }
    if let Some(home) = std::env::var_os("HOME") {
        let home_app = format!("{}/Applications/cmux.app/Contents/Resources/bin/cmux", home.to_string_lossy());
        if Path::new(&home_app).exists() { return Some(home_app); }
    }
    if Path::new("/opt/homebrew/bin/cmux").exists() {
        return Some("/opt/homebrew/bin/cmux".into());
    }
    None
}

fn resolve_claude_cli() -> String {
    use std::path::Path;
    use std::process::Command;
    // login shell로 which claude 실행 — Finder 실행 시 PATH가 제한되므로 zsh -l 필요
    if let Ok(out) = Command::new("/bin/zsh").args(["-l", "-c", "which claude"]).output() {
        let p = String::from_utf8_lossy(&out.stdout).trim().split('\n').next().unwrap_or("").trim().to_string();
        if !p.is_empty() && Path::new(&p).exists() {
            return p;
        }
    }
    // fallback: bash login shell
    if let Ok(out) = Command::new("/bin/bash").args(["-l", "-c", "which claude"]).output() {
        let p = String::from_utf8_lossy(&out.stdout).trim().split('\n').next().unwrap_or("").trim().to_string();
        if !p.is_empty() && Path::new(&p).exists() {
            return p;
        }
    }
    // 알려진 고정 경로 (homebrew/local 우선, cmux 번들은 마지막)
    for p in &[
        "/opt/homebrew/bin/claude",
        "/usr/local/bin/claude",
        "/Applications/cmux.app/Contents/Resources/bin/claude",
    ] {
        if Path::new(p).exists() { return (*p).into(); }
    }
    "claude".into()
}

fn wait_cmux_ready(cli: &str, total: std::time::Duration) -> bool {
    let deadline = std::time::Instant::now() + total;
    while std::time::Instant::now() < deadline {
        if Command::new(cli).arg("ping").output().map(|o| o.status.success()).unwrap_or(false) {
            return true;
        }
        std::thread::sleep(std::time::Duration::from_millis(250));
    }
    false
}

/// cmux에 열린 창이 없으면 새 창을 생성한다.
/// TabManager는 열린 창이 있을 때만 활성화된다.
fn ensure_cmux_window(cli: &str) {
    let out = Command::new(cli).args(["list-windows"]).output();
    if let Ok(o) = out {
        let stdout = String::from_utf8_lossy(&o.stdout);
        if stdout.trim() == "No windows" {
            let _ = Command::new(cli).args(["new-window"]).output();
            std::thread::sleep(std::time::Duration::from_millis(500));
        }
    }
}

fn cmux_send_with_retry(cli: &str, payload: &str) -> Result<(), String> {
    let mut last_err = String::new();
    for attempt in 0..3 {
        let out = Command::new(cli).args(["send", payload]).output()
            .map_err(|e| format!("cmux send spawn 실패: {}", e))?;
        if out.status.success() { return Ok(()); }
        last_err = String::from_utf8_lossy(&out.stderr).trim().to_string();
        if !last_err.contains("Broken pipe") && !last_err.contains("errno 32") { break; }
        if attempt < 2 { std::thread::sleep(std::time::Duration::from_millis(300)); }
    }
    Err(format!("cmux send 실패: {}", if last_err.is_empty() { "unknown".into() } else { last_err }))
}

fn cmux_install_error() -> String {
    "cmux가 설치되지 않았습니다.\n설치: brew tap manaflow-ai/cmux && brew install --cask cmux".to_string()
}

fn first_worktree(worktree_path: &Option<String>) -> Option<String> {
    worktree_path.as_deref()
        .and_then(|s| s.split(',').next())
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
}

#[tauri::command]
fn get_platform() -> String {
    if cfg!(target_os = "windows") { "windows".to_string() }
    else if cfg!(target_os = "macos") { "macos".to_string() }
    else { "linux".to_string() }
}

#[tauri::command]
fn open_cmux_claude(name: String, folder_path: Option<String>, worktree_path: Option<String>, bypass: bool) -> Result<String, String> {
    if cfg!(windows) { return Err("cmux는 맥에서만 가능합니다".into()); }

    let cd_path = first_worktree(&worktree_path)
        .or(folder_path)
        .filter(|s| !s.is_empty())
        .ok_or_else(|| "프로젝트 경로가 없습니다.".to_string())?;

    let cli = resolve_cmux_cli().ok_or_else(cmux_install_error)?;
    let _ = Command::new("open").args(["-a", "cmux"]).status();

    if !wait_cmux_ready(&cli, std::time::Duration::from_secs(5)) {
        return Err(cmux_access_help_msg("cmux 소켓 준비 대기 시간 초과 (5초)"));
    }
    ensure_cmux_window(&cli);

    let claude_cli = if bypass { "claude --dangerously-skip-permissions" } else { "claude" };
    // Atomic: create a fresh workspace at the project path and run claude there.
    // Title format mirrors tmux (build_window_title): "⚡️ project › worktree" (bypass) or "🔷 project › worktree".
    let title = build_window_title(&name, worktree_path.as_deref(), true, bypass, false);
    let out = Command::new(&cli)
        .args(["new-workspace", "--cwd", &cd_path, "--command", claude_cli, "--name", &title])
        .output()
        .map_err(|e| format!("cmux new-workspace 실행 실패: {}", e))?;
    if !out.status.success() {
        let stderr = String::from_utf8_lossy(&out.stderr).trim().to_string();
        return Err(cmux_access_help_msg(&format!("cmux new-workspace 실패: {}", stderr)));
    }
    Ok(format!("cmux Claude{} 실행 중", if bypass { " bypass" } else { "" }))
}

#[tauri::command]
fn open_cmux_claude_new(name: String, folder_path: Option<String>, worktree_path: Option<String>, bypass: bool) -> Result<String, String> {
    if cfg!(windows) { return Err("cmux는 맥에서만 가능합니다".into()); }

    let cd_path = first_worktree(&worktree_path)
        .or(folder_path)
        .filter(|s| !s.is_empty())
        .ok_or_else(|| "프로젝트 경로가 없습니다.".to_string())?;

    let cli = resolve_cmux_cli().ok_or_else(cmux_install_error)?;
    let _ = Command::new("open").args(["-a", "cmux"]).status();

    if !wait_cmux_ready(&cli, std::time::Duration::from_secs(5)) {
        return Err(cmux_access_help_msg("cmux 소켓 준비 대기 시간 초과 (5초)"));
    }
    ensure_cmux_window(&cli);

    let claude_cli = if bypass { "claude --dangerously-skip-permissions" } else { "claude" };
    // is_fresh=true distinguishes the "↺ 새창" button from the regular one.
    let title = build_window_title(&name, worktree_path.as_deref(), true, bypass, true);
    let out = Command::new(&cli)
        .args(["new-workspace", "--cwd", &cd_path, "--command", claude_cli, "--name", &title])
        .output()
        .map_err(|e| format!("cmux new-workspace 실행 실패: {}", e))?;
    if !out.status.success() {
        let stderr = String::from_utf8_lossy(&out.stderr).trim().to_string();
        return Err(cmux_access_help_msg(&format!("cmux new-workspace 실패: {}", stderr)));
    }
    Ok(format!("cmux 새창{} 시작 ↺", if bypass { " bypass" } else { "" }))
}

#[tauri::command]
fn open_cmux_terminal(name: String, folder_path: Option<String>) -> Result<String, String> {
    if cfg!(windows) { return Err("cmux는 맥에서만 가능합니다".into()); }

    // Empty/missing path → fall back to $HOME (root area).
    let cd_path = folder_path
        .filter(|s| !s.trim().is_empty())
        .or_else(|| std::env::var("HOME").ok())
        .unwrap_or_else(|| "/".into());

    let cli = resolve_cmux_cli().ok_or_else(cmux_install_error)?;
    let _ = Command::new("open").args(["-a", "cmux"]).status();

    if !wait_cmux_ready(&cli, std::time::Duration::from_secs(5)) {
        return Err(cmux_access_help_msg("cmux 소켓 준비 대기 시간 초과 (5초)"));
    }
    ensure_cmux_window(&cli);

    let title = format!("🪟 {}", name);
    let out = Command::new(&cli)
        .args(["new-workspace", "--cwd", &cd_path, "--name", &title])
        .output()
        .map_err(|e| format!("cmux new-workspace 실행 실패: {}", e))?;
    if !out.status.success() {
        let stderr = String::from_utf8_lossy(&out.stderr).trim().to_string();
        return Err(cmux_access_help_msg(&format!("cmux new-workspace 실패: {}", stderr)));
    }
    Ok("cmux 터미널 열림".into())
}

#[tauri::command]
fn open_cmux_tmux(name: String, folder_path: Option<String>, worktree_path: Option<String>, bypass: bool, fresh: bool) -> Result<String, String> {
    if cfg!(windows) { return Err("cmux는 맥에서만 가능합니다".into()); }
    let cd_path = worktree_path
        .as_deref()
        .and_then(|s| s.split(',').next())
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .or_else(|| folder_path.clone())
        .filter(|s| !s.trim().is_empty())
        .ok_or("프로젝트 경로가 없습니다.")?;
    let claude_cli = if bypass { "claude --dangerously-skip-permissions" } else { "claude" };
    let session_name = name.chars().map(|c| if c.is_alphanumeric() || c == '-' || c == '_' { c } else { '_' }).collect::<String>();
    let session_name = if session_name.is_empty() { "port".to_string() } else { session_name };
    let tmux_cmd = if fresh {
        format!("tmux kill-session -t {session_name} 2>/dev/null; tmux new-session -s {session_name} -c '{cd_path}' {claude_cli}")
    } else {
        format!("tmux new-session -A -s {session_name} -c '{cd_path}' {claude_cli}")
    };
    let title = format!("{} (tmux){}", name, if bypass { " ⚡" } else { "" });
    let cli = resolve_cmux_cli().ok_or_else(cmux_install_error)?;
    let _ = Command::new("open").args(["-a", "cmux"]).status();
    if !wait_cmux_ready(&cli, std::time::Duration::from_secs(5)) {
        return Err(cmux_access_help_msg("cmux 소켓 준비 대기 시간 초과 (5초)"));
    }
    let out = Command::new(&cli)
        .args(["new-workspace", "--cwd", &cd_path, "--command", &tmux_cmd, "--name", &title])
        .output()
        .map_err(|e| format!("cmux tmux 실행 실패: {}", e))?;
    if !out.status.success() {
        let stderr = String::from_utf8_lossy(&out.stderr).trim().to_string();
        return Err(cmux_access_help_msg(&format!("cmux tmux 실패: {}", stderr)));
    }
    Ok(format!("cmux tmux{} 실행 중", if bypass { " bypass" } else { "" }))
}

#[tauri::command]
fn open_cmux_localhost(port: u16, name: String) -> Result<String, String> {
    if cfg!(windows) { return Err("cmux는 맥에서만 가능합니다".into()); }
    let url = format!("http://localhost:{}", port);
    let cli = resolve_cmux_cli().ok_or_else(cmux_install_error)?;
    let _ = Command::new("open").args(["-a", "cmux"]).status();
    if !wait_cmux_ready(&cli, std::time::Duration::from_secs(5)) {
        return Err(cmux_access_help_msg("cmux 소켓 준비 대기 시간 초과 (5초)"));
    }
    let out = Command::new(&cli)
        .args(["new-pane", "--type", "browser", "--url", &url, "--focus", "true"])
        .output()
        .map_err(|e| format!("cmux browser 실행 실패: {}", e))?;
    if !out.status.success() {
        let stderr = String::from_utf8_lossy(&out.stderr).trim().to_string();
        return Err(cmux_access_help_msg(&format!("cmux browser 실패: {}", stderr)));
    }
    Ok(format!("cmux 브라우저로 localhost:{} 열림", port))
}

#[tauri::command]
fn open_cmux_agent_view() -> Result<String, String> {
    if cfg!(windows) { return Err("cmux는 맥에서만 가능합니다".into()); }
    let cli = resolve_cmux_cli().ok_or_else(cmux_install_error)?;
    let _ = Command::new("open").args(["-a", "cmux"]).status();
    if !wait_cmux_ready(&cli, std::time::Duration::from_secs(5)) {
        return Err(cmux_access_help_msg("cmux 소켓 준비 대기 시간 초과 (5초)"));
    }
    ensure_cmux_window(&cli);
    let home = std::env::var("HOME").unwrap_or_else(|_| "/".into());
    let out = Command::new(&cli)
        .args(["new-workspace", "--cwd", &home, "--command", &format!("{} agents", resolve_claude_cli()), "--name", "🤖 Agent View"])
        .output()
        .map_err(|e| format!("cmux new-workspace 실행 실패: {}", e))?;
    if !out.status.success() {
        let stderr = String::from_utf8_lossy(&out.stderr).trim().to_string();
        return Err(cmux_access_help_msg(&format!("cmux new-workspace 실패: {}", stderr)));
    }
    Ok("cmux Session Resume 열림".into())
}

#[tauri::command]
fn open_cmux_project_agents(folder_path: Option<String>, name: String) -> Result<String, String> {
    if cfg!(windows) { return Err("cmux는 맥에서만 가능합니다".into()); }
    let home = std::env::var("HOME").unwrap_or_else(|_| "/".into());
    let cd_path = folder_path.filter(|s| !s.trim().is_empty()).unwrap_or_else(|| home.clone());
    let cli = resolve_cmux_cli().ok_or_else(cmux_install_error)?;
    let _ = Command::new("open").args(["-a", "cmux"]).status();
    if !wait_cmux_ready(&cli, std::time::Duration::from_secs(5)) {
        return Err(cmux_access_help_msg("cmux 소켓 준비 대기 시간 초과 (5초)"));
    }
    ensure_cmux_window(&cli);
    let base_name = if !name.trim().is_empty() {
        name.clone()
    } else {
        cd_path.split('/').filter(|s| !s.is_empty()).last().unwrap_or("project").to_string()
    };
    let title = format!("🤖 {} agents", base_name);
    let out = Command::new(&cli)
        .args(["new-workspace", "--cwd", &cd_path, "--command", &format!("{} agents", resolve_claude_cli()), "--name", &title])
        .output()
        .map_err(|e| format!("cmux new-workspace 실행 실패: {}", e))?;
    if !out.status.success() {
        let stderr = String::from_utf8_lossy(&out.stderr).trim().to_string();
        return Err(cmux_access_help_msg(&format!("cmux new-workspace 실패: {}", stderr)));
    }
    Ok(format!("cmux Agent View 열림 ({})", base_name))
}

#[tauri::command]
fn open_claude_bg(folder_path: Option<String>, name: String, bypass: Option<bool>) -> Result<String, String> {
    if cfg!(windows) { return Err("claude --bg는 맥에서만 가능합니다".into()); }
    let home = std::env::var("HOME").unwrap_or_else(|_| "/".into());
    let cd_path = folder_path.filter(|s| !s.trim().is_empty()).unwrap_or_else(|| home.clone());
    let label = if !name.trim().is_empty() {
        name.clone()
    } else {
        cd_path.split('/').filter(|s| !s.is_empty()).last().unwrap_or("project").to_string()
    };
    let prompt = format!("{} 작업 시작", label);
    let use_bypass = bypass.unwrap_or(false);

    // login shell로 실행 — ~/.zshrc 소싱 → 올바른 PATH + claude 인증 토큰 자동 로드
    // (Tauri 직접 spawn은 Finder 실행 시 최소 PATH만 상속받아 claude/node를 못 찾음)
    let claude_cli = resolve_claude_cli();
    let bypass_flag = if use_bypass { " --dangerously-skip-permissions" } else { "" };
    let escaped_prompt = prompt.replace('\'', "'\"'\"'"); // sh single-quote escape
    let shell_cmd = format!(
        "cd '{}' && '{}'{} --bg '{}'",
        escape_sq(&cd_path),
        claude_cli,
        bypass_flag,
        escaped_prompt
    );

    let out = Command::new("/bin/zsh")
        .args(["-l", "-c", &shell_cmd])
        .output()
        .map_err(|e| format!("claude --bg 실행 실패: {}", e))?;

    if !out.status.success() {
        // 프론트엔드에서 "claude --bg 실패:" 프리픽스를 붙이므로, 여기서는 raw 에러만 반환
        let stderr = String::from_utf8_lossy(&out.stderr).trim().to_string();
        let stdout = String::from_utf8_lossy(&out.stdout).trim().to_string();
        let combined = if !stderr.is_empty() && !stdout.is_empty() {
            format!("{}\n{}", stderr, stdout)
        } else if !stderr.is_empty() {
            stderr
        } else if !stdout.is_empty() {
            stdout
        } else {
            "알 수 없는 오류".into()
        };
        return Err(combined);
    }
    let stdout = String::from_utf8_lossy(&out.stdout).trim().to_string();
    Ok(format!("agent view에 등록됨: {}\n{}", label, stdout))
}

/// If the error pattern suggests access denied (cmuxOnly mode), append guidance.
fn cmux_access_help_msg(base: &str) -> String {
    format!(
        "{}\n\n💡 cmux 설정 확인: cmux 메뉴 → Settings → Socket Control → \"Allow All\"로 변경 후 재시도하세요. (현재 cmuxOnly 모드는 외부 앱의 호출을 차단)",
        base
    )
}

#[tauri::command]
fn get_global_shortcut(app: tauri::AppHandle) -> String {
    let path = app.path().app_data_dir()
        .map(|d| d.join("shortcut.json"));
    path.ok()
        .and_then(|p| std::fs::read_to_string(p).ok())
        .and_then(|s| serde_json::from_str::<serde_json::Value>(&s).ok())
        .and_then(|v| v["shortcut"].as_str().map(|s| s.to_string()))
        .unwrap_or_else(|| "CommandOrControl+Alt+P".to_string())
}

#[tauri::command]
fn set_global_shortcut(app: tauri::AppHandle, shortcut: String, old_shortcut: String) -> Result<(), String> {
    if !old_shortcut.is_empty() {
        let _ = app.global_shortcut().unregister(old_shortcut.as_str());
    }
    app.global_shortcut().register(shortcut.as_str())
        .map_err(|e| e.to_string())?;
    let path = app.path().app_data_dir()
        .map_err(|e| e.to_string())?
        .join("shortcut.json");
    let json = serde_json::json!({ "shortcut": shortcut });
    std::fs::write(&path, json.to_string())
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
fn detect_start_command(folder_path: String) -> Option<String> {
    let path = std::path::Path::new(&folder_path);

    // package.json → bun run dev / bun run start
    let pkg_path = path.join("package.json");
    if pkg_path.exists() {
        if let Ok(content) = fs::read_to_string(&pkg_path) {
            if let Ok(pkg) = serde_json::from_str::<serde_json::Value>(&content) {
                if let Some(scripts) = pkg.get("scripts") {
                    if scripts.get("dev").is_some() {
                        return Some("bun run dev".to_string());
                    }
                    if scripts.get("start").is_some() {
                        return Some("bun run start".to_string());
                    }
                }
            }
        }
        return Some("bun run dev".to_string());
    }

    // pyproject.toml → uv run
    if path.join("pyproject.toml").exists() {
        return Some("uv run python main.py".to_string());
    }

    // Cargo.toml → cargo run
    if path.join("Cargo.toml").exists() {
        return Some("cargo run".to_string());
    }

    None
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  let window_visible = Arc::new(Mutex::new(true));
  let vis_shortcut = Arc::clone(&window_visible);
  let vis_close = Arc::clone(&window_visible);
  tauri::Builder::default()
    .manage(AppState {
        processes: Mutex::new(HashMap::new()),
    })
    .invoke_handler(tauri::generate_handler![
        load_ports,
        save_ports,
        scan_command_files,
        open_app_data_dir,
        load_portal,
        save_portal,
        load_workspace_roots,
        save_workspace_roots,
        execute_command,
        detect_start_command,
        stop_command,
        force_restart_command,
        detect_port,
        check_port_status,
        build_app,
        install_app_to_applications,
        open_build_folder,
        open_folder,
        import_ports_from_file,
        open_in_chrome,
        open_log,
        read_log_content,
        check_wsl,
        install_wsl,
        install_wsl_tmux,
        open_tmux_claude,
        open_tmux_claude_fresh,
        open_tmux_claude_bypass,
        open_terminal_claude,
        open_terminal_claude_bypass,
        run_claude_with_prompt,
        export_dmg,
        git_worktree_add,
        git_worktree_remove,
        git_merge_branch,
        list_git_worktrees,
        check_file_exists,
        create_folder,
        suggest_name,
        suggest_names_batch,
        open_cmux_claude,
        open_cmux_claude_new,
        open_cmux_terminal,
        open_cmux_tmux,
        open_cmux_localhost,
        open_cmux_agent_view,
        open_cmux_project_agents,
        open_claude_bg,
        get_global_shortcut,
        set_global_shortcut,
        get_platform,
    ])
    .plugin(tauri_plugin_dialog::init())
    .plugin(tauri_plugin_fs::init())
    .plugin(tauri_plugin_updater::Builder::new().build())
    .plugin(tauri_plugin_process::init())
    .plugin(
      tauri_plugin_global_shortcut::Builder::new()
        .with_handler(move |app_handle, _shortcut, event| {
          if event.state() == ShortcutState::Pressed {
            if let Some(window) = app_handle.get_webview_window("main") {
              let mut vis = vis_shortcut.lock().unwrap();
              if *vis {
                let _ = window.hide();
                *vis = false;
              } else {
                let _ = window.show();
                let _ = window.unminimize();
                let _ = window.set_focus();
                *vis = true;
              }
            }
          }
        })
        .build()
    )
    .setup(|app| {
      if cfg!(debug_assertions) {
        app.handle().plugin(
          tauri_plugin_log::Builder::default()
            .level(log::LevelFilter::Info)
            .build(),
        )?;
      }
      // 창 닫기 → 숨김 (백그라운드 유지 — 단축키가 항상 동작하도록)
      if let Some(window) = app.get_webview_window("main") {
        let win = window.clone();
        window.on_window_event(move |event| {
          if let tauri::WindowEvent::CloseRequested { api, .. } = event {
            api.prevent_close();
            let _ = win.hide();
            *vis_close.lock().unwrap() = false;
          }
        });
      }
      // 저장된 글로벌 단축키 불러와서 등록
      let shortcut_path = app.path().app_data_dir()
        .map(|d| d.join("shortcut.json"))
        .ok();
      let saved = shortcut_path.as_ref()
        .and_then(|p| std::fs::read_to_string(p).ok())
        .and_then(|s| serde_json::from_str::<serde_json::Value>(&s).ok())
        .and_then(|v| v["shortcut"].as_str().map(|s| s.to_string()))
        .unwrap_or_else(|| "CommandOrControl+Alt+P".to_string());
      let _ = app.global_shortcut().register(saved.as_str());
      Ok(())
    })
    .build(tauri::generate_context!())
    .expect("error while building tauri application")
    .run(|app_handle, event| {
      // macOS 전용: Dock 아이콘 클릭 시 숨겨진 창 복원 (Reopen variant 는 macOS 만 존재)
      #[cfg(target_os = "macos")]
      {
        if let tauri::RunEvent::Reopen { has_visible_windows, .. } = event {
          if !has_visible_windows {
            if let Some(window) = app_handle.get_webview_window("main") {
              let _ = window.show();
              let _ = window.unminimize();
              let _ = window.set_focus();
            }
          }
        }
      }
      #[cfg(not(target_os = "macos"))]
      {
        let _ = (app_handle, event); // Windows/Linux 미사용 인자 경고 억제
      }
    });
}
