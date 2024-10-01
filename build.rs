use std::{env, path::Path};

fn main() {
    #[cfg(feature = "enable")]
    {
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
        }
    }
}
