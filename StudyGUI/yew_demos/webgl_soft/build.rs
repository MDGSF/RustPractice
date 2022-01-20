use anyhow::{anyhow, Result};
use std::fs;
use std::process::Command;

fn main() -> Result<()> {
    compile_glsl()?;
    format_rust_code();
    Ok(())
}

fn format_rust_code() {
    Command::new("cargo")
        .args(&["fmt", "--", "src/*.rs"])
        .status()
        .expect("cargo fmt failed");
}

const GLSL_VERT_INPUT_FILE: &'static str = "./glsl/vert.glsl";
const GLSL_FRAG_INPUT_FILE: &'static str = "./glsl/frag.glsl";
const GLSL_VERT_OUTPUT_FILE: &'static str = "./glsl/compiled_vert.glsl";
const GLSL_FRAG_OUTPUT_FILE: &'static str = "./glsl/compiled_frag.glsl";

fn compile_glsl() -> Result<()> {
    let vert_code = fs::read_to_string(GLSL_VERT_INPUT_FILE)?;
    let frag_code = fs::read_to_string(GLSL_FRAG_INPUT_FILE)?;

    let compiled_vert_code = glsl_preprocess_include(&vert_code)?;
    let compiled_frag_code = glsl_preprocess_include(&frag_code)?;

    fs::write(GLSL_VERT_OUTPUT_FILE, compiled_vert_code)?;
    fs::write(GLSL_FRAG_OUTPUT_FILE, compiled_frag_code)?;

    Ok(())
}

fn glsl_preprocess_include(code: &str) -> Result<String> {
    let mut result = String::new();
    for line in code.lines() {
        if line.starts_with("include") {
            result.push('\n');
            result.push_str(&glsl_preprocess_include_process_one_line(line)?);
            result.push('\n');
        } else {
            result.push_str(line);
            result.push('\n');
        }
    }
    Ok(result)
}

fn glsl_preprocess_include_process_one_line(line: &str) -> Result<String> {
    let v: Vec<&str> = line.split(' ').collect();
    if v.len() != 2 {
        return Err(anyhow!("Invalid glsl include syntax"));
    }
    let filename = v[1].replace("\"", "");
    Ok(fs::read_to_string(filename)?)
}
