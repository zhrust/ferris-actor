use std::fs;
use std::env;
use std::path::PathBuf;
use toml::Value;

pub fn get_version() -> String {
    //let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));

    // 获取当前工作目录
    let current_dir = env::current_dir().unwrap();
    log::debug!("current_dir:\n\t {:}", current_dir.display());
    // 将当前工作目录转换为 PathBuf 对象
    let mut path_buf = PathBuf::from(current_dir);
    log::debug!("PathBuf:src/act/version\n\t{:?}", path_buf);
    //let version = "0.0.42";
    //path_buf.pop(); // Move up one level to src/
    //path_buf.pop(); // Move up one level to project root
    path_buf.push("Cargo.toml");
    log::debug!("PathBuf:Cargo.toml\n\t{:?}", path_buf);

    let cargo_toml = fs::read_to_string(path_buf).unwrap();
    let cargo_toml: Value = cargo_toml.parse().unwrap();

    let version = cargo_toml
        .get("package")
        .and_then(|package| package.get("version"))
        .and_then(|version| version.as_str())
        .unwrap_or("0.0.0");

    version.to_string()
}
