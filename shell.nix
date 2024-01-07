
with import ./pkgs.nix {};

stdenv.mkDerivation rec {
  name = "ksp2d-rust-env";
  env = buildEnv { name = name; paths = buildInputs; };

  buildInputs = [
    rustup
    SDL2
    SDL2_mixer
    valgrind
    llvm_11
  ];
}
