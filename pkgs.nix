# To update nix-prefetch-git https://github.com/NixOS/nixpkgs
import ((import <nixpkgs> {}).fetchFromGitHub {
  owner = "NixOS";
  repo = "nixpkgs";
  rev = "d4b40706f6b0c2d64b8e2401fb34784d9ea5aeb7";
  sha256  = "zG+TSTXSee6dgXwrz7ZvY7v9BEbDZk0mHxBBy2+o43c=";
})