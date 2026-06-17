{ stdenv
, rustPlatform
, writableTmpDirAsHomeHook
, binaryen
, cargo
, rustc
, wasm-bindgen-cli
, wasm-pack
}:

{ version }:

let
  cargoToml = fromTOML (builtins.readFile ./Cargo.toml);
  manifestJson = builtins.fromJSON (builtins.readFile ./extension/manifest.json);

  addonId = manifestJson.browser_specific_settings.gecko.id;

  firefoxExtensionPath = "extensions/{ec8030f7-c20a-464f-9b0e-13a3a9e97384}";
in

stdenv.mkDerivation {
  pname = cargoToml.package.name;
  inherit version;
  inherit addonId;

  src = ./.;

  cargoDeps = rustPlatform.importCargoLock { lockFile = ./Cargo.lock; };

  nativeBuildInputs = [
    rustPlatform.cargoSetupHook
    rustPlatform.cargoCheckHook
    writableTmpDirAsHomeHook
    binaryen
    cargo
    rustc
    rustc.llvmPackages.lld
    wasm-bindgen-cli
    wasm-pack
  ];

  doCheck = true;
  # For some reason cargoCheckHook doesn't use .cargo/config.toml,
  # so we have to specify web-sys unstable apis flag again here
  RUSTFLAGS = "--cfg=web_sys_unstable_apis";
  cargoCheckType = "release";

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
