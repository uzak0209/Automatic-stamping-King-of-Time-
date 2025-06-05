use std::env;
use std::fs;
use std::fs::File;
use std::io;
use std::io::{BufRead, Write};
use std::path::PathBuf;
use std::process::Command;
use winreg::enums::*;
use winreg::RegKey;

struct Environment {
    employee_id: String,
    division_code: String,
    token: String,
}

/// ユーザーから入力を受け取る共通関数
fn get_user_input(prompt: &str) -> io::Result<String> {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin().lock().read_line(&mut input)?;
    Ok(input.trim().to_string())
}


/// インストールディレクトリのパスを取得
fn get_install_dir() -> PathBuf {
    let local_appdata = env::var("LOCALAPPDATA").unwrap();
    PathBuf::from(&local_appdata).join("MyApp")
}

/// レジストリにスタートアップ登録
fn register_startup(install_path: &PathBuf) -> io::Result<()> {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let run_key = hkcu.open_subkey_with_flags(
        "Software\\Microsoft\\Windows\\CurrentVersion\\Run",
        KEY_WRITE,
    )?;
    run_key.set_value("MyTargetApp", &install_path.to_str().unwrap())?;
    Ok(())
}

/// セットアップのインストール処理
fn install_setup() -> io::Result<()> {
    let source_exe = PathBuf::from(".\\target\\start.exe");
    let install_dir = get_install_dir();
    let install_path = install_dir.join("start.exe");
    
    // すでにコピー済みか確認
    if !install_path.exists() {
        fs::create_dir_all(&install_dir)?;
        fs::copy(&source_exe, &install_path)?;
        
        println!("インストール処理を開始します...");
        
        // レジストリ登録
        register_startup(&install_path)?;
        
        println!("✅ コピー＆登録完了: {}", install_path.display());
    } else {
        println!("すでにインストール済みです: {}", install_path.display());
    }
    
    Ok(())
}

/// シャットダウン時タスクを登録
fn register_shutdown_task(exe_path: &str) -> io::Result<bool> {
    let task_name = "MyShutdownTask";
    let ps = format!(
        "Register-ScheduledTask -TaskName '{}' -Trigger (New-ScheduledTaskTrigger -OnEvent -LogName 'System' -Source 'User32' -EventId 1074) -Action (New-ScheduledTaskAction -Execute '{}') -RunLevel Highest -Force",
        task_name, exe_path
    );
    
    let status = Command::new("powershell")
        .args(["-Command", &ps])
        .status()?;
        
    Ok(status.success())
}

/// シャットダウン時タスクの設定
fn setup_shutdown_task() -> io::Result<()> {
    let source_exe = PathBuf::from(".\\target\\shutdown.exe");
    let install_dir = get_install_dir();
    let install_path = install_dir.join("shutdown.exe");
    
    let answer = get_user_input("シャットダウン時に退勤を打刻しますか？y/n")?;
    
    if answer == "y" {
        let exe_path = install_path.to_str().unwrap();
        fs::create_dir_all(&install_dir)?;
        fs::copy(&source_exe, &install_path)?;
        if register_shutdown_task(exe_path)? {
            println!("✅ シャットダウン時実行タスクを登録しました");
        } else {
            println!("⚠️ タスク登録に失敗しました（管理者権限が必要な場合があります）");
        }
    }
    
    Ok(())
}


/// 環境変数を.envファイルに書き込み
fn write_env_variable(file: &mut File, key: &str, prompt: &str) -> io::Result<()> {
    let value = get_user_input(prompt)?;
    writeln!(file, "{}={}", key, value)?;
    Ok(())
}

/// .envファイルの作成と書き込み
fn create_env_file() -> io::Result<()> {
    let install_dir = get_install_dir();
    let env_path = install_dir.join(".env");
    let mut file = File::create(&env_path)?;
    
    write_env_variable(&mut file, "ID", "EmployeeIDを入力してください")?;
    write_env_variable(&mut file, "DIVISION_CODE", "divisionCodeを入力してください")?;
    write_env_variable(&mut file, "TOKEN", "TOKENを入力してください")?;
    
    println!(".env を書き出しました: {}", env_path.display());
    
    Ok(())
}

fn main() {
    if let Err(e) = install_setup() {
        panic!("インストールでエラーが発生しました: {}", e);
    }
    
    if let Err(e) = setup_shutdown_task() {
        panic!("シャットダウンタスク設定でエラーが発生しました: {}", e);
    }
    
    if let Err(e) = create_env_file() {
        panic!("環境設定ファイル作成でエラーが発生しました: {}", e);
    }
}