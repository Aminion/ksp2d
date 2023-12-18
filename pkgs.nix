# To update nix-prefetch-git https://github.com/NixOS/nixpkgs
import ((import <nixpkgs> {}).fetchFromGitHub {
  owner = "NixOS";
  repo = "nixpkgs";
  rev = "6ae8bce49df89886d5eb0ef95199a0f120d78b62";
  sha256  = "Y9Br/PIeplmDRzEE2iO6rIe3THlle7EAz1rlyE+PeCQ=";
})