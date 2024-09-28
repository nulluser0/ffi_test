use std::{ffi::c_char, fmt::Display};

use crate::server_api::HostFunctions;

#[repr(C)]
pub struct PluginMetadata {
    pub name: *mut c_char,
    pub author: *mut c_char,
    pub description: *mut c_char,
    pub version: *mut c_char,
}

pub struct RustTypePluginMetadata {
    pub name: &'static str,
    pub author: &'static str,
    pub description: &'static str,
    pub version: &'static str,
}

impl From<PluginMetadata> for RustTypePluginMetadata {
    fn from(metadata: PluginMetadata) -> Self {
        let name = unsafe {
            std::ffi::CString::from_raw(metadata.name)
                .into_string()
                .unwrap()
        };
        let author = unsafe {
            std::ffi::CString::from_raw(metadata.author)
                .into_string()
                .unwrap()
        };
        let description = unsafe {
            std::ffi::CString::from_raw(metadata.description)
                .into_string()
                .unwrap()
        };
        let version = unsafe {
            std::ffi::CString::from_raw(metadata.version)
                .into_string()
                .unwrap()
        };

        RustTypePluginMetadata {
            name: Box::leak(name.into_boxed_str()),
            author: Box::leak(author.into_boxed_str()),
            description: Box::leak(description.into_boxed_str()),
            version: Box::leak(version.into_boxed_str()),
        }
    }
}

impl Display for RustTypePluginMetadata {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Plugin {{ name: {}, author: {}, description: {}, version: {} }}",
            self.name, self.author, self.description, self.version,
        )
    }
}

/// These are the functions that a plugin must implement.
/// They will be called by the server at various points.
#[repr(C)]
pub struct Plugin {
    /// Returns metadata about the plugin.
    pub metadata: extern "C" fn() -> PluginMetadata,

    /// Called when the plugin is initialized. Receives a pointer to HostFunctions.
    pub init: extern "C" fn(host: *const HostFunctions),

    /// Called during game tick.
    pub tick: extern "C" fn(),

    /// Called when the plugin is shutting down.
    pub shutdown: extern "C" fn(),
}
