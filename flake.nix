{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    fup.url = "github:gytis-ivaskevicius/flake-utils-plus/master";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  description = "Default flake";

  outputs = {self, ...} @ inputs:
    inputs.fup.lib.mkFlake {
      inherit self inputs;

      channelsConfig = {
        allowUnfree = true;
      };

      sharedOverlays = [
        inputs.rust-overlay.overlays.default
      ];

      outputsBuilder = channels: {
        devShells = {
          default = channels.nixpkgs.pkgs.mkShell {
            nativeBuildInputs = with channels.nixpkgs.pkgs; [
              rust-bin.stable.latest.default
              cargo-flamegraph

              rustPlatform.bindgenHook

              cmake
              pkg-config
            ];

            buildInputs = with channels.nixpkgs.pkgs; [
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
            ];
          };
        };
      };
    };
}
