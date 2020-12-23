extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
  println!("cargo:rustc-link-lib=nng");
  println!("cargo:rustc-link-search=./libnng/ubuntu16.04");

  // Tell cargo to invalidate the built crate whenever the wrapper changes
  println!("cargo:rerun-if-changed=wrapper.h");

  let bindings = bindgen::Builder::default()
    .clang_arg("-I./libnng/include")
    .header("wrapper.h")
    .derive_default(true)
    .prepend_enum_name(false)
    .default_enum_style(bindgen::EnumVariation::Rust {
      non_exhaustive: false,
    })
    .constified_enum("nng_flag_enum")
    .generate()
    .expect("Unable to generate bindings");

  let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
  bindings
    .write_to_file(out_path.join("bindings.rs"))
    .expect("Couldn't write bindings!");
}
