let
    pkgs = import <nixpkgs> {};

    libraries = with pkgs;[
      gtk3
      cairo
      gdk-pixbuf
      glib
      dbus
      openssl_3
      librsvg
      webkitgtk_4_1
      libsoup_3
    ];
in
pkgs.mkShell {
  buildInputs = with pkgs; [
    rustup
    sass
    tailwindcss
    cargo-edit
    cargo-leptos
    rust-analyzer
    taplo
    prettierd
    nixd
    pkg-config
    dbus
    openssl_3
    glib
    gtk3
    libsoup_3
    webkitgtk_4_1
    appimagekit
    librsvg
    cargo-tauri
    corepack_latest
    nodejs_latest
  ];

  shellHook = ''
    # Tauri stuff
    export LD_LIBRARY_PATH=${pkgs.lib.makeLibraryPath libraries}:$LD_LIBRARY_PATH
    export XDG_DATA_DIRS=${pkgs.gsettings-desktop-schemas}/share/gsettings-schemas/${pkgs.gsettings-desktop-schemas.name}:${pkgs.gtk3}/share/gsettings-schemas/${pkgs.gtk3.name}:$XDG_DATA_DIRS
    # Nvidia is the worst company
    export WEBKIT_DISABLE_DMABUF_RENDERER=1

    # Setup rustup with the nightly toolchain and default toolchain as nightly
    rustup default nightly
    rustup update nightly

    # Optionally, add wasm target if your project needs it
    rustup target add wasm32-unknown-unknown --toolchain nightly
  '';
}
