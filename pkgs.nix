# To update nix-prefetch-git https://github.com/NixOS/nixpkgs
import ((import <nixpkgs> {}).fetchFromGitHub {
  owner = "NixOS";
  repo = "nixpkgs";
  rev = "b68f30f0b4bd01586d5c90e8a2f8b87f4ee86458";
  sha256  = "XGM9LyEVbcZ9m37ckNNu+5kmR/Lj0Npt1o4phhac7oo=";
})