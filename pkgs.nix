# To update nix-prefetch-git https://github.com/NixOS/nixpkgs
import ((import <nixpkgs> {}).fetchFromGitHub {
  owner = "NixOS";
  repo = "nixpkgs";
  rev = "3ab807f275232d227e846b5947775dc99e24e63c";
  sha256  = "h92wkQ0FRALA0tcA9ezhFolvLxi3BKXJQwa1iuaLSrc=";
})