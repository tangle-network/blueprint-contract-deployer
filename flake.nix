{
  description = "Eigenlayer contract deployment development environment";
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    # Rust
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs = {
        nixpkgs.follows = "nixpkgs";
      };
    };
    foundry = {
      url = "github:shazow/foundry.nix/main";
      inputs = {
        nixpkgs.follows = "nixpkgs";
        flake-utils.follows = "flake-utils";
      };
    };
  };

  outputs =
    {
      self,
      nixpkgs,
      rust-overlay,
      foundry,
      flake-utils,
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        overlays = [
          (import rust-overlay)
          foundry.overlay
        ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        lib = pkgs.lib;
        toolchain = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
      in
      {
        devShells.default = pkgs.mkShell {
          name = "eigenlayer-contract-deployer";
          nativeBuildInputs = [
            pkgs.pkg-config
            pkgs.clang
            pkgs.libclang.lib
            pkgs.openssl.dev
            pkgs.gmp
            # Protocol Buffers
            pkgs.protobuf
            # Mold Linker for faster builds (only on Linux)
            (lib.optionals pkgs.stdenv.isLinux pkgs.mold)
            (lib.optionals pkgs.stdenv.isDarwin pkgs.darwin.apple_sdk.frameworks.Security)
            (lib.optionals pkgs.stdenv.isDarwin pkgs.darwin.apple_sdk.frameworks.SystemConfiguration)
          ];
          buildInputs = [
            # We want the unwrapped version, wrapped comes with nixpkgs' toolchain
            pkgs.rust-analyzer-unwrapped
            # Finally the toolchain
            toolchain
            pkgs.foundry-bin
            # Nodejs
            pkgs.nodePackages.typescript-language-server
            pkgs.nodejs_22
            pkgs.nodePackages.yarn
            pkgs.taplo
          ];
          packages = [
            pkgs.cargo-nextest
            pkgs.cargo-expand
            pkgs.cargo-dist
          ];
          # Environment variables
          RUST_SRC_PATH = "${toolchain}/lib/rustlib/src/rust/library";
          LD_LIBRARY_PATH = lib.makeLibraryPath [
            pkgs.gmp
            pkgs.libclang
            pkgs.openssl.dev
            pkgs.stdenv.cc.cc
          ];
        };
      }
    );
}
