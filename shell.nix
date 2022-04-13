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
  ];
  buildInputs = [
    libmysqlclient
diesel-cli
  cargo-make watchexec clippy rustfmt rust-analyzer openssl lld pkgconfig
  pscale
  (latest.rustChannels.nightly.rust.override {
    targets = ["wasm32-unknown-unknown" "x86_64-unknown-linux-gnu"];
  })
  latest.rustChannels.nightly.cargo
  latest.rustChannels.nightly.rust-src
  latest.rustChannels.nightly.rustc
  ];

  shellHook = ''
    export PATH=$PATH:~/.cargo/bin
  '';
}
