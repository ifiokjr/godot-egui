use std::fs::{create_dir, File};
use std::io::{BufWriter, Write};

use cargo_toml::Manifest;
const EGUI_PKG_NAME: &str = "egui";

// To help ensure that we can demonstrate compatibility and emit relevant errors when importing egui themes, we need to get some version information about of the lockfile this was built with.
fn main() {
    // TODO: Try to make this work with the cargo lock files as they will have more accurate depenency information.
    let toml_path = concat!(env!("CARGO_MANIFEST_DIR"), "/Cargo.toml");
    let manifest = Manifest::from_path(toml_path).unwrap_or_else(|_| {
        panic!("cargo.toml path `{toml_path}` should exist before the build kicks off");
    });
    // We need to ensure that the generated directory is created properly.
    let directory = std::env::var("OUT_DIR").expect("OUT_DIR must exist");
    let directory = directory + "/generated";
    let _ = create_dir(directory.as_str());
    let filepath = directory + "/meta.rs";
    let out_file = File::create(filepath).expect("cannot create file");

    let mut writer = BufWriter::new(out_file);
    writeln!(writer, "// THIS IS AN AUTOGENERATED FILE AND SHOULD NOT BE MODIFIED OR SAVED")
        .expect("failed to write");

    let version = manifest.package.unwrap().version.unwrap();
    // env!("CARGO_PKG_VERSION"); //&egui_theme_pkg.version;
    writeln!(writer, "const EGUI_THEME_VERSION: &str = \"{version}\";").expect("failed to write");

    let version = manifest
        .dependencies
        .get(EGUI_PKG_NAME)
        .expect("could not find egui")
        .detail()
        .unwrap()
        .version
        .clone()
        .unwrap_or_default();

    // env!("CARGO_PKG_DEP_EGUI_VERSION"); //&egui_pkg.version;
    writeln!(writer, "const EGUI_VERSION: &str = \"{version}\";").expect("failed to write");
}
