let pkgs = import <nixpkgs> { };
in pkgs.mkShell rec {
  nativeBuildInputs = with pkgs; [ pkg-config ];
  buildInputs = with pkgs; [
    # rust support
    cargo
    rustc
    clang
    glib

    # rust tools
    rustfmt
    rust-analyzer
    clippy

    # libraries
    sqlite

    # other tools
    sqlite-web
  ];

  LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath (buildInputs ++ nativeBuildInputs);
}
