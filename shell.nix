{ pkgs ? import <nixpkgs> { } }:

pkgs.mkShell {
  packages = with pkgs; [
    python3
    python3Packages.beautifulsoup4
    python3Packages.requests
  ];
}
