{ pkgs
, lib
, rustPlatform
, fetchCrate
, buildWasmBindgenCli
}:

let
  wasmBindgenCliSourceHashes = {
    "0.2.114" = "sha256-xrCym+rFY6EUQFWyWl6OPA+LtftpUAE5pIaElAIVqW0=";
    "0.2.117" = "sha256-vtDQXL8FSgdutqXG7/rBUWgrYCtzdmeVQQkWkjasvZU=";
  };
  wasmBindgenCliVendorHashes = {
    "0.2.114" = "sha256-Z8+dUXPQq7S+Q7DWNr2Y9d8GMuEdSnq00quUR0wDNPM=";
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
