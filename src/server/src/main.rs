// main.rs

use libloading::{Library, Symbol};
use server_api::HostFunctions;
use std::ffi::CStr;
use std::os::raw::c_char;

mod plugin_interface;
mod server_api;

const PLUGIN_API_VERSION: u32 = 1;

struct PluginHandle {
    _lib: Library, // Hold the library to keep it loaded
    plugin: *const plugin_interface::Plugin,
    plugin_metadata: plugin_interface::RustTypePluginMetadata,
}

impl PluginHandle {
    unsafe fn new(lib_path: &str) -> Self {
        let lib = Library::new(lib_path).expect("Failed to load plugin library");
        let get_plugin: Symbol<unsafe extern "C" fn() -> *const plugin_interface::Plugin> = lib
            .get(b"get_plugin")
            .expect("Failed to load `get_plugin` symbol");
        let plugin = get_plugin();
        PluginHandle {
            _lib: lib,
            plugin,
            plugin_metadata: ((*plugin).metadata)().into(),
        }
    }
}

struct Server {
    plugins: Vec<PluginHandle>,
    host_functions: HostFunctions,
}

impl Server {
    fn new() -> Self {
        // Define host functions
        let host_functions = HostFunctions {
            version: PLUGIN_API_VERSION,
            log: Server::log_message,
            spawn_entity: Server::spawn_entity,
            get_eventually_modified_number: Server::get_eventually_modified_number,
        };

        Server {
            plugins: Vec::new(),
            host_functions,
        }
    }

    /// Function exposed to plugins for logging.
    extern "C" fn log_message(message: *const c_char) {
        unsafe {
            if message.is_null() {
                eprintln!("Received null pointer in log_message");
                return;
            }
            let c_str = CStr::from_ptr(message);
            match c_str.to_str() {
                Ok(str_slice) => println!("[Plugin Log]: {}", str_slice),
                Err(_) => eprintln!("Received invalid UTF-8 string in log_message"),
            }
        }
    }

    /// Function exposed to plugins for spawning entities.
    extern "C" fn spawn_entity(entity_id: u32) {
        println!("Spawning entity with ID: {}", entity_id);
    }

    /// Function exposed to plugins for getting an eventually modified number.
    extern "C" fn get_eventually_modified_number() -> u32 {
        42
    }

    /// Loads a plugin from the specified path.
    fn load_plugin(&mut self, path: &str) {
        unsafe {
            let plugin_handle = PluginHandle::new(path);
            // Initialize the plugin with host functions
            ((*plugin_handle.plugin).init)(&self.host_functions);
            println!("Loaded plugin from {}", path);
            println!("Plugin metadata: {}", plugin_handle.plugin_metadata);
            self.plugins.push(plugin_handle);
        }
    }

    /// Calls the tick function on all loaded plugins.
    fn tick_plugins(&self) {
        for plugin_handle in &self.plugins {
            unsafe {
                ((*plugin_handle.plugin).tick)();
            }
        }
    }

    /// Calls the shutdown function on all loaded plugins.
    fn shutdown_plugins(&self) {
        for plugin_handle in &self.plugins {
            unsafe {
                ((*plugin_handle.plugin).shutdown)();
            }
        }
    }
}

fn main() {
    let mut server = Server::new();

    // Path to the plugin dynamic library
    // Ensure the plugin is compiled as a dynamic library (`cdylib`)
    #[cfg(not(debug_assertions))]
    let plugin_path = if cfg!(target_os = "windows") {
        "target\\release\\example_plugin.dll"
    } else if cfg!(target_os = "macos") {
        "target/release/libexample_plugin.dylib"
    } else {
        "target/release/libexample_plugin.so"
    };

    #[cfg(debug_assertions)]
    let plugin_path = if cfg!(target_os = "windows") {
        "target\\debug\\example_plugin.dll"
    } else if cfg!(target_os = "macos") {
        "target/debug/libexample_plugin.dylib"
    } else {
        "target/debug/libexample_plugin.so"
    };

    // Load the plugin
    server.load_plugin(plugin_path);

    // Simulate server running
    println!("Server is running. Press Enter to tick plugins.");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    // Tick plugins
    server.tick_plugins();

    println!("Press Enter to shutdown plugins.");
    input.clear();
    std::io::stdin().read_line(&mut input).unwrap();

    // Shutdown plugins
    server.shutdown_plugins();

    println!("Server has shut down.");
}
