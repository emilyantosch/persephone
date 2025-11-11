{
  pkgs ? import <nixpkgs-unstable> { }
}:
pkgs.mkShell rec {
  nativeBuildInputs = with pkgs; [
    rustc
    rustfmt
    clippy
    cargo
    wayland-utils
    openssl
    glfw-wayland
    xdg-utils
    sqlite
    sqlite.dev
    openssl.dev
    pkg-config
    waylandpp.dev
    wlvncc
    libxkbcommon.dev
    xkbd
    xwayland
  ];
  buildInputs = with pkgs; [
    at-spi2-atk
    libxkbcommon.dev
    alsa-lib
    libudev-zero
    atkmm
    cairo
    gdk-pixbuf
    glib
    gtk3
    harfbuzz
    librsvg
    vulkan-loader
    vulkan-tools
    libsoup_3
    pango
    webkitgtk_4_1
    openssl
  ];
  LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath [
    pkgs.vulkan-loader
    pkgs.libxkbcommon
  ];
}
