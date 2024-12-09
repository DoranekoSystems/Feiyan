extern crate cc;
use std::process::Command;

fn get_git_hash() -> String {
    let output = Command::new("git").args(&["rev-parse", "HEAD"]).output();

    match output {
        Ok(output) if output.status.success() => {
            String::from_utf8_lossy(&output.stdout).trim().to_string()
        }
        _ => "unknown".to_string(),
    }
}

fn main() {
    println!("cargo:rustc-link-search=native=/usr/local/lib");
    let git_hash = get_git_hash();
    println!("cargo:rustc-env=GIT_HASH={}", git_hash);

    let mut build = cc::Build::new();
    let target_os = std::env::var("CARGO_CFG_TARGET_OS").unwrap();
    println!("cargo:rustc-env=TARGET_OS={}", target_os);

    if cfg!(windows) {
        println!("cargo:rustc-cfg=host_os=\"windows\"");
    }

    if target_os == "windows" {
        build.flag("/std:c++17").flag("/W4").flag("/Zi");
    } else {
        build.flag("-std=c++17").flag("-Wall").flag("-v").flag("-g");
    }
    match target_os.as_str() {
        "windows" => {
            if build.get_compiler().is_like_msvc() {
                build.flag("/FS"); // Fixes the PDB write issue
                build.flag("/EHsc"); // Enables proper exception handling
            }
            build.file("src/cpp/src/windows/native_api.cpp");
            build.file("src/cpp/src/windows/file_api.cpp");
            build.file("src/cpp/src/windows/debugger.cpp");
        }
        "macos" => {
            println!("cargo:rustc-link-arg=-lc++");
            println!("cargo:rustc-link-arg=-framework");
            println!("cargo:rustc-link-arg=Foundation");
            println!("cargo:rustc-link-arg=-framework");
            println!("cargo:rustc-link-arg=AVFoundation");
            println!("cargo:rustc-link-arg=-framework");
            println!("cargo:rustc-link-arg=CoreMedia");
            println!("cargo:rustc-link-arg=-framework");
            println!("cargo:rustc-link-arg=BackgroundTasks");
            println!("cargo:rustc-link-arg=-framework");
            println!("cargo:rustc-link-arg=SystemConfiguration");
            build.file("src/cpp/src/darwin/native_api.mm");
            build.file("src/cpp/src/darwin/file_api.mm");
            build.file("src/cpp/src/darwin/debugger.mm");
            build.file("src/cpp/src/common/util.cpp");
        }
        "ios" => {
            println!("cargo:rustc-link-arg=-mios-version-min=13.0");
            println!("cargo:rustc-link-arg=-lc++");
            println!("cargo:rustc-link-arg=-framework");
            println!("cargo:rustc-link-arg=Foundation");
            println!("cargo:rustc-link-arg=-framework");
            println!("cargo:rustc-link-arg=UIKit");
            println!("cargo:rustc-link-arg=-framework");
            println!("cargo:rustc-link-arg=AVFoundation");
            println!("cargo:rustc-link-arg=-framework");
            println!("cargo:rustc-link-arg=CoreMedia");
            println!("cargo:rustc-link-arg=-framework");
            println!("cargo:rustc-link-arg=BackgroundTasks");
            println!("cargo:rustc-link-arg=-framework");
            println!("cargo:rustc-link-arg=SystemConfiguration");
            build.file("src/cpp/src/darwin/native_api.mm");
            build.file("src/cpp/src/darwin/file_api.mm");
            build.file("src/cpp/src/darwin/debugger.mm");
            build.file("src/cpp/src/common/util.cpp");
        }

        "android" => {
            build.cpp_link_stdlib("stdc++");
            println!("cargo:rustc-link-lib=static=c++_static");
            println!("cargo:rustc-link-lib=static=c++abi");
            println!("cargo:rustc-link-lib=static=c++");
            build.flag_if_supported("-DTARGET_IS_ANDROID");
            build.file("src/cpp/src/linux/native_api.cpp");
            build.file("src/cpp/src/linux/file_api.cpp");
            build.file("src/cpp/src/linux/debugger.cpp");
        }

        "linux" => {
            build.cpp(true);
            println!("cargo:rustc-link-arg=-lstdc++");
            build.file("src/cpp/src/linux/native_api.cpp");
            build.file("src/cpp/src/linux/file_api.cpp");
            build.file("src/cpp/src/linux/debugger.cpp");
        }

        _ => {
            panic!("Unsupported target OS");
        }
    }

    build.compile("libnative.a");
}
