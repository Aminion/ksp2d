# To update nix-prefetch-git https://github.com/NixOS/nixpkgs
import ((import <nixpkgs> {}).fetchFromGitHub {
  owner = "NixOS";
  repo = "nixpkgs";
  rev = "87f0efcadc6089cbfd82992dd78d42f3ec2750fb";
  sha256  = "sha256-zPMo4a+e2P4cVBd7MdUCUrRmY2LD86Vj3J4PrcoBr/Y=";
})