with import ./pkgs.nix {};

stdenv.mkDerivation rec {
  name = "ksp2d-rust-env";
  env = buildEnv { name = name; paths = buildInputs; };

  buildInputs = [
    llvm_18
    SDL2
    SDL2_mixer
    SDL2_gfx
    SDL2_ttf
    open-sans
  ];
   shellHook = ''
    ln -sf ${open-sans}/share/fonts/truetype ./
  '';
}
