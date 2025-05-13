{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };
  outputs = { self, nixpkgs, flake-utils }: flake-utils.lib.eachDefaultSystem
    (system:
      let
        pkgs = import nixpkgs {
          inherit system;
        };
      in
      rec {
        packages.default = pkgs.rustPlatform.buildRustPackage {
          pname = "rust-platformer";
          version = "0.1.0";

          src = ./.;

          cargoLock = {
            lockFile = ./Cargo.lock;
          };

        #   buildInputs = with pkgs; [
        #     alsa-lib
        #     libudev-zero
        #     xorg.libX11
        #     xorg.libXcursor
        #     xorg.libXrandr
        #     xorg.libXi
        #   ];
          nativeBuildInputs = with pkgs; [
            pkg-config
            makeWrapper
          ];
        };

        devShells.default = pkgs.mkShell {
          # Pull in everything needed to build our crate.
          inputsFrom = [ packages.default ];

          # Other development niceties.
          RUST_SRC_PATH = "${pkgs.rustPlatform.rustLibSrc}";
          packages = with pkgs; [
            rust-analyzer
          ];
        };
      }
    );
}