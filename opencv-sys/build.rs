fn main() -> miette::Result<()> {
    let mut include_paths: Vec<std::path::PathBuf> = vec![std::path::PathBuf::from("src")];
    let mut libs: Vec<String> = vec![];

    let mut opencv: pkg_config::Library = pkg_config::probe_library("opencv4").unwrap();
    include_paths.append(&mut opencv.include_paths);
    libs.append(&mut opencv.libs);

    let mut builder = autocxx_build::Builder::new("src/lib.rs", include_paths).build()?;
    builder.cpp(true).std("c++20").compile("opencv-sys");

    libs.iter()
        .for_each(|x: &String| println!("cargo:rustc-link-lib={}", x));

    println!("cargo:rerun-if-changed=src/lib.rs");

    Ok(())
}
