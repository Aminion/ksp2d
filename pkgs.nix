# To update nix-prefetch-git https://github.com/NixOS/nixpkgs
import ((import <nixpkgs> {}).fetchFromGitHub {
  owner = "NixOS";
  repo = "nixpkgs";
  rev = "8575f6e2331b2b7b906fa2528bda25c880302e0b";
  sha256  = "6WD+YJs2d2y1VWRbH8UvOfLOmHeHfoqjkRupwNtLzoU=";
})