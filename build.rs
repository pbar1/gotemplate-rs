use std::env;
use std::path::Path;
use std::process::Command;

fn main() {
    if !Path::new("go/libgotemplate.h").exists() || !Path::new("go/libgotemplate.a").exists() {
        Command::new("go")
            .args(&[
                "build",
                "-buildmode=c-archive",
                "-o",
                "libgotemplate.a",
                "gotemplate.go",
            ])
            .current_dir("go")
            .status()
            .expect("failed to go build");
    }

    let target_os = env::var("CARGO_CFG_TARGET_OS");
    match target_os.as_ref().map(|x| &**x) {
        Ok("macos") => {
            println!("cargo:rustc-link-lib=framework={}", "CoreFoundation");
            println!("cargo:rustc-link-lib=framework={}", "Security");
        }
        _ => {}
    }

    println!("cargo:rustc-link-search=native={}", "./go");
    println!("cargo:rustc-link-lib=static={}", "gotemplate");
}
