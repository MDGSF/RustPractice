extern crate clap;

use anyhow::Result;
use clap::{App, Arg};
use serde::{Deserialize, Serialize};
use std::ffi::OsStr;
use std::fs;
use std::path::Path;
use std::sync::atomic::{AtomicUsize, Ordering};

#[derive(Serialize, Deserialize, Debug)]
struct Task {
  repeat: i32,
  detect_flag: String,
  name: String,
  source: String,
  path: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct TaskList {
  #[serde(rename = "data")]
  task_list: Vec<Task>,
}

fn main() -> Result<()> {
  let matches = App::new("gen_task_list")
    .version("1.0")
    .author("HuangJian <1342042894@qq.com>")
    .about("Generates task.json")
    .arg(
      Arg::with_name("video")
        .short("d")
        .long("video")
        .value_name("VIDEO DIRECTORY")
        .help("Sets the video directory")
        .required(true)
        .takes_value(true),
    )
    .arg(
      Arg::with_name("output_dir")
        .short("o")
        .long("output_dir")
        .value_name("OUTPUT DIRECTORY")
        .help("Sets the output directory")
        .required(true)
        .takes_value(true),
    )
    .get_matches();

  let video_dir = matches.value_of("video").unwrap();
  let video_dir = Path::new(video_dir);

  let output_dir = matches.value_of("output_dir").unwrap();
  let output_dir = Path::new(output_dir);

  walk_dir(&video_dir, &output_dir)?;

  Ok(())
}

static GLOBAL_VIDEO_COUNT: AtomicUsize = AtomicUsize::new(1);

fn walk_dir(directory: &Path, output_dir: &Path) -> Result<()> {
  for entry in fs::read_dir(directory)? {
    let entry = entry?;
    let path = entry.path();
    if path.is_dir() {
      let video_dir = Path::new(directory).join(path.to_path_buf());
      if path.file_name() == Some(OsStr::new("video")) {
        let count = GLOBAL_VIDEO_COUNT.fetch_add(1, Ordering::SeqCst);

        let task = Task {
          repeat: 1,
          detect_flag: video_dir
            .clone()
            .join("base.flag")
            .to_str()
            .unwrap()
            .to_string(),
          name: format!("task{}", count),
          source: video_dir.to_str().unwrap().to_string(),
          path: video_dir
            .clone()
            .join("log.txt")
            .to_str()
            .unwrap()
            .to_string(),
        };

        let mut task_list = TaskList {
          task_list: Vec::new(),
        };
        task_list.task_list.push(task);

        let serialized = serde_json::to_string_pretty(&task_list).unwrap();

        let task_name = video_dir
          .clone()
          .join("task.json")
          .to_str()
          .unwrap()
          .replace("/", "_");
        let task_path = output_dir.clone().join(count.to_string() + &task_name);

        fs::write(task_path, serialized)?;
      } else {
        walk_dir(&video_dir, &output_dir)?;
      }
    }
  }

  Ok(())
}
