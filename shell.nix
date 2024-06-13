{
  pkgs ? import <nixpkgs> { },
  lib,
}:
let
  packages = with pkgs; [
    rust-analyzer
    rustfmt
    clippy
    mold
    cmake

    pkg-config
    xorg.libX11
    libGL
    alsa-lib
    xorg.libXi

    #xorg.libXcursor
    #xorg.libXrandr
    #xorg.libXinerama
  ];
in
pkgs.mkShell {
  # Get dependencies from the main package
  inputsFrom = [ (pkgs.callPackage ./default.nix { }) ];
  nativeBuildInputs = packages;
  buildInputs = packages;
  env = {
    LIBCLANG_PATH = "${pkgs.libclang.lib}/lib";
    LD_LIBRARY_PATH = "${lib.makeLibraryPath packages}";
  };
}
