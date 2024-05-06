{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  buildInputs = [
    pkgs.rustup pkgs.sass pkgs.tailwindcss
  ];

  shellHook = ''
    # Setup rustup with the nightly toolchain and default toolchain as nightly
    rustup default nightly
    rustup update nightly

    # Optionally, add wasm target if your project needs it
    rustup target add wasm32-unknown-unknown --toolchain nightly

    cargo install cargo-leptos
  '';
}
