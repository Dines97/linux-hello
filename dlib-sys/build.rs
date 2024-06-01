fn main() -> miette::Result<()> {
    let mut include_paths: Vec<std::path::PathBuf> = vec![std::path::PathBuf::from("src")];
    let mut libs: Vec<String> = vec![];

    let mut opencv: pkg_config::Library = pkg_config::probe_library("opencv4").unwrap();
    include_paths.append(&mut opencv.include_paths);
    libs.append(&mut opencv.libs);

    let mut dlib: pkg_config::Library = pkg_config::probe_library("dlib-1").unwrap();
    include_paths.append(&mut dlib.include_paths);
    libs.append(&mut dlib.libs);

    let mut blas: pkg_config::Library = pkg_config::probe_library("blas").unwrap();
    include_paths.append(&mut blas.include_paths);
    libs.append(&mut blas.libs);

    let mut x11: pkg_config::Library = pkg_config::probe_library("x11").unwrap();
    include_paths.append(&mut x11.include_paths);
    libs.append(&mut x11.libs);

    let mut cuda: pkg_config::Library = pkg_config::probe_library("cuda").unwrap();
    include_paths.append(&mut cuda.include_paths);
    libs.append(&mut cuda.libs);

    let mut cudart: pkg_config::Library = pkg_config::probe_library("cudart").unwrap();
    include_paths.append(&mut cudart.include_paths);
    libs.append(&mut cudart.libs);

    let mut cusolver: pkg_config::Library = pkg_config::probe_library("cusolver").unwrap();
    include_paths.append(&mut cusolver.include_paths);
    libs.append(&mut cusolver.libs);

    let mut cublas: pkg_config::Library = pkg_config::probe_library("cublas").unwrap();
    include_paths.append(&mut cublas.include_paths);
    libs.append(&mut cublas.libs);

    let mut curand: pkg_config::Library = pkg_config::probe_library("curand").unwrap();
    include_paths.append(&mut curand.include_paths);
    libs.append(&mut curand.libs);

    let mut builder = autocxx_build::Builder::new("src/lib.rs", include_paths).build()?;
    builder.cpp(true).std("c++17").compile("dlib-sys");

    libs.iter()
        .for_each(|x: &String| println!("cargo:rustc-link-lib={}", x));
    println!("cargo:rustc-link-lib=cudnn");

    println!("cargo:rerun-if-changed=src/lib.rs");

    Ok(())
}
