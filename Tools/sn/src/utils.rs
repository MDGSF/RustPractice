#[allow(dead_code)]
pub fn path_exists(path: &str) -> bool {
    std::path::Path::new(path).exists()
}

#[allow(dead_code)]
pub fn dir_exists(path: &str) -> bool {
    std::path::Path::new(path).is_dir()
}

pub fn file_exists(path: &str) -> bool {
    std::path::Path::new(path).is_file()
}

pub fn guess_shell() -> String {
    if file_exists("/bin/bash") {
        return "/bin/bash".to_string();
    } else if file_exists("/usr/bin/bash") {
        return "/usr/bin/bash".to_string();
    } else if file_exists("/bin/sh") {
        return "/bin/sh".to_string();
    } else if file_exists("/usr/bin/sh") {
        return "/usr/bin/sh".to_string();
    } else if file_exists("/system/bin/sh") {
        return "/system/bin/sh".to_string();
    }
    "sh".to_string()
}
