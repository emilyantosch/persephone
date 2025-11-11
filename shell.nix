{
  pkgs ? import <nixpkgs-unstable> { },
}:
pkgs.mkShell {
  nativeBuildInputs = with pkgs; [
    rustc
    rustfmt
    clippy
    cargo
    openssl
    xdg-utils
    sqlite
    sqlite.dev
    openssl.dev
    pkg-config
  ];
}
