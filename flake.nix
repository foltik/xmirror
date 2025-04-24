{
  description = "Mirror keyboard and mouse events to a remote server";

  inputs = {
    nixpkgs.url      = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url  = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { nixpkgs, flake-utils, rust-overlay, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ rust-overlay.overlays.default ];
        };

        rust-compiler = pkgs.rust-bin.stable.latest.default.override {
          extensions = [ "rust-src" ];
        };

        linuxDeps = pkgs.lib.optionals pkgs.stdenv.isLinux (with pkgs.xorg; [
          libX11
          libXtst
        ]);

        darwinDeps = pkgs.lib.optionals pkgs.stdenv.isDarwin (with pkgs.darwin.apple_sdk_11_0.frameworks; [
          CoreGraphics
          ApplicationServices
        ]);
      in
      {
        devShells.default = pkgs.mkShell rec {
          nativeBuildInputs = with pkgs; [ pkg-config ];
          buildInputs = linuxDeps ++ darwinDeps;

          packages = with pkgs; [
            rust-compiler
            rust-analyzer
          ];

          shellHook = ''
            export LD_LIBRARY_PATH="$LD_LIBRARY_PATH:${pkgs.lib.makeLibraryPath buildInputs}"
          '';
        };
      });
}
