
with import ./pkgs.nix {};

stdenv.mkDerivation rec {
  name = "ksp2d-rust-env";
  env = buildEnv { name = name; paths = buildInputs; };

  buildInputs = [
    rustup
    SDL2
    SDL2_mixer
    SDL2_gfx
    valgrind
    llvm_11
  ];
}
