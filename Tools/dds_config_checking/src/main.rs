use anyhow::Result;
use clap::Parser;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use walkdir::{DirEntry, WalkDir};

/// Checking dds config is valid or not.
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Directory to checking dds config.json
    #[clap(short, long)]
    dds_directory: String,
}

#[derive(Serialize, Deserialize)]
struct DDSConfigFile {
    log_level: i32,
    topics: Vec<DDSConfigTopic>,
}

#[derive(Serialize, Deserialize)]
struct DDSConfigTopic {
    topic: String,
    dds_mode: String,
    buff_num: i32,
    elem_max_size: i32,
}

struct Topic {
    filename: String,
    buff_num: i32,
    elem_max_size: i32,
}

fn is_json(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.ends_with(".json"))
        .unwrap_or(false)
}

fn main() -> Result<()> {
    let args = Args::parse();

    println!("Hello {}!", args.dds_directory);

    let mut all_topics = HashMap::new();

    for entry in WalkDir::new(args.dds_directory) {
        let entry = entry?;
        if !is_json(&entry) {
            continue;
        }
        println!("{}", entry.path().display());

        let content = match fs::read_to_string(entry.path()) {
            Ok(content) => content,
            Err(_) => continue,
        };

        let dds_config_file: DDSConfigFile = match serde_json::from_str(&content) {
            Ok(dds_config_file) => dds_config_file,
            Err(_) => {
                continue;
            }
        };

        for in_topic in dds_config_file.topics.iter() {
            if all_topics.contains_key(&in_topic.topic) {
                let pre_topic: &Topic = all_topics.get(&in_topic.topic).unwrap();
                if in_topic.buff_num != pre_topic.buff_num
                    || in_topic.elem_max_size != pre_topic.elem_max_size
                {
                    println!("topic: {}", in_topic.topic);

                    println!("pre_topic.filename: {}", pre_topic.filename);
                    println!("pre_topic.buff_num: {}", pre_topic.buff_num);
                    println!("pre_topic.elem_max_size: {}", pre_topic.elem_max_size);

                    println!("in_topic.filename: {}", entry.path().display());
                    println!("in_topic.buff_num: {}", in_topic.buff_num);
                    println!("in_topic.elem_max_size: {}", in_topic.elem_max_size);

                    std::process::exit(0);
                }
            } else {
                let out_topic = Topic {
                    filename: String::from(entry.path().to_str().unwrap()),
                    buff_num: in_topic.buff_num,
                    elem_max_size: in_topic.elem_max_size,
                };

                all_topics.insert(in_topic.topic.clone(), out_topic);
            }
        }
    }

    Ok(())
}
