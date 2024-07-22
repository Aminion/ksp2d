# To update nix-prefetch-git https://github.com/NixOS/nixpkgs
import ((import <nixpkgs> {}).fetchFromGitHub {
  owner = "NixOS";
  repo = "nixpkgs";
  rev = "3a6785efdb7b7412208e356b4064278aff90c6e2";
  sha256  = "sha256-bZYIRki8ElGw3hbc6t7VG3GiCpVYYNUVBDiW1fPjbck=";
})