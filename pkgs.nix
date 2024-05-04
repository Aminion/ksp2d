# To update nix-prefetch-git https://github.com/NixOS/nixpkgs
import ((import <nixpkgs> {}).fetchFromGitHub {
  owner = "NixOS";
  repo = "nixpkgs";
  rev = "ff1b721763bb047a31cc18f57210cc552918e6d7";
  sha256  = "08+zG84LnWLSUnsd1a7Ufw0oXRF2U0WDXaBLnhgjW+s=";
})