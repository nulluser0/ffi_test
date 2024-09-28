fn main() {
    // let crate_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();

    // println!("cargo:rerun-if-changed=src/server_api.rs");
    // println!("cargo:rerun-if-changed=src/plugin_interface.rs");

    // // create C header files for the plugins API
    // // cbindgen::Builder::new()
    // //     .with_crate(crate_dir)
    // //     .with_language(cbindgen::Language::C)
    // //     .with_include_guard("PLUGIN_INTERFACE_H")
    // //     .generate()
    // //     .expect("Unable to generate bindings")
    // //     .write_to_file("include/plugin_interface.h");

    // let config: cbindgen::Config = cbindgen::Config {
    //     language: cbindgen::Language::C,
    //     ..Default::default()
    // };

    // cbindgen::generate_with_config(&crate_dir, config)
    //     .expect("Unable to generate bindings")
    //     .write_to_file("include/plugin_interface.h");
}
