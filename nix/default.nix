{
  rustPlatform,
  pkg-config,
  libGL,
  libxkbcommon,
  wayland,
  libclang,
  cargo,
  cargo-watch,
  rustc,
  rust-analyzer,
  clippy,
  lib,
  lockFile,
  ...
}: let
  cargoToml = builtins.fromTOML (builtins.readFile ../Cargo.toml);
  libPath = lib.makeLibraryPath [
    libGL
    libxkbcommon
    wayland
    pkg-config
    libclang
  ];
in
  rustPlatform.buildRustPackage rec {
    pname = cargoToml.package.name;
    inherit (cargoToml.package) version;

    src = ../.;

    nativeBuildInputs = [
      pkg-config
      wayland
      cargo
      cargo-watch
      rustc
      rust-analyzer
      clippy
      libGL
      libxkbcommon
      libclang
    ];

    cargoLock = {
      inherit lockFile;
      outputHashes = {
        "oxiced-0.5.1" = "sha256-ihgUXOiyirZ1CeHi/CNSHfMmi9OOBnIE0D5qUIaCwXE=";
      };
    };

    copyLibs = true;
    LD_LIBRARY_PATH = libPath;
    LIBCLANG_PATH = "${libclang.lib}/lib";
    doCheck = false;

    postFixup = ''
      patchelf --set-rpath "${libPath}" "$out/bin/hyprdock"
    '';

    meta = with lib; {
      description = "Docking program for Hyprland";
      homepage = "https://github.com/Xetibo/hyprdock";
      changelog = "https://github.com/Xetibo/hyprdock/releases/tag/${version}";
      license = licenses.gpl3;
      maintainers = with maintainers; [dashietm];
      mainProgram = "hyprdock";
    };
  }
