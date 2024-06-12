{
  rustPlatform,
  pkg-config,
  cudaPackages,
  cxx-rs,
  opencv,
  dlib,
  blas,
  lapack,
  xorg,
  openssl,
  jemalloc,
}:
rustPlatform.buildRustPackage {
  pname = "linux-hello";
  version = "0.1.0";

  src = ../.;

  cargoLock.lockFile = ../Cargo.lock;

  nativeBuildInputs = [
    rustPlatform.bindgenHook

    pkg-config

    jemalloc
  ];

  buildInputs = [
    cxx-rs

    jemalloc

    (opencv.override {
      enableGtk3 = true;
      enableCuda = true;
      # enableCudnn = true;
    })
    (dlib.override {
      guiSupport = true;
      cudaSupport = true;

      # sse4Support = true;
      # avxSupport = true;
    })

    # opencv
    # dlib

    blas
    lapack
    xorg.libX11.dev
    openssl

    cudaPackages.cuda_cudart.dev
    cudaPackages.libcublas.dev
    cudaPackages.libcurand.dev
    cudaPackages.libcusolver.dev

    cudaPackages.cudnn.dev
    cudaPackages.cudnn.lib
  ];
  meta = {
  };
}
