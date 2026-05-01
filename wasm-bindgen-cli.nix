{ lib
, rustPlatform
, fetchCrate
, buildWasmBindgenCli
}:

let
  hash = "sha256-xrCym+rFY6EUQFWyWl6OPA+LtftpUAE5pIaElAIVqW0=";
  vendorHash = "sha256-Z8+dUXPQq7S+Q7DWNr2Y9d8GMuEdSnq00quUR0wDNPM=";

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
