{ pkgs ? import <nixpkgs> { } }:

pkgs.mkShell {
  packages = with pkgs; [
    bacon
    cargo
    clippy
    rust-analyzer
    rustc
    rustfmt
  ];
}
