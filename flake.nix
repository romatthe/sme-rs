{
  inputs = {
    nixpkgs.url = "nixpkgs/nixos-unstable";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    rust-manifest = {
      url = "https://static.rust-lang.org/dist/channel-rust-1.80.0.toml";
      flake = false;
    };
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, fenix, flake-utils, nixpkgs, rust-manifest }:
    flake-utils.lib.eachDefaultSystem (system: 
      let
        toolchain = (fenix.packages.${system}.fromManifestFile rust-manifest).toolchain;
        pkgs = nixpkgs.legacyPackages.${system};
      in {
        packages.default = (pkgs.makeRustPlatform {
          cargo = toolchain;
          rustc = toolchain;
        }).buildRustPackage {
          pname = "sme-rs";
          version = "0.1.0";

          src = ./.;

          cargoLock.lockFile = ./Cargo.lock;
        };

        devShell = with pkgs; mkShell rec {
          buildInputs = [
            rust-analyzer
            toolchain
          ];

          RUST_BACKTRACE = "1";

          shellHook = ''
  
          '';
        };
      });
}