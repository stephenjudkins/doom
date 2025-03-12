use std::{fs, path::PathBuf};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let all_paths: Vec<PathBuf> = fs::read_dir("doomgeneric")?
        .filter_map(|d| d.ok())
        .map(|d| d.path())
        .collect();
    let c_paths = all_paths.iter().filter(|d| {
        d.file_name()
            .and_then(|f| f.to_str())
            .map(|f| f.ends_with(".c"))
            .unwrap_or(false)
    });
    let h_paths = all_paths.iter().filter(|d| {
        d.file_name()
            .and_then(|f| f.to_str())
            .map(|f| f.ends_with(".h"))
            .unwrap_or(false)
    });

    c_paths
        .clone()
        .chain(h_paths)
        .for_each(|p| println!("cargo:rerun-if-changed={}", p.to_str().unwrap()));

    cc::Build::new()
        .flag("-w")
        .files(c_paths)
        .compile("doomgeneric");

    Ok(())
}
