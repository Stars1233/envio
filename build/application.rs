use std::path::PathBuf;

pub fn export_completion_paths() -> Result<(), Box<dyn std::error::Error>> {
    let completions_base = PathBuf::from("../../../completions");
    println!(
        "cargo:rustc-env=ENVIO_GENERATED_COMPLETION_BASH={}",
        completions_base.join("envio.bash").display()
    );
    println!(
        "cargo:rustc-env=ENVIO_GENERATED_COMPLETION_FISH={}",
        completions_base.join("envio.fish").display()
    );
    println!(
        "cargo:rustc-env=ENVIO_GENERATED_COMPLETION_ZSH={}",
        completions_base.join("_envio").display()
    );
    println!(
        "cargo:rustc-env=ENVIO_GENERATED_COMPLETION_PS1={}",
        completions_base.join("_envio.ps1").display()
    );

    Ok(())
}

pub fn export_build_env_vars() {
    for var in &[
        "PROFILE",
        "TARGET",
        "CARGO_CFG_TARGET_FAMILY",
        "CARGO_CFG_TARGET_OS",
        "CARGO_CFG_TARGET_ARCH",
        "CARGO_CFG_TARGET_POINTER_WIDTH",
        "CARGO_CFG_TARGET_ENDIAN",
        "CARGO_CFG_TARGET_FEATURE",
        "HOST",
    ] {
        println!(
            "cargo:rustc-env={}={}",
            var,
            std::env::var(var).unwrap_or_else(|_| "unknown".into())
        );
    }

    let build_timestamp: String = chrono::Local::now()
        .naive_local()
        .format("%Y-%m-%d %H:%M:%S")
        .to_string();

    println!("cargo:rustc-env=BUILD_TIMESTAMP={}", build_timestamp);
}
