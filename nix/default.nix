inputs:
inputs.flake-parts.lib.mkFlake {inherit inputs;} {
  systems = ["x86_64-linux"];
  perSystem = {
    pkgs,
    system,
    lib,
    self',
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
    packages = {
      default = pkgs.callPackage ./package.nix {};
    };

    devShells = {
      default =
        (self'.packages.default.overrideAttrs (old: {
          nativeBuildInputs = with pkgs;
            old.nativeBuildInputs
            ++ [
              cmake

              cargo-flamegraph
            ];

          shellHook = ''
            export LINUX_HELLO__CONFIG_FILEPATH="./config.toml"
            export LINUX_HELLO__DATA_FILEPATH="$HOME/.local/share/linux-hello/data.json"
            export LINUX_HELLO__MODELS__FACE_RECOGNITION__FILEPATH="$HOME/.local/state/linux-hello/models/dlib_face_recognition_resnet_model_v1.dat"
            export LINUX_HELLO__MODELS__SHAPE_PREDICTOR__FILEPATH="$HOME/.local/state/linux-hello/models/shape_predictor_68_face_landmarks_GTX.dat"
          '';
        }))
        .override {
          rustPlatform = pkgs.makeRustPlatform {
            cargo = pkgs.rust-bin.stable.latest.default;
            rustc = pkgs.rust-bin.stable.latest.default;
          };
        };
    };
  };
}
