use std::process::Command;

fn main() {
    // 每次构建时重新生成 CSS，确保资源文件是最新的
    println!("cargo:rerun-if-changed=tailwind.config.js");
    println!("cargo:rerun-if-changed=input.css");
    println!("cargo:rerun-if-changed=templates/");

    // 获取当前 crate 目录 (server/web/)
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set");
    let web_dir = std::path::Path::new(&manifest_dir);

    // tailwindcss binary 在当前目录
    let tailwind_path = web_dir.join("tailwindcss");

    // 输入和输出路径
    let input_path = web_dir.join("input.css");
    let output_path = web_dir.join("assets/css/app.css");

    // 确保输出目录存在
    if let Some(parent) = output_path.parent() {
        std::fs::create_dir_all(parent).expect("Failed to create output directory");
    }

    // 检查 tailwindcss binary 是否存在
    if !tailwind_path.exists() {
        println!(
            "cargo:warning=tailwindcss binary not found at {:?}, skipping CSS generation",
            tailwind_path
        );
        return;
    }

    // 运行 tailwindcss
    let output = Command::new(&tailwind_path)
        .args([
            "-i",
            input_path.to_str().unwrap(),
            "-o",
            output_path.to_str().unwrap(),
            "--minify",
        ])
        .current_dir(web_dir)
        .output();

    match output {
        Ok(result) => {
            if result.status.success() {
                println!("cargo:info=Successfully generated CSS at {:?}", output_path);
            } else {
                let stderr = String::from_utf8_lossy(&result.stderr);
                println!("cargo:warning=tailwindcss failed: {}", stderr);
            }
        }
        Err(e) => {
            println!("cargo:warning=Failed to run tailwindcss: {}", e);
        }
    }
}
