{ lib
, rustPlatform
, fetchCrate
, buildWasmBindgenCli
}:

let
  hash = "sha256-di+qBAdd7pENLiIB9CoZoab+W5xeDoByMREcCGTSzWo=";
  vendorHash = "sha256-FTv2GZIAQs0ePdIZXIXil7JbZ6kIT05VG6vqC1qNFxQ=";

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
