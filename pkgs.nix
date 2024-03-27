# To update nix-prefetch-git https://github.com/NixOS/nixpkgs
import ((import <nixpkgs> {}).fetchFromGitHub {
  owner = "NixOS";
  repo = "nixpkgs";
  rev = "2944f24f1297a372c19fc815a4ba98ce48ef1340";
  sha256  = "O0PbS0Q3/6GuYVeAdsr5cBJ+TiPpGNXltXwmXaXoqDk=";
})