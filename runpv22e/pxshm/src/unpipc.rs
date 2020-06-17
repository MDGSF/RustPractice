use libc;

/* default permissions for new files */
pub const FILE_MODE: u32 = libc::S_IRUSR | libc::S_IWUSR | libc::S_IRGRP | libc::S_IROTH;
