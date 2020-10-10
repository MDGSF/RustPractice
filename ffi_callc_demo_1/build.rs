fn main() {
  println!(r"cargo:rustc-link-search=./src/ffi_array");
  println!(r"cargo:rustc-link-search=./src/ffi_opaque");
}
