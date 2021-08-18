use std::error::Error;

fn is_elf(binary: &[u8]) -> bool {
  match binary {
    [0x7f, b'E', b'L', b'F', ..] => true,
    _ => false,
  }
}

fn main() -> Result<(), Box<dyn Error>> {
  let current_exe = std::env::current_exe()?;
  let binary = std::fs::read(&current_exe)?;

  if is_elf(&binary) {
    println!("{} is an ELF binary", current_exe.display());
  } else {
    println!("{} is not an ELF binary", current_exe.display());
  }

  Ok(())
}
