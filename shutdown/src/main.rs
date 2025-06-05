use std::collections::HashMap;
use std::{fs, io};
use chrono::{ Local};
use reqwest::blocking::Client;
use serde::Serialize;



#[derive(Serialize)]
#[derive(Debug)]
struct Body{
    time:String,
    isOmittedWorkingDay:bool,
    code:String,
    divisionCode:String,
}
impl Body {
    // Example constructor for Body
    pub fn new(time: String, code: String, divisionCode:String) -> Self {
        Body {
            time,
            isOmittedWorkingDay: true,
            code,
            divisionCode,
        }
    }
}
#[derive(Debug)]
enum MyError {
    Io(io::Error),//IOのエラー
}

impl From<io::Error> for MyError {
    fn from(e: io::Error) -> Self {
        MyError::Io(e)
    }
}

impl MyError {
    pub fn print_message(&self) {
        match self {
            MyError::Io(e) => {
                eprintln!("IOエラーが発生しました: {}", e);
            }
        }
    }
}

fn read_env() -> Result<HashMap<String,String>, MyError> {
    let line = fs::read_to_string("./env")?;//ファイル読み込み
    let map: HashMap<String, String> = line.lines()//ラインを改行で分割
        .filter_map(|line| line.split_once('='))//ラインごとに=で区切ってmap関数に(k,v)を渡す
        .map(|(k, v)| (k.trim().to_string(), v.trim().to_string()))
        .collect();//Hashmapに格納していく
    Ok(map)
}
fn main(){
    let token:String;
    let employeeKey:String;
    let divisionCode:String;
    match read_env() {
        Ok(content) => {
            token=content["TOKEN"].to_string();
            employeeKey=content["ID"].to_string();
            divisionCode=content["DIVISION_CODE"].to_string();
        }
        Err(e) => {
            e.print_message();
            panic!();
        }
    }
    let now = Local::now();
    let formatted = now.format("%Y-%m-%dT%H:%M:%S").to_string();
    let body=Body::new( formatted,"2".to_string(),divisionCode);
        // APIのURL
    let url = format!("https://api.kingtime.jp/daily-workings/timerecord/{}",employeeKey);
    print!("{:?}",body);
    // クライアント作成
    let client = Client::new();

    let response = client
        .post(url)
        .header("Authorization", format!("Bearer {}", token)) 
        .json(&body)
        .timeout(std::time::Duration::from_secs(5))
        .send();
    match response{
        Ok(res) => {
            let status = res.status();
            println!("ステータスコード: {}", status);
        }
        Err(e) => {
            eprintln!("エラーが発生しました: {}", e);
        }
    }
}
