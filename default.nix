{ pkgs
, lib
, stdenv
, rustPlatform
, fetchCrate
, buildWasmBindgenCli
, binaryen
, cargo
, rustc
, wasm-pack
}:

{ version }:

let
  cargoToml = builtins.fromTOML (builtins.readFile ./Cargo.toml);
  manifestJson = builtins.fromJSON (builtins.readFile ./extension/manifest.json);

  addonId = manifestJson.browser_specific_settings.gecko.id;

  firefoxExtensionPath = "extensions/{ec8030f7-c20a-464f-9b0e-13a3a9e97384}";

  wasmBindgenVersion =
    (lib.findFirst (p: p.name == "wasm-bindgen")
      (throw "wasm-bindgen package not found in Cargo.lock")
      (lib.fromTOML (lib.readFile ./Cargo.lock)).package).version;

  wasmBindgenCliSourceHashes = {
    "0.2.114" = "sha256-xrCym+rFY6EUQFWyWl6OPA+LtftpUAE5pIaElAIVqW0=";
    "0.2.117" = "sha256-vtDQXL8FSgdutqXG7/rBUWgrYCtzdmeVQQkWkjasvZU=";
  };
  wasmBindgenCliVendorHashes = {
    "0.2.114" = "sha256-Z8+dUXPQq7S+Q7DWNr2Y9d8GMuEdSnq00quUR0wDNPM=";
  };

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

stdenv.mkDerivation {
  pname = cargoToml.package.name;
  inherit version;
  inherit addonId;

  src = ./.;

  cargoDeps = rustPlatform.importCargoLock { lockFile = ./Cargo.lock; };

  nativeBuildInputs = [
    rustPlatform.cargoSetupHook
    binaryen
    cargo
    rustc
    rustc.llvmPackages.lld
    wasm-bindgen-cli
    wasm-pack
  ];

  configurePhase = ''
    export HOME=$(mktemp -d)
  '';

  buildPhase = ''
    wasm-pack build --target no-modules --no-typescript --no-pack --mode no-install
  '';

  installPhase = ''
    mkdir -p $out/share/mozilla/${firefoxExtensionPath}/${addonId}
    cp pkg/* $out/share/mozilla/${firefoxExtensionPath}/${addonId}/
    cp $src/extension/* $out/share/mozilla/${firefoxExtensionPath}/${addonId}/
  '';

  meta.mozPermissions = manifestJson.permissions;
}
