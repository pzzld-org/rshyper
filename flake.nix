{
  description = "A developmental environment for a Rust project using Nix flakes with direnv";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };
        rustToolchain = pkgs.rust-bin.stable.latest.default;
      in rec {
        packages.default = pkgs.rustPlatform.buildRustPackage {
          pname = "rshyper";
          version = "0.2.0";
          src = self; # ./.;
          cargoLock = {
            lockFile = ./Cargo.lock;
          };
        };

        devShells.default = pkgs.mkShell {
          buildInputs = [
            rustToolchain
            pkgs.cargo
            pkgs.rust-analyzer
            pkgs.pkg-config
            pkgs.openssl
          ];
          shellHook = ''
            export CARGO_HOME=$PWD/.cargo
          '';
        };
      }
    );
}