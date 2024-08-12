# To update nix-prefetch-git https://github.com/NixOS/nixpkgs
import ((import <nixpkgs> {}).fetchFromGitHub {
  owner = "NixOS";
  repo = "nixpkgs";
  rev = "2e11a22606820ece544b1b7f14de9ae32392ef9a";
  sha256  = "sha256-GHu351+l1CTw/tiqn6OlRs9I5vEakZvcjXGLidmPWPE=";
})