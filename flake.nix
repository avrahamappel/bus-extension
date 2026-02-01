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
        packages.bus-extension = (pkgs.callPackage ./. { }) {
          version = if self ? shortRev then self.shortRev else "dirty";
        };
        packages.default = config.packages.bus-extension;

        devShells.default = pkgs.mkShell {
          packages = config.packages.default.nativeBuildInputs ++ (with pkgs; [
            bacon
            clippy
            rust-analyzer
            rustfmt
            zip
          ]);
        };
      };
    });
}
