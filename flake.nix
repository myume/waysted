{
  description = "A screen time tracker for wayland";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    utils.url = "github:numtide/flake-utils";
  };

  outputs = {
    self,
    nixpkgs,
    utils,
  }:
    utils.lib.eachDefaultSystem (
      system: let
        pkgs = import nixpkgs {inherit system;};
        inherit (pkgs) lib rustPlatform;
      in {
        packages.default = rustPlatform.buildRustPackage {
          pname = "waysted";
          version = "0.1.0";

          src = ./.;

          cargoLock.lockFile = ./Cargo.lock;

          meta = {
            description = "A Lightweight screentime tracker for wayland";
            license = lib.licenses.mit;
            platforms = lib.platforms.linux;
            mainProgram = "waysted";
          };
        };

        devShell = with pkgs;
          mkShell {
            buildInputs = [cargo rustc rustfmt pre-commit rustPackages.clippy];
            RUST_SRC_PATH = rustPlatform.rustLibSrc;
          };
      }
    );
}
