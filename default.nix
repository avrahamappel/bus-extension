{ stdenv
, rustPlatform
, cargo
, rustc
, wasm-bindgen-cli
, wasm-pack
}:

let
  cargoToml = builtins.fromTOML (builtins.readFile ./Cargo.toml);
  manifestJson = builtins.fromJSON (builtins.readFile ./extension/manifest.json);

  addonId = manifestJson.browser_specific_settings.gecko.id;

  firefoxExtensionPath = "extensions/{ec8030f7-c20a-464f-9b0e-13a3a9e97384}";
in

stdenv.mkDerivation {
  pname = cargoToml.package.name;
  inherit (cargoToml.package) version;
  inherit addonId;

  src = ./.;

  cargoDeps = rustPlatform.importCargoLock { lockFile = ./Cargo.lock; };

  nativeBuildInputs = [
    rustPlatform.cargoSetupHook
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
