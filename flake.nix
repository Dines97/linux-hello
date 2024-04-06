{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-parts.url = "github:hercules-ci/flake-parts";
    # rust-overlay.url = "github:oxalica/rust-overlay";
  };

  description = "Default flake";

  outputs = inputs @ {...}: import ./nix inputs;
}
