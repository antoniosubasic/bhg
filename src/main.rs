use chrono::prelude::*;
use serde::Serialize;
use serde_json::{ser::PrettyFormatter, Serializer, Value};
use std::{collections::HashMap, env, fs, path::PathBuf};

struct Variables {
    pub directory: PathBuf,
    pub path: PathBuf,
    variables: HashMap<String, Value>,
}

impl Variables {
    fn new() -> Self {
        let directory = dirs::home_dir()
            .expect("failed to get home directory")
            .join(".config/bhg");
        let path = directory.join("variables.json");

        let mut variables: HashMap<String, Value> =
            serde_json::from_str(&fs::read_to_string(&path).expect("failed to read variables"))
                .expect("failed to parse variables");

        let now = Local::now();
        variables.insert("year".to_string(), Value::String(now.format("%Y").to_string()));
        variables.insert("month".to_string(), Value::String(now.format("%m").to_string()));
        variables.insert("day".to_string(), Value::String(now.format("%d").to_string()));
        variables.insert("hour".to_string(), Value::String(now.format("%H").to_string()));
        variables.insert("minute".to_string(), Value::String(now.format("%M").to_string()));
        variables.insert("second".to_string(), Value::String(now.format("%S").to_string()));
        variables.insert("microsecond".to_string(), Value::String(now.format("%f").to_string()));
        variables.insert("weekday".to_string(), Value::String(now.format("%u").to_string()));
        variables.insert("weekdayName".to_string(), Value::String(now.format("%A").to_string()));
        variables.insert("monthName".to_string(), Value::String(now.format("%B").to_string()));
        variables.insert("timestamp".to_string(), Value::String(now.timestamp().to_string()));
        variables.insert("timestampMs".to_string(), Value::String(now.timestamp_millis().to_string()));
        variables.insert("timestampISO".to_string(), Value::String(now.to_rfc3339()));

        Self {
            directory,
            path,
            variables,
        }
    }
}

fn main() {
    let variables = Variables::new();
    let args: Vec<String> = env::args().collect();

    match args.get(1).expect("no arguments provided").as_str() {
        "--init" | "-i" => {
            if variables.path.exists() {
                panic!("variables.json already exists");
            } else {
                let formatter = PrettyFormatter::with_indent(b"    ");
                let mut buffer = Vec::new();
                let mut serializer = Serializer::with_formatter(&mut buffer, formatter);

                [("name", Value::String("John Doe".to_owned()))]
                    .serialize(&mut serializer)
                    .expect("failed to serialize variables");

                fs::create_dir_all(variables.directory).expect("failed to create config directory");
                fs::write(variables.path, buffer).expect("failed to create variables.json");
            }
        }
        _ => {
            let output_file = env::current_dir()
                .expect("failed to get current directory")
                .join(args.get(1).expect("no output file provided"))
                .to_owned();

            let file_extension = output_file
                .extension()
                .expect("failed to get output file extension")
                .to_str()
                .expect("failed converting OsStr to string");

            let mut content =
                fs::read_to_string(variables.directory.join(format!("base.{}", file_extension)))
                    .expect("failed to read base file");

            content = content.replace("{description}", &args[2..].join(" "));
            for (key, value) in variables.variables.iter() {
                content = content.replace(&format!("{{{}}}", key), value.as_str().unwrap());
            }

            let longest_line = content
                .lines()
                .map(|line| line.chars().count() - if line.starts_with("->") { 2 } else { 0 })
                .max()
                .expect("failed to get longest line");

            let lines = content
                .lines()
                .map(|line| {
                    if !line.starts_with("->") {
                        line.to_string()
                    } else {
                        let spaces_before =
                            ((longest_line - (line.trim().chars().count() - 2)) as f64 / 2_f64)
                                .ceil();

                        format!(
                            "{}{}",
                            " ".repeat(spaces_before as usize),
                            &line[2..].trim()
                        )
                    }
                })
                .collect::<Vec<String>>();

            fs::write(output_file, lines.join("\n")).expect("failed to write output file");
        }
    }
}
