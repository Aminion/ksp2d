# To update nix-prefetch-git https://github.com/NixOS/nixpkgs
import ((import <nixpkgs> {}).fetchFromGitHub {
  owner = "NixOS";
  repo = "nixpkgs";
  rev = "d275de7163c0717b2072c0a67ea6919b0e6a9e0d";
  sha256  = "wPDUS2zT7AZVqM5GNe/V6dqznL65UaHsAYUhrExtD9s=";
})