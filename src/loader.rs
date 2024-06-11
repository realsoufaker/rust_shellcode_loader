use rust_embed::RustEmbed;
use std::fs::{create_dir_all, write, remove_dir_all};
use std::process::Command;


#[derive(RustEmbed)]
#[folder = "src/temp/"]
struct TemplateFiles;

pub fn load(shell: &str, file_name: &str, method: &str) {
    // 设置要替换的占位符和值d
    let custom_code_placeholder = "${shell}";
    let custom_code_value = shell;

    let package_name_placeholder = "${package_name}";
    let package_name_value = file_name;

    let main_path = format!("{}/main.rs", method);
    let cargo_path = format!("{}/Cargo.toml", method);

    // 读取模板文件
    let main_rs = TemplateFiles::get(&main_path).unwrap();
    let cargo_toml = TemplateFiles::get(&cargo_path).unwrap();

    // 将模板文件转换为字符串并替换占位符
    let main_rs_content = std::str::from_utf8(main_rs.data.as_ref()).unwrap()
        .replace(custom_code_placeholder, custom_code_value);
    let cargo_toml_content = std::str::from_utf8(cargo_toml.data.as_ref()).unwrap()
        .replace(package_name_placeholder, package_name_value);

    // 创建项目目录并写入文件
    create_dir_all("project/src").unwrap();
    write("project/src/main.rs", main_rs_content).unwrap();
    write("project/Cargo.toml", cargo_toml_content).unwrap();

    // 编译生成的项目
    let mut cmd = Command::new("cargo")
        .args(["build", "--release", "-Z", "unstable-options", "--out-dir", "../"])
        // .arg("--release")
        .current_dir("project")
        .spawn()
        .expect("Failed to compile the project");

    cmd.wait().expect("Failed to wait on child process");
    let _ = remove_dir_all("project");
}