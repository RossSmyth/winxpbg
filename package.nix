{
  lib,
  rustPlatform,
  pkg-config,
  wrapGAppsHook4,
  glib,
  gtk4,
  webkitgtk_6_0,
  gtk4-layer-shell,
}:
let
  fs = lib.fileset;

  files = fs.unions [
    ./view
  ];
in
rustPlatform.buildRustPackage {
  pname = "winxpbg";
  inherit ((builtins.fromTOML (builtins.readFile ./view/Cargo.toml)).package) version;

  src = fs.toSource {
    root = ./view;
    fileset = files;
  };

  cargoLock.lockFile = ./view/Cargo.lock;

  nativeBuildInputs = [
    pkg-config
    wrapGAppsHook4
  ];

  buildInputs = [
    glib
    gtk4
    webkitgtk_6_0
    gtk4-layer-shell
  ];

  meta = {
    description = "Windows XP as your desktop background";
    mainProgram = "minxpbg";
  };
}
