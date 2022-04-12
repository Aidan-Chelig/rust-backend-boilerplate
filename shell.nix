# RUST

let
  moz_overlay = import (builtins.fetchTarball https://github.com/mozilla/nixpkgs-mozilla/archive/master.tar.gz);
  nixpkgs = import <nixpkgs> { overlays = [ moz_overlay ]; };
in

with nixpkgs;

mkShell {
  name = "rust";
    nativeBuildInputs = [
    pkgconfig
    clang lld # To use lld linker
  ];
  buildInputs = [
  cargo-make watchexec clippy rustfmt rust-analyzer openssl lld pkgconfig
  (latest.rustChannels.stable.rust.override {
    targets = ["wasm32-unknown-unknown" "x86_64-unknown-linux-gnu"];
  })
  latest.rustChannels.stable.cargo
  latest.rustChannels.stable.rust-src
  latest.rustChannels.stable.rustc
  ];

  shellHook = ''
    export PATH=$PATH:~/.cargo/bin
  '';
}
