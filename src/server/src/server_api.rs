use std::ffi::c_char;

/// These are the functions that the server provides to plugins.
#[repr(C)]
pub struct HostFunctions {
    /// Version of the plugin interface.
    pub version: u32,

    /// Function for logging messages from plugins.
    pub log: extern "C" fn(message: *const c_char),

    /// Function for spawning an entity in the server.
    pub spawn_entity: extern "C" fn(entity_id: u32),

    /// Function for getting an eventually modified number from the server.
    pub get_eventually_modified_number: extern "C" fn() -> u32,
}
