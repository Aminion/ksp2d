# To update nix-prefetch-git https://github.com/NixOS/nixpkgs
import ((import <nixpkgs> {}).fetchFromGitHub {
  owner = "NixOS";
  repo = "nixpkgs";
  rev = "1e814d335141532933814e5f93f0d45d0e9c9f5b";
  sha256  = "sha256-6vPxxQN/2K7myRPTvsuMpHgXGozv8lep4vg7qOxzs+Y=";
})