use chrono::prelude::*;
use serde::Serialize;
use serde_json::ser::PrettyFormatter;
use serde_json::Value;
use std::{collections::HashMap, env, fs, path::PathBuf};

struct Source {
    pub path: PathBuf,
}

impl Source {
    fn new() -> Self {
        Self {
            path: dirs::home_dir()
                .expect("failed to get home directory")
                .join(".bhg"),
        }
    }

    fn config(&self) -> PathBuf {
        self.path.join("config.json")
    }

    fn get_config(&self) -> HashMap<String, Value> {
        serde_json::from_str(
            &fs::read_to_string(self.config()).expect("failed to read config file"),
        )
        .expect("failed to parse config file")
    }

    fn get_variables(&self, description: &str) -> HashMap<String, Value> {
        let mut config = self.get_config().to_owned();

        let now = Local::now();

        config.insert(
            "description".to_string(),
            Value::String(description.to_string()),
        );
        config.insert(
            "year".to_string(),
            Value::String(now.format("%Y").to_string()),
        );
        config.insert(
            "month".to_string(),
            Value::String(now.format("%m").to_string()),
        );
        config.insert(
            "day".to_string(),
            Value::String(now.format("%d").to_string()),
        );
        config.insert(
            "hour".to_string(),
            Value::String(now.format("%H").to_string()),
        );
        config.insert(
            "minute".to_string(),
            Value::String(now.format("%M").to_string()),
        );
        config.insert(
            "second".to_string(),
            Value::String(now.format("%S").to_string()),
        );
        config.insert(
            "microsecond".to_string(),
            Value::String(now.format("%f").to_string()),
        );
        config.insert(
            "weekday".to_string(),
            Value::String(now.format("%u").to_string()),
        );
        config.insert(
            "weekdayName".to_string(),
            Value::String(now.format("%A").to_string()),
        );
        config.insert(
            "monthName".to_string(),
            Value::String(now.format("%B").to_string()),
        );
        config.insert(
            "timestamp".to_string(),
            Value::String(now.timestamp().to_string()),
        );
        config.insert(
            "timestampMs".to_string(),
            Value::String(now.timestamp_millis().to_string()),
        );
        config.insert("timestampISO".to_string(), Value::String(now.to_rfc3339()));

        config
    }

    fn get_default_config() -> HashMap<&'static str, Value> {
        vec![("tabSize", Value::Number(serde_json::Number::from(4)))]
            .into_iter()
            .collect()
    }

    fn as_pathbuf(&self) -> PathBuf {
        self.path.clone()
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let source = Source::new();

    match args.get(1).expect("no arguments provided").as_str() {
        "--init" | "-i" => {
            if source.config().exists() {
                println!("config file already exists");
                std::process::exit(1);
            } else {
                let formatter = PrettyFormatter::with_indent(b"    ");
                let mut buffer = Vec::new();
                let mut serializer = serde_json::Serializer::with_formatter(&mut buffer, formatter);
                Source::get_default_config()
                    .serialize(&mut serializer)
                    .expect("failed to serialize config");

                fs::create_dir_all(source.as_pathbuf()).expect("failed to create source directory");
                fs::write(source.config(), buffer).expect("failed to write config file");
            }
        }
        _ => {
            let variables = source.get_variables(&args[2..].join(" "));

            let output_file = env::current_dir()
                .expect("failed to get current directory")
                .join(args.get(1).expect("No output file provided"))
                .to_owned();

            let file_extension = output_file
                .to_str()
                .unwrap()
                .split('.')
                .last()
                .expect("failed to get output file extension");

            let mut content =
                fs::read_to_string(source.as_pathbuf().join(format!("base.{}", file_extension)))
                    .expect("failed to read base file");

            for (key, value) in variables.iter() {
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
