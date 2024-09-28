# ffi_test

simple ffi test for plugins lol

basically, plugins have to implement some extern "C" functions. The server (or plugins system) will call these functions.

```rust
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
```

for the plugin to interface with host (server) functions (aka plugins API), a HostFunctions struct is given to the plugin via the init function.

It will contain host functions, as well as the API version, and more stuff that can be added.

For example, a function for getting the list of available plugins, or checking a player's position can be added.

```rust
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
```

Two crates in this workspace.
.
- `server`: the server crate containing the plugins system and its api. It dynamically loads plugins (DLL/SO/DYLIB files).
- `example_plugin`: an example plugin which shows how the plugin would interface with the server's plugins API.

just run `cargo run` or `cargo run --release` to see it in action.
