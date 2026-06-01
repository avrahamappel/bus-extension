{ lib
, rustPlatform
, fetchCrate
, buildWasmBindgenCli
}:

let
  hash = "sha256-vO4RSxi/sMWxmsEs3GuljdMfIRSu75A+Q+c5wgYToRU=";
  vendorHash = "sha256-Inup6vvJSG5ghNyeDPyZbfZo4d0LsMG2OJfStoaeDBs=";

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
