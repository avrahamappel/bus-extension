{
  description = ''A Firefox extension to enhance the "Where's my bus" page of https://tstg.mybusplanner.ca'';

  inputs = {
    nixpkgs.url = "nixpkgs/nixpkgs-unstable";
  };

  outputs = inputs@{ self, flake-parts, ... }:

    flake-parts.lib.mkFlake { inherit inputs; } ({ ... }: {

      systems = [
        "x86_64-linux"
        "aarch64-darwin"
      ];

      perSystem = { config, pkgs, ... }: {
        packages = rec {
          bus-extension = (pkgs.callPackage ./. {
            inherit wasm-bindgen-cli;
          }) {
            version = builtins.concatStringsSep "-" [
              (builtins.substring 0 4 self.lastModifiedDate)
              (builtins.substring 4 2 self.lastModifiedDate)
              (builtins.substring 6 2 self.lastModifiedDate)
              (self.shortRev or self.dirtyShortRev)
            ];
          };

          wasm-bindgen-cli = pkgs.callPackage ./wasm-bindgen-cli.nix { };

          default = bus-extension;
        };

        devShells.default = pkgs.mkShell {
          packages = config.packages.default.nativeBuildInputs ++ (with pkgs; [
            bacon
            clippy
            rust-analyzer
            rustfmt
            zip
          ]);
        };

        apps.update-wasm-bindgen = {
          type = "app";
          program = pkgs.writeShellScriptBin "update-wasm-bindgen" ''
            ${pkgs.lib.getExe pkgs.nix-update} \
              --flake \
              --version=skip \
              --override-filename wasm-bindgen-cli.nix \
              wasm-bindgen-cli
          '';
        };
      };
    });
}
