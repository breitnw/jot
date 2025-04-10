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

    # web tools
    nodejs
    nodePackages.prettier
    vscode-langservers-extracted
    html-tidy
    stylelint
    jsbeautifier

    # libraries
    sqlite
    SDL2
    SDL2_ttf
    SDL2_image
    openssl.dev

    # other tools
    sqlite-web
  ];

  LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath (buildInputs ++ nativeBuildInputs);
}
