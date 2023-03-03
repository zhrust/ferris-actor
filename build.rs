//use std::env;
use std::fs;
//use std::path::Path;
//use std::path::{Path, PathBuf};
//use cargo_metadata::{Metadata, MetadataCommand, Package};
//use cargo_metadata::Package;
use cargo_metadata::{Metadata, MetadataCommand};

//extern crate chrono;
//use chrono::prelude::*;
////use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};
////use chrono::{DateTime, Duration, Utc};
//use chrono::Local;//, NaiveDate};

fn main() {
    // 获取 Cargo.toml 中的版本号
    let metadata: Metadata = MetadataCommand::new()
            .no_deps()
            .exec()
            .unwrap();

    let version = metadata.packages[0].version.to_string();
    //let bts = metadata.packages[0].bts.to_string();


/*
    let pkg = metadata.packages.pop().unwrap();
    let build_version = pkg
        .manifest_path
        .as_ref()
        .map(Path::new)
        .and_then(|path| Package::from_manifest_path(path).ok())
        .and_then(|pkg| pkg.manifest().metadata().get("BUILD_VERSION").map(|v| v.to_string()))
        .unwrap_or_else(|| "unknown".to_string());
let build_version = <Utf8PathBuf as AsRef<Path>>::as_ref(&pkg.manifest_path)
        .as_ref()
        .map(Path::new)
        .and_then(|path| Package::from_manifest_path(path).ok())
        .and_then(|pkg| pkg.manifest().metadata().get("BUILD_VERSION").cloned())
        .unwrap_or_else(|| env!("CARGO_PKG_VERSION").to_string());


    println!("Build version: {}", build_version);
 */
    
    //// 将版本号设置为 BUILD_VERSION 环境变量
    //env::set_var("BUILD_VERSION", version);
    //// 获取 BUILD_VERSION 环境变量的值
    //let version = env::var("BUILD_VERSION").unwrap();
    //println!("cargo:rustc-env=BUILD_VERSION={}", version);

    //let now = chrono::Local::now().format("%y.%m%d.%H%M%S");
    //println!("BUILD_TIMESTAMP: {} {}", version, build_version);
    // 读取文件内容
    let mut contents = fs::read_to_string("src/act/version.rs").unwrap();

    // 将版本信息插入到指定位置
    contents = contents.replace("#VERSION#", &version);

    // 将更改后的内容写回文件
    fs::write("src/act/version.rs", contents).unwrap();
}


//write!(&mut f, "pub const VERSION: &str = \"{}\";\n", version).unwrap();