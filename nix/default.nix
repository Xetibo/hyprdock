{
  cargo,
  gtk-layer-shell,
  gtk3,
  lib,
  pkg-config,
  rustc,
  craneLib,
  ...
}: let
  cargoToml = builtins.fromTOML (builtins.readFile ../Cargo.toml);
in
  craneLib.buildPackage {
    pname = cargoToml.package.name;
    inherit (cargoToml.package) version;

    src = craneLib.cleanCargoSource ../.;

    buildInputs = [
      pkg-config
      gtk3
      gtk-layer-shell
    ];

    nativeBuildInputs = [
      pkg-config
      cargo
      rustc
    ];

    meta = with lib; {
      description = "Docking program for Hyprland";
      homepage = "https://github.com/Xetibo/hyprdock";
      changelog = "https://github.com/Xetibo/hyprdock/releases/tag/${version}";
      license = licenses.gpl3;
      maintainers = with maintainers; [dashietm];
      mainProgram = "hyprdock";
    };
  }
