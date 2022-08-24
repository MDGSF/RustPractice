use anyhow::Result;
use clap::Parser;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use walkdir::{DirEntry, WalkDir};

/// Apply config.json template to all config.json in topic_dir.
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Config.json template
    #[clap(short, long)]
    config_tpl: String,

    /// Directory include multi config.json
    #[clap(short, long)]
    topic_dir: String,
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

fn is_json(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.ends_with(".json"))
        .unwrap_or(false)
}

fn read_config(filename: &str) -> Result<DDSConfigFile> {
    let content = fs::read_to_string(filename)?;
    let dds_config_file = serde_json::from_str(&content)?;
    Ok(dds_config_file)
}

fn write_config(filename: &str, config: &DDSConfigFile) -> Result<()> {
    let serialized = serde_json::to_string_pretty(&config)?;
    let mut file = File::create(filename)?;
    file.write_all(serialized.as_bytes())?;
    Ok(())
}

fn main() -> Result<()> {
    let args = Args::parse();

    println!("config_tpl: {}!", args.config_tpl);
    println!("topic_dir: {}!", args.topic_dir);

    let tpl = read_config(&args.config_tpl)?;
    let mut tpl_topics = HashMap::new();
    for topic in tpl.topics.iter() {
        tpl_topics.insert(topic.topic.clone(), topic);
    }

    for entry in WalkDir::new(args.topic_dir) {
        let entry = entry?;
        if !is_json(&entry) {
            continue;
        }
        println!("{}", entry.path().display());

        let filename = entry.path().to_str().unwrap();

        let mut config = match read_config(filename) {
            Ok(config) => config,
            Err(_) => continue,
        };

        let mut need_update = false;
        for topic in config.topics.iter_mut() {
            if tpl_topics.contains_key(&topic.topic) {
                let tpl_topic = tpl_topics.get(&topic.topic).unwrap();
                topic.buff_num = tpl_topic.buff_num;
                topic.elem_max_size = tpl_topic.elem_max_size;
                need_update = true;
            }
        }

        if need_update {
            write_config(filename, &config)?;
        }
    }

    Ok(())
}
