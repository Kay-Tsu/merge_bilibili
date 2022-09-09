// extern crate core;

use std::{env, process};
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::path::Path;
use std::process::{Command, Output};

use serde::{Deserialize, Serialize};

pub struct Config {
    pub in_path: String,
    pub out_path: String,
}

#[derive(Debug)]
struct MyError(String);

impl Display for MyError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Error for MyError {

}

#[derive(Deserialize, Serialize)]
struct JsonContent {
    page_data: SubContent
}

#[derive(Deserialize, Serialize)]
struct SubContent {
    part: String,
    page: i32
}

struct AudioVideoConfig {
    out_file: String,
    audio: String,
    video: String
}

impl AudioVideoConfig {
    fn new() -> AudioVideoConfig {
        AudioVideoConfig {
            out_file: String::from(""),
            audio: String::from(""),
            video: String::from(""),
        }
    }
}

impl Display for AudioVideoConfig {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}, {}, {}", self.audio, self.video, self.out_file)
    }
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next();

        let in_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a in_path string"),
        };
        let out_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a out_path string"),
        };

        Ok(Config{
            in_path,
            out_path
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // 处理目录
    for entry in std::fs::read_dir(&config.in_path)? {
        let d = entry?;
        if d.path().is_dir() {
            let mut audio_video = AudioVideoConfig::new();
            visit_dir(d.path().as_path(), &mut audio_video);
            // println!("{}", audio_video);
            // 合并文件
            // ffmpeg -i video.m4s -i audio.m4s -c:v copy -c:a aac -strict experimental output.mp4
            let command = String::from("ffmpeg -i ") + audio_video.video.as_str()
             + " -i " + audio_video.audio.as_str() + " -c:v copy -c:a aac -strict experimental "
            + config.out_path.as_str() + "/" + audio_video.out_file.as_str() + ".mp4";
            exec(command.as_ref());
        }
    }
    Ok(())
}

fn exec(command: &str) -> Output {
    println!("{}", command);
    let output_result = Command::new("sh")
        .arg("-c")
        .arg(command)
        .output();
    let output = match output_result {
        Ok(value) => value,
        Err(_) => {
            eprintln!("Failed to execute command: {}", command);
            process::exit(1);
        }
    };

    output
}

fn visit_dir(path: &Path, audio_video: &mut AudioVideoConfig) -> std::io::Result<()> {
    for entry in std::fs::read_dir(path)? {
        let d = entry?;
        // 读取json文件, 获取文件名称
        if d.path().is_file() && d.path().file_name().unwrap().to_str().unwrap() == "entry.json" {
            // println!("{}", d.file_name().to_str().unwrap());
            let filename = get_file_name(d.path().as_path());
            // 替换异常字符, 保证ffmpeg合并正常
            let f1 = filename.replace(" ", "");
            let f2 = f1.replace("-", "_");
            let f3 = f2.replace("&", "_");
            audio_video.out_file = f3;
        }

        if d.path().is_file() && d.path().file_name().unwrap().to_str().unwrap() == "audio.m4s" {
            // println!("{}", d.file_name().to_str().unwrap());
            audio_video.audio = d.path().as_path().to_str().unwrap().to_string();
        }

        if d.path().is_file() && d.path().file_name().unwrap().to_str().unwrap() == "video.m4s" {
            // println!("{}", d.file_name().to_str().unwrap());
            audio_video.video = d.path().as_path().to_str().unwrap().to_string();
        }

        if d.path().is_dir() {
            visit_dir(d.path().as_path(), audio_video);
        }
    }

    Ok(())
}

fn get_file_name(path: &Path) -> String {
    let content = std::fs::read_to_string(path).unwrap();
    let json: JsonContent = serde_json::from_str(&content).unwrap();
    // println!("{}.{}", json.page_data.page, &json.page_data.part);
    String::from(json.page_data.page.to_string() + "." + json.page_data.part.as_str())
}