# To update nix-prefetch-git https://github.com/NixOS/nixpkgs
import ((import <nixpkgs> {}).fetchFromGitHub {
  owner = "NixOS";
  repo = "nixpkgs";
  rev = "f67b36bad67f1cf683a61cf6ac98e12d3b5a6532";
  sha256  = "qLRT7SxojcBPc7Dzocd6Www4+I+UeFrkcmR1Gxgz/vM=";
})