# To update nix-prefetch-git https://github.com/NixOS/nixpkgs
import ((import <nixpkgs> {}).fetchFromGitHub {
  owner = "NixOS";
  repo = "nixpkgs";
  rev = "f8ed07ecd1938b54e76b17d938cd6212bd98d7a0";
  sha256  = "P2xRrfoayWUCAM/3Rq+YaYEabd2ar4lc2H55tYZnL8E=";
})