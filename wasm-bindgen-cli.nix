{ pkgs
, lib
, rustPlatform
, fetchCrate
, buildWasmBindgenCli
}:

let
  wasmBindgenCliSourceHashes = {
    "0.2.114" = "sha256-xrCym+rFY6EUQFWyWl6OPA+LtftpUAE5pIaElAIVqW0=";
  };
  wasmBindgenCliVendorHashes = {
    "0.2.114" = "sha256-Z8+dUXPQq7S+Q7DWNr2Y9d8GMuEdSnq00quUR0wDNPM=";
    "0.2.120" = "sha256-Dkkx8Bhfk+y/jEz9Fzwytmv2N3Gj/7ST+5MlPRzzetU=";
  };

  wasmBindgenVersion =
    (lib.findFirst (p: p.name == "wasm-bindgen")
      (throw "wasm-bindgen package not found in Cargo.lock")
      (lib.fromTOML (lib.readFile ./Cargo.lock)).package).version;

  wasmBindgenCliBuilt = buildWasmBindgenCli rec {
    src = fetchCrate {
      pname = "wasm-bindgen-cli";
      version = wasmBindgenVersion;
      hash = wasmBindgenCliSourceHashes.${wasmBindgenVersion} or lib.fakeHash;
    };
    cargoDeps = rustPlatform.fetchCargoVendor {
      inherit src;
      inherit (src) pname version;
      hash = wasmBindgenCliVendorHashes.${wasmBindgenVersion} or lib.fakeHash;
    };
  };

  wasm-bindgen-cli =
    pkgs.${"wasm-bindgen-cli_${lib.replaceString "." "_" wasmBindgenVersion}"}
      or wasmBindgenCliBuilt;
in

wasm-bindgen-cli
