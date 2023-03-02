use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;

fn get_version_from_cargo_toml(cargo_toml: &str) -> Option<&str> {
    let toml: toml::Value = toml::from_str(cargo_toml).ok()?;
    toml.get("package")?.get("version")?.as_str()
}

fn main() {
    let is_release_build = std::env::var("PROFILE").unwrap() == "release";
    //let out_dir = std::env::var("OUT_DIR").unwrap();

    let out_dir = if is_release_build {
            std::env::var("OUT_DIR").unwrap()
        } else {
            std::env::var("CARGO_MANIFEST_DIR").unwrap()
        };

    let snap_template = include_str!("src/act/snap.rs");
    let snap_template_path = Path::new(&out_dir).join("snap_template.rs");
    let mut f = File::create(&snap_template_path).unwrap();
    f.write_all(snap_template.as_bytes()).unwrap();


    if is_release_build {
        let version = env::var("CARGO_PKG_VERSION").unwrap();
        let dest_path = Path::new(&out_dir).join("version.rs");
        let mut f = File::create(&dest_path).unwrap();
        write!(&mut f, "pub const VERSION: &str = \"{}\";\n", version).unwrap();
    } else {
        let dest_path = Path::new(&out_dir).join("version.rs");
        let mut f = File::create(&dest_path).unwrap();
        write!(&mut f, "pub const VERSION: &str = \"development\";\n").unwrap();
    }


    // Set the new environment variable for use in snap.rs
    let snap_out_dir = Path::new(&out_dir).join("snap");
    std::fs::create_dir_all(&snap_out_dir).unwrap();
    std::env::set_var("SNAP_OUT_DIR", snap_out_dir);

    // Set the OUT_DIR and SNAP_OUT_DIR environment variables for use in snap.rs
    println!("cargo:rustc-env=OUT_DIR={}", out_dir);
    println!("cargo:rustc-env=SNAP_OUT_DIR={}", snap_out_dir.display());
    println!("cargo:rerun-if-changed=build.rs");
    
}


