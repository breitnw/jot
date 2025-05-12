{
  description = "A devShell example";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };
      in {
        devShells.default = with pkgs;
          mkShell {
            buildInputs = [
              # rust support
              clang
              glib
              rust-bin.stable.latest.default
              rust-analyzer

              # web tools
              html-tidy
              stylelint
              jsbeautifier
              typescript-language-server # js/ts language server
              typescript # needed for above

              # libraries
              sqlite
              SDL2
              SDL2_ttf
              SDL2_image
              openssl.dev

              # other tools
              sqlite-web
            ];
          };
      });
}
