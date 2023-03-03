//use std::env;
use std::fs;
use cargo_metadata::{Metadata, MetadataCommand};

fn main() {
    // 获取 Cargo.toml 中的版本号
    let metadata: Metadata = MetadataCommand::new()
            .no_deps()
            .exec()
            .unwrap();

    let version = metadata.packages[0].version.to_string();

    //// 将版本号设置为 BUILD_VERSION 环境变量
    //env::set_var("BUILD_VERSION", version);
    //// 获取 BUILD_VERSION 环境变量的值
    //let version = env::var("BUILD_VERSION").unwrap();
    //println!("cargo:rustc-env=BUILD_VERSION={}", version);

    let now = chrono::Local::now().format("%y.%m%d.%H%M%S");
    println!("BUILD_TIMESTAMP: {} {}", version, now);

    let ver_btstamp = String::from(
            format!(
                "pub const VERSION: &str = \"v{} (built on {})\";\n"
                    , version
                    , now
                ));

    //// 读取文件内容
    //let mut contents = fs::read_to_string("src/act/version.rs").unwrap();
    //// 将版本信息插入到指定位置
    //contents = contents.replace("#VERSION#", &version);
    //// 将更改后的内容写回文件
    //fs::write("src/act/version.rs", contents).unwrap();
    fs::write("src/act/version.rs", ver_btstamp).unwrap();

    //write!(&mut f, "pub const VERSION: &str = \"{}\";\n", version).unwrap();
}