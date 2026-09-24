use std::{env, fs, path::PathBuf};

const CHIPS: &[(&str, &str)] = &[("hk32f030mf4p6", "hk32f030mf4p6")];

fn main() {
    let selected = CHIPS
        .iter()
        .filter(|(feature, _)| {
            let variable = format!(
                "CARGO_FEATURE_{}",
                feature.replace('-', "_").to_ascii_uppercase()
            );
            env::var_os(variable).is_some()
        })
        .collect::<Vec<_>>();

    if selected.len() != 1 {
        panic!(
            "enable exactly one HK32 chip feature; enabled: {:?}",
            selected
        );
    }

    let manifest_dir = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap());
    let chip_dir = manifest_dir.join("src/chips").join(selected[0].1);
    println!("cargo:rustc-link-search={}", chip_dir.display());
    println!(
        "cargo:rerun-if-changed={}",
        chip_dir.join("pac.rs").display()
    );
    println!(
        "cargo:rerun-if-changed={}",
        chip_dir.join("device.x").display()
    );
    println!(
        "cargo:rerun-if-changed={}",
        chip_dir.join("memory.x").display()
    );

    let selected_source = format!(
        "include!({:#?});\n",
        chip_dir.join("pac.rs").to_string_lossy()
    );
    fs::write(
        PathBuf::from(env::var_os("OUT_DIR").unwrap()).join("selected.rs"),
        selected_source,
    )
    .unwrap();
}
