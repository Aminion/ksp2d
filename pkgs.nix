# To update nix-prefetch-git https://github.com/NixOS/nixpkgs
import ((import <nixpkgs> {}).fetchFromGitHub {
  owner = "NixOS";
  repo = "nixpkgs";
  rev = "b9af749fffd47cf9b6c78ff8d774a640b27c1394";
  sha256  = "GbgxiKdJVitsSs3JaSRrSS+nKfzLOfI6gS4SUSSuoL0=";
})