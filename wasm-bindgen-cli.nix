{ lib
, rustPlatform
, fetchCrate
, buildWasmBindgenCli
}:

let
  hash = "sha256-9Og1O/6/VC3sc5hBjoIWBErmLW2UHGMsTQO+6+m/dng=";
  vendorHash = "sha256-j81sUf1m5yXU4FJfGj7GV+hdqVCV8mRmNb0QIsMCjqw=";

  version =
    (lib.findFirst (p: p.name == "wasm-bindgen")
      (throw "wasm-bindgen package not found in Cargo.lock")
      (lib.fromTOML (lib.readFile ./Cargo.lock)).package).version;

  wasm-bindgen-cli = buildWasmBindgenCli rec {
    src = fetchCrate {
      pname = "wasm-bindgen-cli";
      inherit version hash;
    };
    cargoDeps = rustPlatform.fetchCargoVendor {
      inherit src;
      inherit (src) pname version;
      hash = vendorHash;
    };
  };

in

wasm-bindgen-cli
