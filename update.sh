#!/usr/bin/env bash

# Update flake
nix flake update

# Update cargo
cargo update

# Update wasm-bindgen-cli hashes
nix run .#update-wasm-bindgen

# Check that nix build succeeds
nix build --no-link
