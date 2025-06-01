use std::env;
use std::fs;
use std::io;
use std::path::PathBuf;
use winreg::enums::*;
use winreg::RegKey;
use std::fs::{File};
use std::io::{ Write, BufRead};
struct Environment{
    EmployeeID:String,
    divisionCode:String,
    token:String,
}
fn install() -> io::Result<()> {
    // コピー元のexe（別アプリ）
    let source_exe = PathBuf::from(".\\target\\target_app.exe");

    // インストール先: %LOCALAPPDATA%\MyApp\target_app.exe
    let local_appdata = env::var("LOCALAPPDATA").unwrap();
    let install_dir = PathBuf::from(&local_appdata).join("MyApp");
    let install_path = install_dir.join("target_app.exe");

    // すでにコピー済みか確認（存在＆サイズ比較でもOK）
    if !install_path.exists() {
 
        fs::create_dir_all(&install_dir)?;
        fs::copy(&source_exe, &install_path)?;
        println!("インストール処理を開始します...");
                // レジストリ登録
        let hkcu = RegKey::predef(HKEY_CURRENT_USER);
        let run_key = hkcu.open_subkey_with_flags(
            "Software\\Microsoft\\Windows\\CurrentVersion\\Run",
            KEY_WRITE,
        )?;

        run_key.set_value("MyTargetApp", &install_path.to_str().unwrap())?;


        println!("✅ コピー＆登録完了: {}", install_path.display());
    } else {
        println!("すでにインストール済みです: {}", install_path.display());
    }

    Ok(())
}

fn write() -> io::Result<()> {
    // 入力を受け取る
    let stdin = io::stdin();
    let mut input = String::new();
    let local_appdata = env::var("LOCALAPPDATA").unwrap();
    let install_dir = PathBuf::from(&local_appdata).join("MyApp");
    // .envファイルのパス
    let env_path = install_dir.join(".env");
    // .envファイルに書き込む
    let mut file = File::create(&env_path)?;
    println!("EmployeeIDを入力してください");
    stdin.lock().read_line(&mut input)?;
    let name = input.trim(); // 改行削除
    writeln!(file, "ID={}", name)?;
    println!("divisionCodeを入力してください");
    stdin.lock().read_line(&mut input)?;
    let name = input.trim(); // 改行削除
    writeln!(file, "DIVISION_CODE={}", name)?;
    println!("TOKENを入力してください");
    stdin.lock().read_line(&mut input)?;
    let name = input.trim(); // 改行削除
    writeln!(file, "TOKEN={}", name)?;
    println!(".env を書き出しました: {}", env_path.display());
    Ok(())
}
fn main() {
    if let Err(e) = install() {
        panic!("インストールでエラーが発生しました: {}", e);
    }
    if let Err(e) = write() {
        panic!("書き込みでエラーが発生しました: {}", e);
    }
}