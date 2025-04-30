let
  pkgs = import (fetchTarball("channel:nixpkgs-unstable")) {};

in pkgs.stdenv.mkDerivation rec {
  name = "ksp2d-rust-env";
  env = pkgs.buildEnv { name = name; paths = buildInputs; };

  buildInputs = with pkgs; [
    llvm_19
    rustup
    SDL2
    SDL2_mixer
    SDL2_gfx
    SDL2_ttf
    open-sans
  ];

   
   shellHook = ''
    ln -sf ${pkgs.open-sans}/share/fonts/truetype ./
  '';
}