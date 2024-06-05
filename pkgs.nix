# To update nix-prefetch-git https://github.com/NixOS/nixpkgs
import ((import <nixpkgs> {}).fetchFromGitHub {
  owner = "NixOS";
  repo = "nixpkgs";
  rev = "85acf0a201f1917b254c8a0807d251a7937b0019";
  sha256  = "sxKU+NhN7FySsfycCFALDyTNzX76g0YNriubrq/5OAQ=";
})