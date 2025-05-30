use std::collections::HashMap;
use std::{fs, io};
use std::num::ParseIntError;
use chrono::{DateTime, FixedOffset, Local};
#[derive(Debug)]
struct Config {
    id: String, 
}
#[derive(Debug)]
struct Body{
    time:String,
    isOmittedWorkingDay:bool,
    code:i8,
    divisionCode:i32,
}
impl Body {
    // Example constructor for Body
    pub fn new(time: String, code: i8, division_code: i32) -> Self {
        Body {
            time,
            isOmittedWorkingDay: true,
            code,
            divisionCode: division_code,
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
    let line = fs::read_to_string("./.env")?;//ファイル読み込み
    let map: HashMap<String, String> = line.lines()//ラインを改行で分割
        .filter_map(|line| line.split_once('='))//ラインごとに=で区切ってmap関数に(k,v)を渡す
        .map(|(k, v)| (k.trim().to_string(), v.trim().to_string()))
        .collect();//Hashmapに格納していく
    Ok(map)
}
fn main(){
    let local_dt=Local::now();
    let offset = FixedOffset::east_opt(9 * 3600).unwrap();
    let fixed_dt: DateTime<FixedOffset> = local_dt.with_timezone(&offset);
    let rfc3339_str = fixed_dt.to_rfc3339();
    print!("{:?}",rfc3339_str);
    let body=Body::new( rfc3339_str,1,1);
    let mut token:String;
    let mut id:String;
    match read_env() {
        Ok(content) => {
            token=content["TOKEN"].to_string();
            id=content["ID"].to_string();
        }
        Err(e) => {
            e.print_message();
            panic!();
        }
    }
        // APIのURL
    let url = format("https://example.com/api/shutdown");

    // クライアント作成
    let client = Client::new();

    // リクエスト送信
    let res = client
        .post(url)
        .bearer_auth(token)
        .json(&body)?;
        
    
}
