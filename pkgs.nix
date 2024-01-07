# To update nix-prefetch-git https://github.com/NixOS/nixpkgs
import ((import <nixpkgs> {}).fetchFromGitHub {
  owner = "NixOS";
  repo = "nixpkgs";
  rev = "3ab933ca57b1d06cf1991320c922ee2a8405b77a";
  sha256  = "pf+dYMVo8E1O7ZNcOInEiD1vfoNrGlLrG/Z3OtwYTQ8=";
})