use std::os::raw::c_char;

const PLUGIN_API_VERSION: u32 = 1;

const PLUGIN_NAME: &str = "Example Plugin";
const PLUGIN_AUTHOR: &str = "Your Name";
const PLUGIN_DESCRIPTION: &str = "An example plugin for the server.";
const PLUGIN_VERSION: &str = "0.1.0";

#[repr(C)]
pub struct HostFunctions {
    pub version: u32,
    pub log: extern "C" fn(message: *const c_char),
    pub spawn_entity: extern "C" fn(entity_id: u32),
    pub get_eventually_modified_number: extern "C" fn() -> u32,
}

#[repr(C)]
pub struct PluginMetadata {
    pub name: *mut c_char,
    pub author: *mut c_char,
    pub description: *mut c_char,
    pub version: *mut c_char,
}

#[repr(C)]
pub struct Plugin {
    pub metadata: extern "C" fn() -> PluginMetadata,
    pub init: extern "C" fn(host: *const HostFunctions),
    pub tick: extern "C" fn(),
    pub shutdown: extern "C" fn(),
}

#[no_mangle]
pub extern "C" fn get_plugin() -> *const Plugin {
    &PLUGIN
}

static PLUGIN: Plugin = Plugin {
    metadata: plugin_metadata,
    init: plugin_init,
    tick: plugin_tick,
    shutdown: plugin_shutdown,
};

static mut HOST_FUNCTIONS: Option<*const HostFunctions> = None;

extern "C" fn plugin_metadata() -> PluginMetadata {
    let name = std::ffi::CString::new(PLUGIN_NAME).unwrap();
    let author = std::ffi::CString::new(PLUGIN_AUTHOR).unwrap();
    let description = std::ffi::CString::new(PLUGIN_DESCRIPTION).unwrap();
    let version = std::ffi::CString::new(PLUGIN_VERSION).unwrap();

    PluginMetadata {
        name: name.into_raw(),
        author: author.into_raw(),
        description: description.into_raw(),
        version: version.into_raw(),
    }
}

extern "C" fn plugin_init(host: *const HostFunctions) {
    unsafe {
        HOST_FUNCTIONS = Some(host);
    }

    unsafe {
        if let Some(host) = HOST_FUNCTIONS {
            if ((*host).version) != PLUGIN_API_VERSION {
                panic!("Plugin API version mismatch!");
            }
        } else {
            panic!("Host functions not set!");
        }
    }

    // Use host functions
    log("Plugin initialized!");
    spawn_entity(1001);
}

extern "C" fn plugin_tick() {
    log("Plugin tick called.");
    // Example: Spawn another entity during update
    log(get_eventually_modified_number().to_string().as_str());
    spawn_entity(1002);
}

extern "C" fn plugin_shutdown() {
    log("Plugin shutting down.");
}

fn log(message: &str) {
    unsafe {
        if let Some(host) = HOST_FUNCTIONS {
            let c_string = std::ffi::CString::new(message).unwrap();
            ((*host).log)(c_string.as_ptr());
        }
    }
}

fn spawn_entity(entity_id: u32) {
    unsafe {
        if let Some(host) = HOST_FUNCTIONS {
            ((*host).spawn_entity)(entity_id);
        }
    }
}

fn get_eventually_modified_number() -> u32 {
    unsafe {
        if let Some(host) = HOST_FUNCTIONS {
            ((*host).get_eventually_modified_number)()
        } else {
            0
        }
    }
}
