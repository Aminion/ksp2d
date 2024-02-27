# To update nix-prefetch-git https://github.com/NixOS/nixpkgs
import ((import <nixpkgs> {}).fetchFromGitHub {
  owner = "NixOS";
  repo = "nixpkgs";
  rev = "51ea18aaa6b1951be64fcfad57088e2b591ca749";
  sha256  = "C9WikltVlB1yAyFEb/HpADTyDuLz5wctZMg+P+UASn4=";
})