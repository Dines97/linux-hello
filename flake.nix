{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-parts.url = "github:hercules-ci/flake-parts";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  description = "Default flake";

  outputs = inputs @ {...}:
    inputs.flake-parts.lib.mkFlake {inherit inputs;} {
      systems = ["x86_64-linux"];
      perSystem = {
        pkgs,
        system,
        ...
      }: {
        _module.args.pkgs = import inputs.nixpkgs {
          inherit system;
          overlays = [
            inputs.rust-overlay.overlays.default
          ];
          config = {
            allowUnfree = true;
          };
        };
        devShells = {
          default = pkgs.cudaPackages.backendStdenv.mkDerivation {
            name = "nix-shell";
            nativeBuildInputs = with pkgs; [
              rust-bin.stable.latest.default
              cargo-flamegraph

              rustPlatform.bindgenHook

              cmake
              pkg-config

              cudaPackages.cuda_nvcc
            ];
            buildInputs = with pkgs; [
              cxx-rs

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
              # cudatoolkit

              cudaPackages.cuda_cudart.dev
              cudaPackages.cuda_cudart.lib
              cudaPackages.cuda_cudart.static
              cudaPackages.cuda_nvcc.dev
              cudaPackages.libcublas.dev
              cudaPackages.libcublas.lib
              cudaPackages.libcublas.static
              cudaPackages.libcurand.dev
              cudaPackages.libcurand.lib
              cudaPackages.libcurand.static
              cudaPackages.libcusolver.dev
              cudaPackages.libcusolver.lib
              cudaPackages.libcusolver.static
              cudaPackages.cudnn.dev
              cudaPackages.cudnn.lib
              cudaPackages.cudnn.static
              cudaPackages.cuda_cccl.dev
            ];
          };
        };
      };
    };
}
