{ pkgs ? import <nixpkgs> { } }:

pkgs.mkShell {
  packages = with pkgs; [
    bacon
    cargo
    clippy
    lld
    rust-analyzer
    rustc
    rustfmt
    wasm-pack
    zip
  ];
}
