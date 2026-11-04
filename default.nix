{ pkgs, nixpkgs, system, makeRustPlatform, rust-overlay }:
let
  rustPkgs = import nixpkgs {
    inherit system;
    overlays = [ (import rust-overlay) ];
  };

  rustVersion = "1.85.0";
  wasmUnknownUknown = "wasm32-unknown-unknown";
  wasm32Wasi = "wasm32-wasi";

  rustDefaultTarget = rustPkgs.rust-bin.stable.${rustVersion}.default;

  rustWithWasmTarget = rustPkgs.rust-bin.nightly.${rustVersion}.default.override {
    targets = [ wasmUnknownUknown ];
  };

  rustPlatform = makeRustPlatform {
    cargo = rustDefaultTarget;
    rustc = rustDefaultTarget;
  };

  rustPlatformWasm = makeRustPlatform {
    cargo = rustWithWasmTarget;
    rustc = rustWithWasmTarget;
  };

  common = {
    version = "0.2.0";
    src = self;

    cargoLock = {
      lockFile = ./Cargo.lock;
    };

    nativeBuildInputs = [ pkgs.pkg-config ];
    PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig";
  };
in {
  workspace = pkgs.rustPlatformWasm.buildRustPackage (common // {
    cargoBuildFlags = "--release --workspace";
  });
}