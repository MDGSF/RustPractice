pub mod mount_parsers;

/// Type-erased errors.
pub type BoxError = std::boxed::Box<
    dyn std::error::Error // must implement Error to satisfy ?
        + std::marker::Send // needed for threads
        + std::marker::Sync, // needed for threads
>;

#[derive(Clone, Default, Debug)]
pub struct Mount {
    pub device: std::string::String,
    pub mount_point: std::string::String,
    pub file_system_type: std::string::String,
    pub options: std::vec::Vec<std::string::String>,
}
