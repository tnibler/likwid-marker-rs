use std::{env, path::Path};

fn main() {
    if let Ok(lib_dir) = env::var("LIKWID_LIB_DIR") {
        let dylib_name = format!(
            "{}likwid{}",
            env::consts::DLL_PREFIX,
            env::consts::DLL_SUFFIX
        );
        if !Path::new(&lib_dir).join(&dylib_name).exists() {
            println!("cargo:warning=LIKWID_LIB_DIR ({lib_dir}) does not contain {dylib_name}.");
        }
        println!("cargo:rustc-link-search=native={}", lib_dir);
        println!("cargo:rustc-link-lib=likwid");
    }
    if let Ok(include_dir) = env::var("LIKWID_INCLUDE_DIR") {
        println!("cargo:include={include_dir}");
    } else {
        match pkg_config::probe_library("likwid") {
            Ok(info) => {
                let joined_paths = env::join_paths(&info.include_paths)
                    .ok()
                    .and_then(|p| p.into_string().ok());
                if let Some(joined_paths) = joined_paths {
                    println!("cargo:include={joined_paths}");
                } else {
                    println!("cargo:warning=Got invalid/non-unicode include paths for likwid from pkg-config\nPaths: {:?}", info.include_paths);
                }
                for lib_path in &info.link_paths {
                    println!("cargo:rustc-link-search=native={}", lib_path.display());
                }
                println!("cargo:rustc-link-lib=likwid");
            }
            Err(err) => {
                println!("cargo:warning=pkg-config probe for likwid failed: {err}");
            }
        }
    }
}
